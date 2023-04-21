// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::cmp::max;
use std::collections::BTreeMap;
use std::sync::Arc;

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use haneul_core::authority::AuthorityState;
use haneul_json_rpc_types::HaneulCommittee;
use haneul_json_rpc_types::{DelegatedStake, Stake, StakeStatus};
use haneul_open_rpc::Module;
use haneul_types::base_types::{MoveObjectType, ObjectID, HaneulAddress};
use haneul_types::committee::EpochId;
use haneul_types::dynamic_field::get_dynamic_field_from_store;
use haneul_types::error::{HaneulError, UserInputError};
use haneul_types::governance::StakedHaneul;
use haneul_types::id::ID;
use haneul_types::object::ObjectRead;
use haneul_types::haneul_serde::BigInt;
use haneul_types::haneul_system_state::haneul_system_state_summary::HaneulSystemStateSummary;
use haneul_types::haneul_system_state::PoolTokenExchangeRate;
use haneul_types::haneul_system_state::HaneulSystemStateTrait;
use haneul_types::haneul_system_state::{
    get_validator_from_table, haneul_system_state_summary::get_validator_by_pool_id, HaneulSystemState,
};
use tracing::{info, instrument};

use crate::api::{GovernanceReadApiServer, JsonRpcMetrics};
use crate::error::Error;
use crate::HaneulRpcModule;

pub struct GovernanceReadApi {
    state: Arc<AuthorityState>,
    pub metrics: Arc<JsonRpcMetrics>,
}

impl GovernanceReadApi {
    pub fn new(state: Arc<AuthorityState>, metrics: Arc<JsonRpcMetrics>) -> Self {
        Self { state, metrics }
    }

    async fn get_staked_haneul(&self, owner: HaneulAddress) -> Result<Vec<StakedHaneul>, Error> {
        let result = self
            .state
            .get_move_objects(owner, MoveObjectType::staked_haneul())
            .await?;
        self.metrics
            .get_stake_haneul_result_size
            .report(result.len() as u64);
        self.metrics
            .get_stake_haneul_result_size_total
            .inc_by(result.len() as u64);
        Ok(result)
    }

    async fn get_stakes_by_ids(
        &self,
        staked_haneul_ids: Vec<ObjectID>,
    ) -> Result<Vec<DelegatedStake>, Error> {
        let stakes_read = staked_haneul_ids
            .iter()
            .map(|id| self.state.get_object_read(id))
            .collect::<Result<Vec<_>, _>>()?;
        if stakes_read.is_empty() {
            return Ok(vec![]);
        }

        let mut stakes: Vec<(StakedHaneul, bool)> = vec![];

        for stake in stakes_read.into_iter() {
            match stake {
                ObjectRead::Exists(_, o, _) => stakes.push((StakedHaneul::try_from(&o)?, true)),
                ObjectRead::Deleted(oref) => {
                    match self
                        .state
                        .database
                        .find_object_lt_or_eq_version(oref.0, oref.1.one_before().unwrap())
                    {
                        Some(o) => stakes.push((StakedHaneul::try_from(&o)?, false)),
                        None => {
                            return Err(Error::UserInputError(UserInputError::ObjectNotFound {
                                object_id: oref.0,
                                version: None,
                            }))
                        }
                    }
                }
                ObjectRead::NotExists(id) => {
                    return Err(Error::UserInputError(UserInputError::ObjectNotFound {
                        object_id: id,
                        version: None,
                    }))
                }
            }
        }

        self.get_delegated_stakes(stakes).await
    }

    async fn get_stakes(&self, owner: HaneulAddress) -> Result<Vec<DelegatedStake>, Error> {
        let stakes = self.get_staked_haneul(owner).await?;
        if stakes.is_empty() {
            return Ok(vec![]);
        }

        self.get_delegated_stakes(stakes.iter().map(|s| (s.clone(), true)).collect())
            .await
    }

    async fn get_delegated_stakes(
        &self,
        stakes: Vec<(StakedHaneul, bool)>,
    ) -> Result<Vec<DelegatedStake>, Error> {
        let pools = stakes.into_iter().fold(
            BTreeMap::<_, Vec<_>>::new(),
            |mut pools, (stake, exists)| {
                pools
                    .entry(stake.pool_id())
                    .or_default()
                    .push((stake, exists));
                pools
            },
        );

        let system_state: HaneulSystemStateSummary =
            self.get_system_state()?.into_haneul_system_state_summary();
        let mut delegated_stakes = vec![];
        for (pool_id, stakes) in pools {
            // Rate table and rate can be null when the pool is not active
            let rate_table = self
                .get_exchange_rate_table(&system_state, &pool_id)
                .await
                .ok();
            let current_rate = if let Some(rate_table) = rate_table {
                self.get_exchange_rate(rate_table, system_state.epoch)
                    .await
                    .ok()
            } else {
                None
            };

            let mut delegations = vec![];
            for (stake, exists) in stakes {
                let status = if !exists {
                    StakeStatus::Unstaked
                } else if system_state.epoch >= stake.activation_epoch() {
                    let estimated_reward = if let (Some(rate_table), Some(current_rate)) =
                        (&rate_table, &current_rate)
                    {
                        let stake_rate = self
                            .get_exchange_rate(*rate_table, stake.activation_epoch())
                            .await
                            .unwrap_or_default();
                        let estimated_reward = ((stake_rate.rate() / current_rate.rate()) - 1.0)
                            * stake.principal() as f64;
                        max(0, estimated_reward.round() as u64)
                    } else {
                        0
                    };
                    StakeStatus::Active { estimated_reward }
                } else {
                    StakeStatus::Pending
                };
                delegations.push(Stake {
                    staked_haneul_id: stake.id(),
                    // TODO: this might change when we implement warm up period.
                    stake_request_epoch: stake.activation_epoch() - 1,
                    stake_active_epoch: stake.activation_epoch(),
                    principal: stake.principal(),
                    status,
                })
            }
            let validator =
                get_validator_by_pool_id(self.state.db().as_ref(), &system_state, pool_id)?;
            delegated_stakes.push(DelegatedStake {
                validator_address: validator.haneul_address,
                staking_pool: pool_id,
                stakes: delegations,
            })
        }
        Ok(delegated_stakes)
    }

    fn get_system_state(&self) -> Result<HaneulSystemState, Error> {
        Ok(self.state.database.get_haneul_system_state_object()?)
    }

    async fn get_exchange_rate_table(
        &self,
        system_state: &HaneulSystemStateSummary,
        pool_id: &ObjectID,
    ) -> Result<ObjectID, Error> {
        let active_rate = system_state.active_validators.iter().find_map(|v| {
            if &v.staking_pool_id == pool_id {
                Some(v.exchange_rates_id)
            } else {
                None
            }
        });

        if let Some(active_rate) = active_rate {
            Ok(active_rate)
        } else {
            // try find from inactive pool
            let validator = get_validator_from_table(
                self.state.db().as_ref(),
                system_state.inactive_pools_id,
                &ID::new(*pool_id),
            )?;

            Ok(validator.exchange_rates_id)
        }
    }

    async fn get_exchange_rate(
        &self,
        table: ObjectID,
        epoch: EpochId,
    ) -> Result<PoolTokenExchangeRate, Error> {
        let exchange_rate: PoolTokenExchangeRate = get_dynamic_field_from_store(
            self.state.db().as_ref(),
            table,
            &epoch,
        )
        .map_err(|err| {
            HaneulError::HaneulSystemStateReadError(format!("Failed to get exchange rate: {:?}", err))
        })?;
        Ok(exchange_rate)
    }
}

#[async_trait]
impl GovernanceReadApiServer for GovernanceReadApi {
    #[instrument(skip(self))]
    async fn get_stakes_by_ids(
        &self,
        staked_haneul_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        info!("get_stakes_by_ids");
        Ok(self.get_stakes_by_ids(staked_haneul_ids).await?)
    }

    #[instrument(skip(self))]
    async fn get_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>> {
        info!("get_stakes");
        Ok(self.get_stakes(owner).await?)
    }

    #[instrument(skip(self))]
    async fn get_committee_info(&self, epoch: Option<BigInt<u64>>) -> RpcResult<HaneulCommittee> {
        info!("get_committee_info");
        Ok(self
            .state
            .committee_store()
            .get_or_latest_committee(epoch.map(|e| *e))
            .map(|committee| committee.into())
            .map_err(Error::from)?)
    }

    #[instrument(skip(self))]
    async fn get_latest_haneul_system_state(&self) -> RpcResult<HaneulSystemStateSummary> {
        info!("get_latest_haneul_system_state");
        Ok(self
            .state
            .database
            .get_haneul_system_state_object()
            .map_err(Error::from)?
            .into_haneul_system_state_summary())
    }

    #[instrument(skip(self))]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        info!("get_reference_gas_price");
        let epoch_store = self.state.load_epoch_store_one_call_per_task();
        Ok(epoch_store.reference_gas_price().into())
    }
}

impl HaneulRpcModule for GovernanceReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        crate::api::GovernanceReadApiOpenRpc::module_doc()
    }
}
