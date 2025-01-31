// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use diesel::{ExpressionMethods, QueryDsl};

use jsonrpsee::{
    core::{DeserializeOwned, RpcResult},
    proc_macros::rpc,
};
use haneul_indexer_alt_schema::schema::kv_epoch_starts;
use haneul_open_rpc::Module;
use haneul_open_rpc_macros::open_rpc;
use haneul_types::{
    base_types::ObjectID,
    dynamic_field::{derive_dynamic_field_id, Field},
    object::Object,
    haneul_serde::BigInt,
    haneul_system_state::{
        haneul_system_state_inner_v1::HaneulSystemStateInnerV1,
        haneul_system_state_inner_v2::HaneulSystemStateInnerV2,
        haneul_system_state_summary::HaneulSystemStateSummary, HaneulSystemStateTrait,
        HaneulSystemStateWrapper,
    },
    TypeTag, HANEUL_SYSTEM_STATE_OBJECT_ID,
};

use crate::{
    context::Context,
    data::objects::load_latest,
    error::{internal_error, rpc_bail, InternalContext, RpcError},
};

use super::rpc_module::RpcModule;

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

pub(crate) struct Governance(pub Context);

#[async_trait::async_trait]
impl GovernanceApiServer for Governance {
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        Ok(rgp_response(&self.0).await?)
    }

    async fn get_latest_haneul_system_state(&self) -> RpcResult<HaneulSystemStateSummary> {
        Ok(latest_haneul_system_state_response(&self.0).await?)
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

/// Load data and generate response for `getReferenceGasPrice`.
async fn rgp_response(ctx: &Context) -> Result<BigInt<u64>, RpcError> {
    use kv_epoch_starts::dsl as e;

    let mut conn = ctx
        .reader()
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
    let wrapper: HaneulSystemStateWrapper =
        fetch_latest_for_system_state(ctx, HANEUL_SYSTEM_STATE_OBJECT_ID)
            .await
            .internal_context("Failed to fetch system state wrapper object")?;

    let inner_id = derive_dynamic_field_id(
        HANEUL_SYSTEM_STATE_OBJECT_ID,
        &TypeTag::U64,
        &bcs::to_bytes(&wrapper.version).context("Failed to serialize system state version")?,
    )
    .context("Failed to derive inner system state field ID")?;

    Ok(match wrapper.version {
        1 => fetch_latest_for_system_state::<Field<u64, HaneulSystemStateInnerV1>>(ctx, inner_id)
            .await
            .internal_context("Failed to fetch inner system state object")?
            .value
            .into_haneul_system_state_summary(),
        2 => fetch_latest_for_system_state::<Field<u64, HaneulSystemStateInnerV2>>(ctx, inner_id)
            .await
            .internal_context("Failed to fetch inner system state object")?
            .value
            .into_haneul_system_state_summary(),
        v => rpc_bail!("Unexpected inner system state version: {v}"),
    })
}

/// Fetch the latest version of the object at ID `object_id`, and deserialize its contents as a
/// Rust type `T`, assuming that it is a Move object (not a package).
async fn fetch_latest_for_system_state<T: DeserializeOwned>(
    ctx: &Context,
    object_id: ObjectID,
) -> Result<T, RpcError> {
    let stored = load_latest(ctx.loader(), object_id)
        .await?
        .ok_or_else(|| internal_error!("No data found"))?
        .serialized_object
        .ok_or_else(|| internal_error!("No content found"))?;

    let object: Object =
        bcs::from_bytes(&stored).context("Failed to deserialize object contents")?;

    let move_object = object
        .data
        .try_as_move()
        .ok_or_else(|| internal_error!("Not a Move object"))?;

    Ok(bcs::from_bytes(move_object.contents()).context("Failed to deserialize Move value")?)
}
