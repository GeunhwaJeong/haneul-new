// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// TODO remove after the functions are implemented
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::store::PgIndexerStoreV2;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use haneul_json_rpc::api::CoinReadApiServer;
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_types::{Balance, CoinPage, HaneulCoinMetadata};
use haneul_open_rpc::Module;
use haneul_types::balance::Supply;
use haneul_types::base_types::{ObjectID, HaneulAddress};

pub(crate) struct CoinReadApiV2 {
    pg_store: PgIndexerStoreV2,
}

impl CoinReadApiV2 {
    pub fn new(pg_store: PgIndexerStoreV2) -> Self {
        Self { pg_store }
    }
}

#[async_trait]
impl CoinReadApiServer for CoinReadApiV2 {
    async fn get_coins(
        &self,
        owner: HaneulAddress,
        coin_type: Option<String>,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<CoinPage> {
        unimplemented!()
    }

    async fn get_all_coins(
        &self,
        owner: HaneulAddress,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<CoinPage> {
        unimplemented!()
    }

    async fn get_balance(
        &self,
        owner: HaneulAddress,
        coin_type: Option<String>,
    ) -> RpcResult<Balance> {
        unimplemented!()
    }

    async fn get_all_balances(&self, _owner: HaneulAddress) -> RpcResult<Vec<Balance>> {
        unimplemented!()
    }

    async fn get_coin_metadata(&self, _coin_type: String) -> RpcResult<Option<HaneulCoinMetadata>> {
        unimplemented!()
    }

    async fn get_total_supply(&self, _coin_type: String) -> RpcResult<Supply> {
        unimplemented!()
    }
}

impl HaneulRpcModule for CoinReadApiV2 {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc::api::CoinReadApiOpenRpc::module_doc()
    }
}
