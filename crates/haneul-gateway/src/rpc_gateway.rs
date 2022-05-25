// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::sync::Arc;

use crate::api::{RpcGatewayApiServer, HaneulRpcModule};
use crate::rpc_gateway::responses::HaneulTypeTag;
use crate::{api::TransactionBytes, config::GatewayConfig, rpc_gateway::responses::ObjectResponse};
use anyhow::anyhow;
use async_trait::async_trait;
use ed25519_dalek::ed25519::signature::Signature;
use jsonrpsee::core::RpcResult;
use jsonrpsee_core::server::rpc_module::RpcModule;
use tracing::debug;

use haneul_config::PersistedConfig;
use haneul_core::gateway_state::{GatewayClient, GatewayState, GatewayTxSeqNumber};
use haneul_core::gateway_types::GetObjectInfoResponse;
use haneul_core::gateway_types::{TransactionEffectsResponse, TransactionResponse};
use haneul_json::HaneulJsonValue;
use haneul_open_rpc::Module;
use haneul_types::haneul_serde::Base64;
use haneul_types::{
    base_types::{ObjectID, HaneulAddress, TransactionDigest},
    crypto,
    crypto::SignableBytes,
    messages::{Transaction, TransactionData},
};

use crate::api::RpcReadApiServer;
use crate::api::RpcTransactionBuilderServer;

pub mod responses;

pub struct RpcGatewayImpl {
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

pub fn create_client(config_path: &Path) -> Result<GatewayClient, anyhow::Error> {
    let config: GatewayConfig = PersistedConfig::read(config_path).map_err(|e| {
        anyhow!(
            "Failed to read config file at {:?}: {}. Have you run `haneul genesis` first?",
            config_path,
            e
        )
    })?;
    let committee = config.make_committee();
    let authority_clients = config.make_authority_clients();
    Ok(Arc::new(GatewayState::new(
        config.db_folder_path,
        committee,
        authority_clients,
    )?))
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

    async fn sync_account_state(&self, address: HaneulAddress) -> RpcResult<()> {
        debug!("sync_account_state : {}", address);
        self.client.sync_account_state(address).await?;
        Ok(())
    }
}

impl HaneulRpcModule for RpcGatewayImpl {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        crate::api::RpcGatewayApiOpenRpc::module_doc()
    }
}

#[async_trait]
impl RpcReadApiServer for GatewayReadApiImpl {
    async fn get_owned_objects(&self, owner: HaneulAddress) -> RpcResult<ObjectResponse> {
        debug!("get_objects : {}", owner);
        let objects = self.client.get_owned_objects(owner).await?;
        Ok(ObjectResponse { objects })
    }

    async fn get_object_info(&self, object_id: ObjectID) -> RpcResult<GetObjectInfoResponse> {
        Ok(self.client.get_object_info(object_id).await?)
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
        crate::api::RpcReadApiOpenRpc::module_doc()
    }
}

#[async_trait]
impl RpcTransactionBuilderServer for TransactionBuilderImpl {
    async fn transfer_coin(
        &self,
        signer: HaneulAddress,
        object_id: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
        recipient: HaneulAddress,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .client
            .transfer_coin(signer, object_id, gas, gas_budget, recipient)
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
                    type_arguments
                        .into_iter()
                        .map(|tag| tag.try_into())
                        .collect::<Result<Vec<_>, _>>()?,
                    rpc_arguments,
                    gas,
                    gas_budget,
                )
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
        crate::api::RpcTransactionBuilderOpenRpc::module_doc()
    }
}
