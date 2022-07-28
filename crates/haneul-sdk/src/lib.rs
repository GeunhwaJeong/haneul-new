// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use futures::StreamExt;
use futures_core::Stream;
use jsonrpsee::core::client::Subscription;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use haneul_json::HaneulJsonValue;
use haneul_json_rpc::api::EventStreamingApiClient;
use haneul_json_rpc::api::RpcBcsApiClient;
use haneul_json_rpc::api::RpcFullNodeReadApiClient;
use haneul_json_rpc::api::RpcGatewayApiClient;
use haneul_json_rpc::api::RpcReadApiClient;
use haneul_json_rpc::api::RpcTransactionBuilderClient;
use haneul_json_rpc::api::WalletSyncApiClient;
use haneul_json_rpc_types::{
    GatewayTxSeqNumber, GetObjectDataResponse, GetRawObjectDataResponse,
    RPCTransactionRequestParams, HaneulEventEnvelope, HaneulEventFilter, HaneulObjectInfo, HaneulTypeTag,
    TransactionBytes, TransactionEffectsResponse, TransactionResponse,
};
use haneul_types::base_types::{ObjectID, HaneulAddress, TransactionDigest};
use haneul_types::haneul_serde::Base64;
pub mod crypto;

// re-export essential haneul crates
pub use haneul_json as json;
pub use haneul_json_rpc_types as rpc_types;
pub use haneul_types as types;

pub struct HaneulClient {
    client: Client,
}

impl HaneulClient {
    pub fn new_http_client(server_url: &str) -> Result<Self, anyhow::Error> {
        let client = HttpClientBuilder::default().build(server_url)?;
        Ok(Self {
            client: Client::Http(client),
        })
    }

    pub async fn new_ws_client(server_url: &str) -> Result<Self, anyhow::Error> {
        let client = WsClientBuilder::default().build(server_url).await?;
        Ok(Self {
            client: Client::Ws(client),
        })
    }
}

impl HaneulClient {
    pub async fn get_objects_owned_by_address(
        &self,
        address: HaneulAddress,
    ) -> anyhow::Result<Vec<HaneulObjectInfo>> {
        Ok(match &self.client {
            Client::Http(c) => c.get_objects_owned_by_address(address),
            Client::Ws(c) => c.get_objects_owned_by_address(address),
        }
        .await?)
    }

    pub async fn get_objects_owned_by_object(
        &self,
        object_id: ObjectID,
    ) -> anyhow::Result<Vec<HaneulObjectInfo>> {
        Ok(match &self.client {
            Client::Http(c) => c.get_objects_owned_by_object(object_id),
            Client::Ws(c) => c.get_objects_owned_by_object(object_id),
        }
        .await?)
    }

    pub async fn get_total_transaction_number(&self) -> anyhow::Result<u64> {
        Ok(match &self.client {
            Client::Http(c) => c.get_total_transaction_number(),
            Client::Ws(c) => c.get_total_transaction_number(),
        }
        .await?)
    }

    pub async fn get_transactions_in_range(
        &self,
        start: GatewayTxSeqNumber,
        end: GatewayTxSeqNumber,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &self.client {
            Client::Http(c) => c.get_transactions_in_range(start, end),
            Client::Ws(c) => c.get_transactions_in_range(start, end),
        }
        .await?)
    }

    pub async fn get_recent_transactions(
        &self,
        count: u64,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &self.client {
            Client::Http(c) => c.get_recent_transactions(count),
            Client::Ws(c) => c.get_recent_transactions(count),
        }
        .await?)
    }

    pub async fn get_transaction(
        &self,
        digest: TransactionDigest,
    ) -> anyhow::Result<TransactionEffectsResponse> {
        Ok(match &self.client {
            Client::Http(c) => c.get_transaction(digest),
            Client::Ws(c) => c.get_transaction(digest),
        }
        .await?)
    }

    pub async fn get_object(&self, object_id: ObjectID) -> anyhow::Result<GetObjectDataResponse> {
        Ok(match &self.client {
            Client::Http(c) => c.get_object(object_id),
            Client::Ws(c) => c.get_object(object_id),
        }
        .await?)
    }

    pub async fn get_raw_object(
        &self,
        object_id: ObjectID,
    ) -> anyhow::Result<GetRawObjectDataResponse> {
        Ok(match &self.client {
            Client::Http(c) => c.get_raw_object(object_id),
            Client::Ws(c) => c.get_raw_object(object_id),
        }
        .await?)
    }

    pub async fn get_transactions_by_input_object(
        &self,
        object: ObjectID,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &self.client {
            Client::Http(c) => c.get_transactions_by_input_object(object),
            Client::Ws(c) => c.get_transactions_by_input_object(object),
        }
        .await?)
    }

    pub async fn get_transactions_by_mutated_object(
        &self,
        object: ObjectID,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &self.client {
            Client::Http(c) => c.get_transactions_by_mutated_object(object),
            Client::Ws(c) => c.get_transactions_by_mutated_object(object),
        }
        .await?)
    }

    pub async fn get_transactions_by_move_function(
        &self,
        package: ObjectID,
        module: Option<String>,
        function: Option<String>,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &self.client {
            Client::Http(c) => c.get_transactions_by_move_function(package, module, function),
            Client::Ws(c) => c.get_transactions_by_move_function(package, module, function),
        }
        .await?)
    }

    pub async fn get_transactions_from_addr(
        &self,
        addr: HaneulAddress,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &self.client {
            Client::Http(c) => c.get_transactions_from_addr(addr),
            Client::Ws(c) => c.get_transactions_from_addr(addr),
        }
        .await?)
    }

    pub async fn get_transactions_to_addr(
        &self,
        addr: HaneulAddress,
    ) -> anyhow::Result<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(match &self.client {
            Client::Http(c) => c.get_transactions_to_addr(addr),
            Client::Ws(c) => c.get_transactions_to_addr(addr),
        }
        .await?)
    }

    pub async fn execute_transaction(
        &self,
        tx_bytes: Base64,
        flag: Base64,
        signature: Base64,
        pub_key: Base64,
    ) -> anyhow::Result<TransactionResponse> {
        Ok(match &self.client {
            Client::Http(c) => c.execute_transaction(tx_bytes, flag, signature, pub_key),
            Client::Ws(c) => c.execute_transaction(tx_bytes, flag, signature, pub_key),
        }
        .await?)
    }

    pub async fn transfer_object(
        &self,
        signer: HaneulAddress,
        object_id: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
        recipient: HaneulAddress,
    ) -> anyhow::Result<TransactionBytes> {
        Ok(match &self.client {
            Client::Http(c) => c.transfer_object(signer, object_id, gas, gas_budget, recipient),
            Client::Ws(c) => c.transfer_object(signer, object_id, gas, gas_budget, recipient),
        }
        .await?)
    }

    pub async fn transfer_haneul(
        &self,
        signer: HaneulAddress,
        haneul_object_id: ObjectID,
        gas_budget: u64,
        recipient: HaneulAddress,
        amount: Option<u64>,
    ) -> anyhow::Result<TransactionBytes> {
        Ok(match &self.client {
            Client::Http(c) => c.transfer_haneul(signer, haneul_object_id, gas_budget, recipient, amount),
            Client::Ws(c) => c.transfer_haneul(signer, haneul_object_id, gas_budget, recipient, amount),
        }
        .await?)
    }

    pub async fn move_call(
        &self,
        signer: HaneulAddress,
        package_object_id: ObjectID,
        module: String,
        function: String,
        type_arguments: Vec<HaneulTypeTag>,
        arguments: Vec<HaneulJsonValue>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> anyhow::Result<TransactionBytes> {
        Ok(match &self.client {
            Client::Http(c) => c.move_call(
                signer,
                package_object_id,
                module,
                function,
                type_arguments,
                arguments,
                gas,
                gas_budget,
            ),
            Client::Ws(c) => c.move_call(
                signer,
                package_object_id,
                module,
                function,
                type_arguments,
                arguments,
                gas,
                gas_budget,
            ),
        }
        .await?)
    }

    pub async fn publish(
        &self,
        sender: HaneulAddress,
        compiled_modules: Vec<Base64>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> anyhow::Result<TransactionBytes> {
        Ok(match &self.client {
            Client::Http(c) => c.publish(sender, compiled_modules, gas, gas_budget),
            Client::Ws(c) => c.publish(sender, compiled_modules, gas, gas_budget),
        }
        .await?)
    }

    pub async fn split_coin(
        &self,
        signer: HaneulAddress,
        coin_object_id: ObjectID,
        split_amounts: Vec<u64>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> anyhow::Result<TransactionBytes> {
        Ok(match &self.client {
            Client::Http(c) => c.split_coin(signer, coin_object_id, split_amounts, gas, gas_budget),
            Client::Ws(c) => c.split_coin(signer, coin_object_id, split_amounts, gas, gas_budget),
        }
        .await?)
    }

    pub async fn merge_coin(
        &self,
        signer: HaneulAddress,
        primary_coin: ObjectID,
        coin_to_merge: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> anyhow::Result<TransactionBytes> {
        Ok(match &self.client {
            Client::Http(c) => c.merge_coin(signer, primary_coin, coin_to_merge, gas, gas_budget),
            Client::Ws(c) => c.merge_coin(signer, primary_coin, coin_to_merge, gas, gas_budget),
        }
        .await?)
    }

    pub async fn batch_transaction(
        &self,
        signer: HaneulAddress,
        single_transaction_params: Vec<RPCTransactionRequestParams>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> anyhow::Result<TransactionBytes> {
        Ok(match &self.client {
            Client::Http(c) => {
                c.batch_transaction(signer, single_transaction_params, gas, gas_budget)
            }
            Client::Ws(c) => {
                c.batch_transaction(signer, single_transaction_params, gas, gas_budget)
            }
        }
        .await?)
    }

    pub async fn sync_account_state(&self, address: HaneulAddress) -> anyhow::Result<()> {
        Ok(match &self.client {
            Client::Http(c) => c.sync_account_state(address),
            Client::Ws(c) => c.sync_account_state(address),
        }
        .await?)
    }

    pub async fn subscribe_event(
        &self,
        filter: HaneulEventFilter,
    ) -> anyhow::Result<impl Stream<Item = Result<HaneulEventEnvelope, anyhow::Error>>> {
        match &self.client {
            Client::Ws(c) => {
                let subscription: Subscription<HaneulEventEnvelope> =
                    c.subscribe_event(filter).await?;
                Ok(subscription.map(|item| Ok(item?)))
            }
            _ => Err(anyhow!(
                "Subscription only supported with web socket client."
            )),
        }
    }
}
#[allow(clippy::large_enum_variant)]
enum Client {
    Http(HttpClient),
    Ws(WsClient),
}
