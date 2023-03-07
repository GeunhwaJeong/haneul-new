// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use std::collections::HashMap;
use std::sync::Arc;
use haneul_json_rpc_types::{HaneulCommittee, HaneulSystemStateRpc};
use haneul_types::haneul_system_state::haneul_system_state_inner_v1::ValidatorMetadataV1;
use haneul_types::haneul_system_state::haneul_system_state_summary::HaneulSystemStateSummary;

use crate::api::GovernanceReadApiServer;
use crate::error::Error;
use crate::HaneulRpcModule;
use async_trait::async_trait;
use jsonrpsee::RpcModule;
use haneul_core::authority::AuthorityState;
use haneul_open_rpc::Module;
use haneul_types::base_types::HaneulAddress;
use haneul_types::committee::EpochId;
use haneul_types::governance::{DelegatedStake, Delegation, DelegationStatus, StakedHaneul};
use haneul_types::haneul_system_state::HaneulSystemStateTrait;

pub struct GovernanceReadApi {
    state: Arc<AuthorityState>,
}

impl GovernanceReadApi {
    pub fn new(state: Arc<AuthorityState>) -> Self {
        Self { state }
    }

    async fn get_staked_haneul(&self, owner: HaneulAddress) -> Result<Vec<StakedHaneul>, Error> {
        Ok(self
            .state
            .get_move_objects(owner, &StakedHaneul::type_())
            .await?)
    }
    async fn get_delegations(&self, owner: HaneulAddress) -> Result<Vec<Delegation>, Error> {
        Ok(self
            .state
            .get_move_objects(owner, &Delegation::type_())
            .await?)
    }
}

#[async_trait]
impl GovernanceReadApiServer for GovernanceReadApi {
    async fn get_delegated_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>> {
        let delegation = self
            .get_delegations(owner)
            .await?
            .into_iter()
            .map(|d| (d.staked_haneul_id.bytes, d))
            .collect::<HashMap<_, _>>();

        Ok(self
            .get_staked_haneul(owner)
            .await?
            .into_iter()
            .map(|staked_haneul| {
                let id = staked_haneul.id();
                DelegatedStake {
                    staked_haneul,
                    delegation_status: delegation
                        .get(&id)
                        .cloned()
                        .map_or(DelegationStatus::Pending, DelegationStatus::Active),
                }
            })
            .collect())
    }

    async fn get_validators(&self) -> RpcResult<Vec<ValidatorMetadataV1>> {
        // TODO: include pending validators as well when the necessary changes are made in move.
        Ok(self
            .state
            .database
            .get_haneul_system_state_object()
            .map_err(Error::from)?
            .get_validator_metadata_vec())
    }

    async fn get_committee_info(&self, epoch: Option<EpochId>) -> RpcResult<HaneulCommittee> {
        Ok(self
            .state
            .committee_store()
            .get_or_latest_committee(epoch)
            .map(|committee| committee.into())
            .map_err(Error::from)?)
    }

    async fn get_haneul_system_state(&self) -> RpcResult<HaneulSystemStateRpc> {
        Ok(self
            .state
            .database
            .get_haneul_system_state_object()
            .map_err(Error::from)?
            .into())
    }

    async fn get_latest_haneul_system_state(&self) -> RpcResult<HaneulSystemStateSummary> {
        Ok(self
            .state
            .database
            .get_haneul_system_state_object()
            .map_err(Error::from)?
            .into_haneul_system_state_summary())
    }

    async fn get_reference_gas_price(&self) -> RpcResult<u64> {
        Ok(self
            .state
            .database
            .get_haneul_system_state_object()
            .map_err(Error::from)?
            .reference_gas_price())
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
