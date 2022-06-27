// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Error;
use async_trait::async_trait;
use tokio::runtime::Handle;

use haneul_core::gateway_state::{GatewayAPI, GatewayTxSeqNumber};
use haneul_json::HaneulJsonValue;
use haneul_json_rpc_api::client::HaneulRpcClient;
use haneul_json_rpc_api::rpc_types::{
    GetObjectDataResponse, GetRawObjectDataResponse, RPCTransactionRequestParams, HaneulObjectInfo,
    HaneulTypeTag, TransactionEffectsResponse, TransactionResponse,
};
use haneul_json_rpc_api::QuorumDriverApiClient;
use haneul_json_rpc_api::RpcBcsApiClient;
use haneul_json_rpc_api::RpcTransactionBuilderClient;
use haneul_json_rpc_api::TransactionBytes;
use haneul_json_rpc_api::WalletSyncApiClient;
use haneul_types::base_types::{ObjectID, HaneulAddress, TransactionDigest};
use haneul_types::messages::{Transaction, TransactionData};
use haneul_types::haneul_serde::Base64;
pub struct RpcGatewayClient {
    client: HaneulRpcClient,
}
use haneul_json_rpc_api::RpcReadApiClient;
impl RpcGatewayClient {
    pub fn new(server_url: String) -> Result<Self, anyhow::Error> {
        Ok(Self {
            client: HaneulRpcClient::new(&server_url)?,
        })
    }
}

#[async_trait]
impl GatewayAPI for RpcGatewayClient {
    async fn execute_transaction(&self, tx: Transaction) -> Result<TransactionResponse, Error> {
        let signature = tx.tx_signature;
        let tx_bytes = Base64::from_bytes(&tx.data.to_bytes());
        let signature_bytes = Base64::from_bytes(signature.signature_bytes());
        let pub_key = Base64::from_bytes(signature.public_key_bytes());

        Ok(self
            .client
            .quorum_driver()
            .execute_transaction(tx_bytes, signature_bytes, pub_key)
            .await?)
    }

    async fn transfer_coin(
        &self,
        signer: HaneulAddress,
        object_id: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
        recipient: HaneulAddress,
    ) -> Result<TransactionData, Error> {
        let bytes: TransactionBytes = self
            .client
            .transaction_builder()
            .transfer_coin(signer, object_id, gas, gas_budget, recipient)
            .await?;
        bytes.to_data()
    }

    async fn transfer_haneul(
        &self,
        signer: HaneulAddress,
        haneul_object_id: ObjectID,
        gas_budget: u64,
        recipient: HaneulAddress,
        amount: Option<u64>,
    ) -> Result<TransactionData, Error> {
        let bytes: TransactionBytes = self
            .client
            .transaction_builder()
            .transfer_haneul(signer, haneul_object_id, gas_budget, recipient, amount)
            .await?;
        bytes.to_data()
    }

    async fn sync_account_state(&self, account_addr: HaneulAddress) -> Result<(), Error> {
        self.client
            .wallet_sync_api()
            .sync_account_state(account_addr)
            .await?;
        Ok(())
    }

    async fn move_call(
        &self,
        signer: HaneulAddress,
        package_object_id: ObjectID,
        module: String,
        function: String,
        type_arguments: Vec<HaneulTypeTag>,
        arguments: Vec<HaneulJsonValue>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> Result<TransactionData, Error> {
        let bytes: TransactionBytes = self
            .client
            .transaction_builder()
            .move_call(
                signer,
                package_object_id,
                module,
                function,
                type_arguments,
                arguments,
                gas,
                gas_budget,
            )
            .await?;
        bytes.to_data()
    }

    async fn publish(
        &self,
        signer: HaneulAddress,
        package_bytes: Vec<Vec<u8>>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> Result<TransactionData, Error> {
        let package_bytes = package_bytes
            .iter()
            .map(|bytes| Base64::from_bytes(bytes))
            .collect();
        let bytes: TransactionBytes = self
            .client
            .transaction_builder()
            .publish(signer, package_bytes, gas, gas_budget)
            .await?;
        bytes.to_data()
    }

    async fn split_coin(
        &self,
        signer: HaneulAddress,
        coin_object_id: ObjectID,
        split_amounts: Vec<u64>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> Result<TransactionData, Error> {
        let bytes: TransactionBytes = self
            .client
            .transaction_builder()
            .split_coin(signer, coin_object_id, split_amounts, gas, gas_budget)
            .await?;
        bytes.to_data()
    }

    async fn merge_coins(
        &self,
        signer: HaneulAddress,
        primary_coin: ObjectID,
        coin_to_merge: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> Result<TransactionData, Error> {
        let bytes: TransactionBytes = self
            .client
            .transaction_builder()
            .merge_coin(signer, primary_coin, coin_to_merge, gas, gas_budget)
            .await?;
        bytes.to_data()
    }

    async fn batch_transaction(
        &self,
        signer: HaneulAddress,
        single_transaction_params: Vec<RPCTransactionRequestParams>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> Result<TransactionData, Error> {
        let bytes: TransactionBytes = self
            .client
            .transaction_builder()
            .batch_transaction(signer, single_transaction_params, gas, gas_budget)
            .await?;
        bytes.to_data()
    }

    async fn get_object(&self, object_id: ObjectID) -> Result<GetObjectDataResponse, Error> {
        Ok(self.client.read_api().get_object(object_id).await?)
    }

    async fn get_raw_object(&self, object_id: ObjectID) -> Result<GetRawObjectDataResponse, Error> {
        Ok(self.client.read_api().get_raw_object(object_id).await?)
    }

    async fn get_objects_owned_by_address(
        &self,
        address: HaneulAddress,
    ) -> Result<Vec<HaneulObjectInfo>, Error> {
        Ok(self
            .client
            .read_api()
            .get_objects_owned_by_address(address)
            .await?)
    }

    async fn get_objects_owned_by_object(
        &self,
        object_id: ObjectID,
    ) -> Result<Vec<HaneulObjectInfo>, Error> {
        Ok(self
            .client
            .read_api()
            .get_objects_owned_by_object(object_id)
            .await?)
    }

    fn get_total_transaction_number(&self) -> Result<u64, Error> {
        let handle = Handle::current();
        let _ = handle.enter();
        Ok(futures::executor::block_on(
            self.client.read_api().get_total_transaction_number(),
        )?)
    }

    fn get_transactions_in_range(
        &self,
        start: GatewayTxSeqNumber,
        end: GatewayTxSeqNumber,
    ) -> Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>, Error> {
        let handle = Handle::current();
        let _ = handle.enter();
        Ok(futures::executor::block_on(
            self.client.read_api().get_transactions_in_range(start, end),
        )?)
    }

    fn get_recent_transactions(
        &self,
        count: u64,
    ) -> Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>, Error> {
        let handle = Handle::current();
        let _ = handle.enter();
        Ok(futures::executor::block_on(
            self.client.read_api().get_recent_transactions(count),
        )?)
    }

    async fn get_transaction(
        &self,
        digest: TransactionDigest,
    ) -> Result<TransactionEffectsResponse, Error> {
        Ok(self.client.read_api().get_transaction(digest).await?)
    }
}
