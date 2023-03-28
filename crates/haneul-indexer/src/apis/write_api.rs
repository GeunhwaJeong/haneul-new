// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;
use haneul_json_rpc::api::{WriteApiClient, WriteApiServer};
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_types::{
    BigInt, DevInspectResults, DryRunTransactionBlockResponse, HaneulTransactionBlockResponse,
    HaneulTransactionBlockResponseOptions,
};
use haneul_open_rpc::Module;
use haneul_types::base_types::{EpochId, HaneulAddress};
use haneul_types::messages::ExecuteTransactionRequestType;

use crate::models::transactions::Transaction;
use crate::store::IndexerStore;
use crate::types::{
    FastPathTransactionBlockResponse, HaneulTransactionBlockResponseWithOptions,
    TemporaryTransactionBlockResponseStore,
};

pub(crate) struct WriteApi<S> {
    fullnode: HttpClient,
    state: S,
}

impl<S: IndexerStore> WriteApi<S> {
    pub fn new(state: S, fullnode_client: HttpClient) -> Self {
        Self {
            state,
            fullnode: fullnode_client,
        }
    }
}

#[async_trait]
impl<S> WriteApiServer for WriteApi<S>
where
    S: IndexerStore + Sync + Send + 'static,
{
    async fn execute_transaction_block(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        options: Option<HaneulTransactionBlockResponseOptions>,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<HaneulTransactionBlockResponse> {
        let fast_path_options = HaneulTransactionBlockResponseOptions::full_content();
        let haneul_transaction_response = self
            .fullnode
            .execute_transaction_block(tx_bytes, signatures, Some(fast_path_options), request_type)
            .await?;

        let fast_path_resp: FastPathTransactionBlockResponse =
            haneul_transaction_response.clone().try_into()?;
        let transaction_store: TemporaryTransactionBlockResponseStore = fast_path_resp.into();
        let transaction: Transaction = transaction_store.try_into()?;
        self.state.persist_fast_path(transaction)?;

        Ok(HaneulTransactionBlockResponseWithOptions {
            response: haneul_transaction_response,
            options: options.unwrap_or_default(),
        }
        .into())
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender_address: HaneulAddress,
        tx_bytes: Base64,
        gas_price: Option<BigInt>,
        epoch: Option<EpochId>,
    ) -> RpcResult<DevInspectResults> {
        self.fullnode
            .dev_inspect_transaction_block(sender_address, tx_bytes, gas_price, epoch)
            .await
    }

    async fn dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> RpcResult<DryRunTransactionBlockResponse> {
        self.fullnode.dry_run_transaction_block(tx_bytes).await
    }
}

impl<S> HaneulRpcModule for WriteApi<S>
where
    S: IndexerStore + Sync + Send + 'static,
{
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc::api::WriteApiOpenRpc::module_doc()
    }
}
