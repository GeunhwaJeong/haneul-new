// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// TODO remove after the functions are implemented
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::store::PgIndexerStoreV2;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use haneul_json_rpc::api::GovernanceReadApiServer;
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_types::HaneulCommittee;
use haneul_json_rpc_types::{DelegatedStake, ValidatorApys};
use haneul_open_rpc::Module;
use haneul_types::base_types::{ObjectID, HaneulAddress};
use haneul_types::haneul_serde::BigInt;
use haneul_types::haneul_system_state::haneul_system_state_summary::HaneulSystemStateSummary;

pub(crate) struct GovernanceReadApiV2 {
    pg_store: PgIndexerStoreV2,
}

impl GovernanceReadApiV2 {
    pub fn new(pg_store: PgIndexerStoreV2) -> Self {
        Self { pg_store }
    }
}

#[async_trait]
impl GovernanceReadApiServer for GovernanceReadApiV2 {
    async fn get_stakes_by_ids(
        &self,
        staked_haneul_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        unimplemented!()
    }
    async fn get_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>> {
        unimplemented!()
    }

    async fn get_committee_info(&self, epoch: Option<BigInt<u64>>) -> RpcResult<HaneulCommittee> {
        unimplemented!()
    }

    async fn get_latest_haneul_system_state(&self) -> RpcResult<HaneulSystemStateSummary> {
        unimplemented!()
    }

    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        unimplemented!()
    }

    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys> {
        unimplemented!()
    }
}

impl HaneulRpcModule for GovernanceReadApiV2 {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc::api::GovernanceReadApiOpenRpc::module_doc()
    }
}
