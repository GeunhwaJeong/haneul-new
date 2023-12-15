// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_api::CoinReadApiClient;
use haneul_json_rpc_api::CoinReadApiServer;
use haneul_json_rpc_types::{Balance, CoinPage, HaneulCoinMetadata};
use haneul_open_rpc::Module;
use haneul_types::balance::Supply;
use haneul_types::base_types::{ObjectID, HaneulAddress};

pub(crate) struct CoinReadApi {
    fullnode: HttpClient,
}

impl CoinReadApi {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self {
            fullnode: fullnode_client,
        }
    }
}

#[async_trait]
impl CoinReadApiServer for CoinReadApi {
    async fn get_coins(
        &self,
        owner: HaneulAddress,
        coin_type: Option<String>,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<CoinPage> {
        self.fullnode
            .get_coins(owner, coin_type, cursor, limit)
            .await
    }

    async fn get_all_coins(
        &self,
        owner: HaneulAddress,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<CoinPage> {
        self.fullnode.get_all_coins(owner, cursor, limit).await
    }

    async fn get_balance(
        &self,
        owner: HaneulAddress,
        coin_type: Option<String>,
    ) -> RpcResult<Balance> {
        self.fullnode.get_balance(owner, coin_type).await
    }

    async fn get_all_balances(&self, owner: HaneulAddress) -> RpcResult<Vec<Balance>> {
        self.fullnode.get_all_balances(owner).await
    }

    async fn get_coin_metadata(&self, coin_type: String) -> RpcResult<Option<HaneulCoinMetadata>> {
        self.fullnode.get_coin_metadata(coin_type).await
    }

    async fn get_total_supply(&self, coin_type: String) -> RpcResult<Supply> {
        self.fullnode.get_total_supply(coin_type).await
    }
}

impl HaneulRpcModule for CoinReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc_api::CoinReadApiOpenRpc::module_doc()
    }
}
