// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use async_trait::async_trait;
use ed25519_dalek::ed25519::signature::Signature;
use jsonrpsee::core::RpcResult;
use jsonrpsee_core::server::rpc_module::RpcModule;
use tracing::debug;

use crate::HaneulRpcModule;
use haneul_core::gateway_state::{GatewayClient, GatewayTxSeqNumber};
use haneul_json::HaneulJsonValue;
use haneul_json_rpc_api::rpc_types::{
    GetObjectDataResponse, HaneulObjectInfo, TransactionEffectsResponse, TransactionResponse,
};
use haneul_json_rpc_api::rpc_types::{RPCTransactionRequestParams, HaneulTypeTag};
use haneul_json_rpc_api::{
    RpcGatewayApiServer, RpcReadApiServer, RpcTransactionBuilderServer, TransactionBytes,
    WalletSyncApiServer,
};
use haneul_open_rpc::Module;
use haneul_types::haneul_serde::Base64;
use haneul_types::{
    base_types::{ObjectID, HaneulAddress, TransactionDigest},
    crypto,
    crypto::SignableBytes,
    messages::{Transaction, TransactionData},
};

pub struct RpcGatewayImpl {
    client: GatewayClient,
}

pub struct GatewayWalletSyncApiImpl {
    client: GatewayClient,
}

pub struct GatewayReadApiImpl {
    client: GatewayClient,
}

pub struct TransactionBuilderImpl {
    client: GatewayClient,
}

impl RpcGatewayImpl {
    pub fn new(client: GatewayClient) -> Self {
        Self { client }
    }
}

impl GatewayWalletSyncApiImpl {
    pub fn new(client: GatewayClient) -> Self {
        Self { client }
    }
}

impl GatewayReadApiImpl {
    pub fn new(client: GatewayClient) -> Self {
        Self { client }
    }
}
impl TransactionBuilderImpl {
    pub fn new(client: GatewayClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RpcGatewayApiServer for RpcGatewayImpl {
    async fn execute_transaction(
        &self,
        tx_bytes: Base64,
        signature: Base64,
        pub_key: Base64,
    ) -> RpcResult<TransactionResponse> {
        let data = TransactionData::from_signable_bytes(&tx_bytes.to_vec()?)?;
        let signature =
            crypto::Signature::from_bytes(&[&*signature.to_vec()?, &*pub_key.to_vec()?].concat())
                .map_err(|e| anyhow!(e))?;
        let result = self
            .client
            .execute_transaction(Transaction::new(data, signature))
            .await;
        Ok(result?)
    }
}

impl HaneulRpcModule for RpcGatewayImpl {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc_api::RpcGatewayApiOpenRpc::module_doc()
    }
}

#[async_trait]
impl WalletSyncApiServer for GatewayWalletSyncApiImpl {
    async fn sync_account_state(&self, address: HaneulAddress) -> RpcResult<()> {
        debug!("sync_account_state : {}", address);
        self.client.sync_account_state(address).await?;
        Ok(())
    }
}

impl HaneulRpcModule for GatewayWalletSyncApiImpl {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc_api::WalletSyncApiOpenRpc::module_doc()
    }
}

#[async_trait]
impl RpcReadApiServer for GatewayReadApiImpl {
    async fn get_objects_owned_by_address(
        &self,
        address: HaneulAddress,
    ) -> RpcResult<Vec<HaneulObjectInfo>> {
        debug!("get_objects_own_by_address : {}", address);
        Ok(self.client.get_objects_owned_by_address(address).await?)
    }

    async fn get_objects_owned_by_object(
        &self,
        object_id: ObjectID,
    ) -> RpcResult<Vec<HaneulObjectInfo>> {
        debug!("get_objects_own_by_object : {}", object_id);
        Ok(self.client.get_objects_owned_by_object(object_id).await?)
    }

    async fn get_object(&self, object_id: ObjectID) -> RpcResult<GetObjectDataResponse> {
        Ok(self.client.get_object(object_id).await?)
    }

    async fn get_recent_transactions(
        &self,
        count: u64,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(self.client.get_recent_transactions(count)?)
    }

    async fn get_transaction(
        &self,
        digest: TransactionDigest,
    ) -> RpcResult<TransactionEffectsResponse> {
        Ok(self.client.get_transaction(digest).await?)
    }

    async fn get_total_transaction_number(&self) -> RpcResult<u64> {
        Ok(self.client.get_total_transaction_number()?)
    }

    async fn get_transactions_in_range(
        &self,
        start: GatewayTxSeqNumber,
        end: GatewayTxSeqNumber,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(self.client.get_transactions_in_range(start, end)?)
    }
}

impl HaneulRpcModule for GatewayReadApiImpl {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc_api::RpcReadApiOpenRpc::module_doc()
    }
}

#[async_trait]
impl RpcTransactionBuilderServer for TransactionBuilderImpl {
    async fn public_transfer_object(
        &self,
        signer: HaneulAddress,
        object_id: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
        recipient: HaneulAddress,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .client
            .public_transfer_object(signer, object_id, gas, gas_budget, recipient)
            .await?;
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn transfer_haneul(
        &self,
        signer: HaneulAddress,
        haneul_object_id: ObjectID,
        gas_budget: u64,
        recipient: HaneulAddress,
        amount: Option<u64>,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .client
            .transfer_haneul(signer, haneul_object_id, gas_budget, recipient, amount)
            .await?;
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn publish(
        &self,
        sender: HaneulAddress,
        compiled_modules: Vec<Base64>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        let compiled_modules = compiled_modules
            .into_iter()
            .map(|data| data.to_vec())
            .collect::<Result<Vec<_>, _>>()?;
        let data = self
            .client
            .publish(sender, compiled_modules, gas, gas_budget)
            .await?;

        Ok(TransactionBytes::from_data(data)?)
    }

    async fn split_coin(
        &self,
        signer: HaneulAddress,
        coin_object_id: ObjectID,
        split_amounts: Vec<u64>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .client
            .split_coin(signer, coin_object_id, split_amounts, gas, gas_budget)
            .await?;
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn merge_coin(
        &self,
        signer: HaneulAddress,
        primary_coin: ObjectID,
        coin_to_merge: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .client
            .merge_coins(signer, primary_coin, coin_to_merge, gas, gas_budget)
            .await?;
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn move_call(
        &self,
        signer: HaneulAddress,
        package_object_id: ObjectID,
        module: String,
        function: String,
        type_arguments: Vec<HaneulTypeTag>,
        rpc_arguments: Vec<HaneulJsonValue>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        let data = async {
            self.client
                .move_call(
                    signer,
                    package_object_id,
                    module,
                    function,
                    type_arguments,
                    rpc_arguments,
                    gas,
                    gas_budget,
                )
                .await
        }
        .await?;
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn batch_transaction(
        &self,
        signer: HaneulAddress,
        params: Vec<RPCTransactionRequestParams>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        let data = async {
            self.client
                .batch_transaction(signer, params, gas, gas_budget)
                .await
        }
        .await?;
        Ok(TransactionBytes::from_data(data)?)
    }
}

impl HaneulRpcModule for TransactionBuilderImpl {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc_api::RpcTransactionBuilderOpenRpc::module_doc()
    }
}
