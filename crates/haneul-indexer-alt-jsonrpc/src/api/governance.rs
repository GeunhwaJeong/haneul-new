// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::convert::Infallible;

use anyhow::Context as _;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use futures::future;

use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::proc_macros::rpc;
use move_core_types::language_storage::StructTag;

use haneul_indexer_alt_reader::consistent_reader::proto::owner::OwnerKind;
use haneul_indexer_alt_reader::governance::RewardsKey;
use haneul_indexer_alt_reader::governance::ValidatorAddressKey;
use haneul_indexer_alt_schema::schema::kv_epoch_starts;
use haneul_json_rpc_api::GovernanceReadApiClient;
use haneul_json_rpc_types::DelegatedStake;
use haneul_json_rpc_types::Stake;
use haneul_json_rpc_types::StakeStatus;
use haneul_json_rpc_types::ValidatorApys;
use haneul_open_rpc::Module;
use haneul_open_rpc_macros::open_rpc;
use haneul_types::HANEUL_SYSTEM_ADDRESS;
use haneul_types::HANEUL_SYSTEM_STATE_OBJECT_ID;
use haneul_types::TypeTag;
use haneul_types::base_types::ObjectID;
use haneul_types::base_types::HaneulAddress;
use haneul_types::dynamic_field::Field;
use haneul_types::dynamic_field::derive_dynamic_field_id;
use haneul_types::governance::STAKED_HANEUL_STRUCT_NAME;
use haneul_types::governance::STAKING_POOL_MODULE_NAME;
use haneul_types::governance::StakedHaneul;
use haneul_types::haneul_serde::BigInt;
use haneul_types::haneul_system_state::HaneulSystemStateTrait;
use haneul_types::haneul_system_state::HaneulSystemStateWrapper;
use haneul_types::haneul_system_state::haneul_system_state_inner_v1::HaneulSystemStateInnerV1;
use haneul_types::haneul_system_state::haneul_system_state_inner_v2::HaneulSystemStateInnerV2;
use haneul_types::haneul_system_state::haneul_system_state_summary::HaneulSystemStateSummary;
use tokio::try_join;

use crate::api::rpc_module::RpcModule;
use crate::context::Context;
use crate::data::latest_epoch;
use crate::data::load_live;
use crate::data::load_live_deserialized;
use crate::error::RpcError;
use crate::error::client_error_to_error_object;
use crate::error::rpc_bail;

#[open_rpc(namespace = "haneulx", tag = "Governance API")]
#[rpc(server, namespace = "haneulx")]
trait GovernanceApi {
    /// Return the reference gas price for the network as of the latest epoch.
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;

    /// Return a summary of the latest version of the Haneul System State object (0x5), on-chain.
    #[method(name = "getLatestHaneulSystemState")]
    async fn get_latest_haneul_system_state(&self) -> RpcResult<HaneulSystemStateSummary>;

    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_haneul_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>>;
}

#[open_rpc(namespace = "haneulx", tag = "Delegation Governance API")]
#[rpc(server, namespace = "haneulx")]
trait DelegationGovernanceApi {
    /// Return the validator APY
    #[method(name = "getValidatorsApy")]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys>;
}

pub(crate) struct Governance(pub Context);

pub(crate) struct DelegationGovernance(HttpClient);

impl DelegationGovernance {
    pub(crate) fn new(client: HttpClient) -> Self {
        Self(client)
    }
}

#[async_trait::async_trait]
impl GovernanceApiServer for Governance {
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        Ok(rgp_response(&self.0).await?)
    }

    async fn get_latest_haneul_system_state(&self) -> RpcResult<HaneulSystemStateSummary> {
        Ok(latest_haneul_system_state_response(&self.0).await?)
    }

    async fn get_stakes_by_ids(
        &self,
        staked_haneul_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        Ok(delegated_stakes_response(&self.0, staked_haneul_ids).await?)
    }

    async fn get_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>> {
        let Self(ctx) = self;
        let config = &ctx.config().objects;

        let tag = StructTag {
            address: HANEUL_SYSTEM_ADDRESS,
            module: STAKING_POOL_MODULE_NAME.to_owned(),
            name: STAKED_HANEUL_STRUCT_NAME.to_owned(),
            type_params: vec![],
        };

        let mut all_stake_ids: Vec<ObjectID> = Vec::new();
        let mut after_cursor = None;

        loop {
            let page = ctx
                .consistent_reader()
                .list_owned_objects(
                    None,
                    OwnerKind::Address,
                    Some(owner.to_string()),
                    Some(tag.to_canonical_string(true)),
                    Some(config.max_page_size as u32),
                    after_cursor,
                    None,
                    true,
                )
                .await
                .context("Failed to fetch owned StakedHaneul objects")
                .map_err(RpcError::<Infallible>::from)?;

            all_stake_ids.extend(page.results.iter().map(|edge| edge.value.0));

            if page.has_next_page {
                after_cursor = page.results.last().map(|edge| edge.token.clone());
            } else {
                break;
            }
        }

        Ok(delegated_stakes_response(&self.0, all_stake_ids).await?)
    }
}

#[async_trait::async_trait]
impl DelegationGovernanceApiServer for DelegationGovernance {
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys> {
        let Self(client) = self;

        client
            .get_validators_apy()
            .await
            .map_err(client_error_to_error_object)
    }
}

impl RpcModule for Governance {
    fn schema(&self) -> Module {
        GovernanceApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}

impl RpcModule for DelegationGovernance {
    fn schema(&self) -> Module {
        DelegationGovernanceApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}

/// Load data and generate response for `getReferenceGasPrice`.
async fn rgp_response(ctx: &Context) -> Result<BigInt<u64>, RpcError> {
    use kv_epoch_starts::dsl as e;

    let mut conn = ctx
        .pg_reader()
        .connect()
        .await
        .context("Failed to connect to the database")?;

    let rgp: i64 = conn
        .first(
            e::kv_epoch_starts
                .select(e::reference_gas_price)
                .order(e::epoch.desc()),
        )
        .await
        .context("Failed to fetch the reference gas price")?;

    Ok((rgp as u64).into())
}

/// Load data and generate response for `getLatestHaneulSystemState`.
async fn latest_haneul_system_state_response(
    ctx: &Context,
) -> Result<HaneulSystemStateSummary, RpcError> {
    let wrapper: HaneulSystemStateWrapper = load_live_deserialized(ctx, HANEUL_SYSTEM_STATE_OBJECT_ID)
        .await
        .context("Failed to fetch system state wrapper object")?;

    let inner_id = derive_dynamic_field_id(
        HANEUL_SYSTEM_STATE_OBJECT_ID,
        &TypeTag::U64,
        &bcs::to_bytes(&wrapper.version).context("Failed to serialize system state version")?,
    )
    .context("Failed to derive inner system state field ID")?;

    Ok(match wrapper.version {
        1 => load_live_deserialized::<Field<u64, HaneulSystemStateInnerV1>>(ctx, inner_id)
            .await
            .context("Failed to fetch inner system state object")?
            .value
            .into_haneul_system_state_summary(),
        2 => load_live_deserialized::<Field<u64, HaneulSystemStateInnerV2>>(ctx, inner_id)
            .await
            .context("Failed to fetch inner system state object")?
            .value
            .into_haneul_system_state_summary(),
        v => rpc_bail!("Unexpected inner system state version: {v}"),
    })
}

/// Given a list of StakedHaneul object IDs, load them, fetch rewards and validator addresses, and
/// return grouped DelegatedStake entries.
///
/// Returns only live staked objects. Stakes that have been withdrawn (or wrapped, deleted,
/// never existed) will be omitted from the response.
async fn delegated_stakes_response(
    ctx: &Context,
    stake_ids: Vec<ObjectID>,
) -> Result<Vec<DelegatedStake>, RpcError> {
    let execution_loader = ctx.execution_loader()?;

    let staked_haneul_futures = stake_ids.iter().map(|id| load_live(ctx, *id));
    let maybe_objects = future::try_join_all(staked_haneul_futures)
        .await
        .context("Failed to load StakedHaneul objects")?;

    let staked_haneuls: Vec<StakedHaneul> = maybe_objects
        .into_iter()
        .flatten()
        .map(|object| {
            let move_object = object.data.try_as_move().context("Not a Move object")?;
            bcs::from_bytes(move_object.contents()).context("Failed to deserialize StakedHaneul")
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let reward_keys: Vec<RewardsKey> = staked_haneuls
        .iter()
        .map(|s| RewardsKey(s.id().into()))
        .collect();
    let validator_keys: Vec<ValidatorAddressKey> = staked_haneuls
        .iter()
        .map(|s| ValidatorAddressKey(s.pool_id().into()))
        .collect();

    let (rewards, validator_addresses, current_epoch) = try_join!(
        async {
            execution_loader
                .load_many(reward_keys)
                .await
                .context("Failed to dry run rewards calculation")
        },
        async {
            execution_loader
                .load_many(validator_keys)
                .await
                .context("Failed to dry run validator address lookup")
        },
        latest_epoch(ctx),
    )?;

    let mut grouped: BTreeMap<(HaneulAddress, ObjectID), Vec<Stake>> = BTreeMap::new();

    // Clients can at most control which stake ids to query. Only live stakes are loaded. Valid
    // stakes should return a reward (could be 0 for pending stakes) and validator (pools are looked
    // up against active and inactive validators.)
    for staked_haneul in &staked_haneuls {
        let reward_key = RewardsKey(staked_haneul.id().into());
        let validator_key = ValidatorAddressKey(staked_haneul.pool_id().into());

        let estimated_reward = *rewards
            .get(&reward_key)
            .with_context(|| format!("Missing reward for StakedHaneul {}", staked_haneul.id()))?;
        let validator_address = validator_addresses
            .get(&validator_key)
            .map(|addr| HaneulAddress::from(ObjectID::from(*addr)))
            .with_context(|| {
                format!(
                    "Missing validator address for staking pool {}",
                    staked_haneul.pool_id()
                )
            })?;

        let status = if current_epoch >= staked_haneul.activation_epoch() {
            StakeStatus::Active { estimated_reward }
        } else {
            StakeStatus::Pending
        };

        grouped
            .entry((validator_address, staked_haneul.pool_id()))
            .or_default()
            .push(Stake {
                staked_haneul_id: staked_haneul.id(),
                stake_request_epoch: staked_haneul.request_epoch(),
                stake_active_epoch: staked_haneul.activation_epoch(),
                principal: staked_haneul.principal(),
                status,
            });
    }

    Ok(grouped
        .into_iter()
        .map(
            |((validator_address, staking_pool), stakes)| DelegatedStake {
                validator_address,
                staking_pool,
                stakes,
            },
        )
        .collect())
}
