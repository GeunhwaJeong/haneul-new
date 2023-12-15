// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_api::{GovernanceReadApiClient, GovernanceReadApiServer};
use haneul_json_rpc_types::HaneulCommittee;
use haneul_json_rpc_types::{DelegatedStake, ValidatorApys};
use haneul_open_rpc::Module;
use haneul_types::base_types::{ObjectID, HaneulAddress};
use haneul_types::haneul_serde::BigInt;
use haneul_types::haneul_system_state::haneul_system_state_summary::HaneulSystemStateSummary;

pub(crate) struct GovernanceReadApi {
    fullnode: HttpClient,
}

impl GovernanceReadApi {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self {
            fullnode: fullnode_client,
        }
    }
}

#[async_trait]
impl GovernanceReadApiServer for GovernanceReadApi {
    async fn get_stakes_by_ids(
        &self,
        staked_haneul_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        self.fullnode.get_stakes_by_ids(staked_haneul_ids).await
    }
    async fn get_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>> {
        self.fullnode.get_stakes(owner).await
    }

    async fn get_committee_info(&self, epoch: Option<BigInt<u64>>) -> RpcResult<HaneulCommittee> {
        self.fullnode.get_committee_info(epoch).await
    }

    async fn get_latest_haneul_system_state(&self) -> RpcResult<HaneulSystemStateSummary> {
        self.fullnode.get_latest_haneul_system_state().await
    }

    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        self.fullnode.get_reference_gas_price().await
    }

    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys> {
        self.fullnode.get_validators_apy().await
    }
}

impl HaneulRpcModule for GovernanceReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc_api::GovernanceReadApiOpenRpc::module_doc()
    }
}
