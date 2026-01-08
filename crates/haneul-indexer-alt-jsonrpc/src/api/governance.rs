// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use diesel::ExpressionMethods;
use diesel::QueryDsl;

use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::proc_macros::rpc;
use haneul_indexer_alt_schema::schema::kv_epoch_starts;
use haneul_json_rpc_api::GovernanceReadApiClient;
use haneul_json_rpc_types::DelegatedStake;
use haneul_json_rpc_types::ValidatorApys;
use haneul_open_rpc::Module;
use haneul_open_rpc_macros::open_rpc;
use haneul_types::HANEUL_SYSTEM_STATE_OBJECT_ID;
use haneul_types::TypeTag;
use haneul_types::base_types::ObjectID;
use haneul_types::base_types::HaneulAddress;
use haneul_types::dynamic_field::Field;
use haneul_types::dynamic_field::derive_dynamic_field_id;
use haneul_types::haneul_serde::BigInt;
use haneul_types::haneul_system_state::HaneulSystemStateTrait;
use haneul_types::haneul_system_state::HaneulSystemStateWrapper;
use haneul_types::haneul_system_state::haneul_system_state_inner_v1::HaneulSystemStateInnerV1;
use haneul_types::haneul_system_state::haneul_system_state_inner_v2::HaneulSystemStateInnerV2;
use haneul_types::haneul_system_state::haneul_system_state_summary::HaneulSystemStateSummary;

use crate::api::rpc_module::RpcModule;
use crate::context::Context;
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
}

#[open_rpc(namespace = "haneulx", tag = "Delegation Governance API")]
#[rpc(server, namespace = "haneulx")]
trait DelegationGovernanceApi {
    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_haneul_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>>;

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
}

#[async_trait::async_trait]
impl DelegationGovernanceApiServer for DelegationGovernance {
    async fn get_stakes_by_ids(
        &self,
        staked_haneul_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        let Self(client) = self;

        client
            .get_stakes_by_ids(staked_haneul_ids)
            .await
            .map_err(client_error_to_error_object)
    }

    async fn get_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>> {
        let Self(client) = self;

        client
            .get_stakes(owner)
            .await
            .map_err(client_error_to_error_object)
    }

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
