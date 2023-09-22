// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// TODO remove after the functions are implemented
#![allow(unused_variables)]
#![allow(dead_code)]

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use haneul_json_rpc::api::WriteApiServer;
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_types::{
    DevInspectResults, DryRunTransactionBlockResponse, HaneulTransactionBlockResponse,
    HaneulTransactionBlockResponseOptions,
};
use haneul_open_rpc::Module;
use haneul_types::base_types::HaneulAddress;
use haneul_types::quorum_driver_types::ExecuteTransactionRequestType;
use haneul_types::haneul_serde::BigInt;

pub(crate) struct WriteApiV2 {
    fullnode_client: HttpClient,
}

impl WriteApiV2 {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self { fullnode_client }
    }
}

#[async_trait]
impl WriteApiServer for WriteApiV2 {
    async fn execute_transaction_block(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        options: Option<HaneulTransactionBlockResponseOptions>,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<HaneulTransactionBlockResponse> {
        unimplemented!()
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender_address: HaneulAddress,
        tx_bytes: Base64,
        gas_price: Option<BigInt<u64>>,
        epoch: Option<BigInt<u64>>,
    ) -> RpcResult<DevInspectResults> {
        unimplemented!()
    }

    async fn dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> RpcResult<DryRunTransactionBlockResponse> {
        unimplemented!()
    }
}

impl HaneulRpcModule for WriteApiV2 {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc::api::WriteApiOpenRpc::module_doc()
    }
}
