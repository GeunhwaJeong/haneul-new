// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;
use haneul_json_rpc::api::{GovernanceReadApiClient, GovernanceReadApiServer};
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_types::{HaneulCommittee, HaneulSystemStateRpc};
use haneul_open_rpc::Module;
use haneul_types::base_types::{EpochId, HaneulAddress};
use haneul_types::governance::DelegatedStake;
use haneul_types::haneul_system_state::ValidatorMetadata;

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
    async fn get_delegated_stakes(&self, owner: HaneulAddress) -> RpcResult<Vec<DelegatedStake>> {
        self.fullnode.get_delegated_stakes(owner).await
    }

    async fn get_validators(&self) -> RpcResult<Vec<ValidatorMetadata>> {
        self.fullnode.get_validators().await
    }

    async fn get_committee_info(&self, epoch: Option<EpochId>) -> RpcResult<HaneulCommittee> {
        self.fullnode.get_committee_info(epoch).await
    }

    async fn get_haneul_system_state(&self) -> RpcResult<HaneulSystemStateRpc> {
        self.fullnode.get_haneul_system_state().await
    }

    async fn get_reference_gas_price(&self) -> RpcResult<u64> {
        self.fullnode.get_reference_gas_price().await
    }
}

impl HaneulRpcModule for GovernanceReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc::api::GovernanceReadApiOpenRpc::module_doc()
    }
}
