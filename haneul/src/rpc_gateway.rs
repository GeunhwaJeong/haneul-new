// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use anyhow::anyhow;
use async_trait::async_trait;
use ed25519_dalek::ed25519::signature::Signature;
use jsonrpsee::core::RpcResult;
use jsonrpsee_proc_macros::rpc;
use move_core_types::identifier::Identifier;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{base64, serde_as};
use tracing::debug;

use haneul_core::gateway_state::{
    gateway_responses::{TransactionEffectsResponse, TransactionResponse},
    GatewayClient, GatewayState, GatewayTxSeqNumber,
};
use haneul_core::haneul_json::HaneulJsonValue;
use haneul_open_rpc_macros::open_rpc;
use haneul_types::base_types::ObjectRef;
use haneul_types::messages::InputObjectKind;
use haneul_types::{
    base_types::{ObjectID, HaneulAddress, TransactionDigest},
    crypto,
    crypto::SignableBytes,
    json_schema,
    json_schema::Base64,
    messages::{Transaction, TransactionData},
    object::ObjectRead,
};

use crate::rpc_gateway::responses::HaneulTypeTag;
use crate::{
    config::{GatewayConfig, PersistedConfig},
    rpc_gateway::responses::{GetObjectInfoResponse, NamedObjectRef, ObjectResponse},
};

pub mod responses;

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum RpcCallArg {
    Pure(json_schema::Base64),
    ImmOrOwnedObject(ObjectID),
    SharedObject(ObjectID),
}

#[open_rpc(
    name = "Haneul JSON-RPC",
    namespace = "haneul",
    contact_name = "Haneul Labs",
    contact_url = "https://haneul-labs.com",
    contact_email = "build@haneul-labs.com",
    license = "Apache-2.0",
    license_url = "https://raw.githubusercontent.com/HaneulLabs/haneul/main/LICENSE",
    description = "Haneul JSON-RPC API for interaction with the Haneul network gateway."
)]
#[rpc(server, client, namespace = "haneul")]
pub trait RpcGateway {
    /// Return the object information for a specified object
    #[method(name = "getObjectTypedInfo")]
    async fn get_object_typed_info(&self, object_id: ObjectID) -> RpcResult<GetObjectInfoResponse>;

    /// Create a transaction to transfer a Haneul coin from one address to another.
    #[method(name = "transferCoin")]
    async fn transfer_coin(
        &self,
        signer: HaneulAddress,
        object_id: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
        recipient: HaneulAddress,
    ) -> RpcResult<TransactionBytes>;

    /// Execute a Move call transaction by calling the specified function in the module of a given package.
    #[method(name = "moveCall")]
    async fn move_call(
        &self,
        signer: HaneulAddress,
        package_object_id: ObjectID,
        #[schemars(with = "json_schema::Identifier")] module: Identifier,
        #[schemars(with = "json_schema::Identifier")] function: Identifier,
        type_arguments: Vec<HaneulTypeTag>,
        arguments: Vec<HaneulJsonValue>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes>;

    /// Publish Move module.
    #[method(name = "publish")]
    async fn publish(
        &self,
        sender: HaneulAddress,
        compiled_modules: Vec<Base64>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes>;

    #[method(name = "splitCoin")]
    async fn split_coin(
        &self,
        signer: HaneulAddress,
        coin_object_id: ObjectID,
        split_amounts: Vec<u64>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes>;

    #[method(name = "mergeCoins")]
    async fn merge_coin(
        &self,
        signer: HaneulAddress,
        primary_coin: ObjectID,
        coin_to_merge: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes>;

    /// Execute the transaction using the transaction data, signature and public key.
    #[method(name = "executeTransaction")]
    async fn execute_transaction(
        &self,
        signed_transaction: SignedTransaction,
    ) -> RpcResult<TransactionResponse>;

    /// Synchronize client state with validators.
    #[method(name = "syncAccountState")]
    async fn sync_account_state(&self, address: HaneulAddress) -> RpcResult<()>;

    /// Return the list of objects owned by an address.
    #[method(name = "getOwnedObjects")]
    async fn get_owned_objects(&self, owner: HaneulAddress) -> RpcResult<ObjectResponse>;

    #[method(name = "getTotalTransactionNumber")]
    async fn get_total_transaction_number(&self) -> RpcResult<u64>;

    #[method(name = "getTransactionsInRange")]
    async fn get_transactions_in_range(
        &self,
        start: GatewayTxSeqNumber,
        end: GatewayTxSeqNumber,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>>;

    #[method(name = "getRecentTransactions")]
    async fn get_recent_transactions(
        &self,
        count: u64,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>>;

    #[method(name = "getTransaction")]
    async fn get_transaction(
        &self,
        digest: TransactionDigest,
    ) -> RpcResult<TransactionEffectsResponse>;

    /// Low level API to get object info. Client Applications should prefer to use
    /// `get_object_typed_info` instead.
    #[method(name = "getObjectInfoRaw")]
    async fn get_object_info(&self, object_id: ObjectID) -> RpcResult<ObjectRead>;
}

pub struct RpcGatewayImpl {
    gateway: GatewayClient,
}

impl RpcGatewayImpl {
    pub fn new(config_path: &Path) -> anyhow::Result<Self> {
        let config: GatewayConfig = PersistedConfig::read(config_path).map_err(|e| {
            anyhow!(
                "Failed to read config file at {:?}: {}. Have you run `haneul genesis` first?",
                config_path,
                e
            )
        })?;
        let committee = config.make_committee();
        let authority_clients = config.make_authority_clients();
        let gateway = Box::new(GatewayState::new(
            config.db_folder_path,
            committee,
            authority_clients,
        )?);
        Ok(Self { gateway })
    }
}

#[async_trait]
impl RpcGatewayServer for RpcGatewayImpl {
    async fn transfer_coin(
        &self,
        signer: HaneulAddress,
        object_id: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
        recipient: HaneulAddress,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .gateway
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
            .collect::<Vec<_>>();
        let data = self
            .gateway
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
            .gateway
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
            .gateway
            .merge_coins(signer, primary_coin, coin_to_merge, gas, gas_budget)
            .await?;
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn get_owned_objects(&self, owner: HaneulAddress) -> RpcResult<ObjectResponse> {
        debug!("get_objects : {}", owner);
        let objects = self
            .gateway
            .get_owned_objects(owner)
            .await?
            .into_iter()
            .map(NamedObjectRef::from)
            .collect();
        Ok(ObjectResponse { objects })
    }

    async fn get_object_info(&self, object_id: ObjectID) -> RpcResult<ObjectRead> {
        Ok(self.gateway.get_object_info(object_id).await?)
    }

    async fn get_object_typed_info(&self, object_id: ObjectID) -> RpcResult<GetObjectInfoResponse> {
        Ok(self
            .gateway
            .get_object_info(object_id)
            .await?
            .try_into()
            .map_err(|e| anyhow!("{}", e))?)
    }

    async fn execute_transaction(
        &self,
        signed_tx: SignedTransaction,
    ) -> RpcResult<TransactionResponse> {
        let data = TransactionData::from_signable_bytes(&signed_tx.tx_bytes)?;
        let signature =
            crypto::Signature::from_bytes(&[&*signed_tx.signature, &*signed_tx.pub_key].concat())
                .map_err(|e| anyhow!(e))?;
        Ok(self
            .gateway
            .execute_transaction(Transaction::new(data, signature))
            .await?)
    }

    async fn move_call(
        &self,
        signer: HaneulAddress,
        package_object_id: ObjectID,
        module: Identifier,
        function: Identifier,
        type_arguments: Vec<HaneulTypeTag>,
        rpc_arguments: Vec<HaneulJsonValue>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        let data = async {
            self.gateway
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

    async fn sync_account_state(&self, address: HaneulAddress) -> RpcResult<()> {
        debug!("sync_account_state : {}", address);
        self.gateway.sync_account_state(address).await?;
        Ok(())
    }

    async fn get_total_transaction_number(&self) -> RpcResult<u64> {
        Ok(self.gateway.get_total_transaction_number()?)
    }

    async fn get_transactions_in_range(
        &self,
        start: GatewayTxSeqNumber,
        end: GatewayTxSeqNumber,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(self.gateway.get_transactions_in_range(start, end)?)
    }

    async fn get_recent_transactions(
        &self,
        count: u64,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        Ok(self.gateway.get_recent_transactions(count)?)
    }

    async fn get_transaction(
        &self,
        digest: TransactionDigest,
    ) -> RpcResult<TransactionEffectsResponse> {
        Ok(self.gateway.get_transaction(digest).await?)
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SignedTransaction {
    #[schemars(with = "json_schema::Base64")]
    #[serde_as(as = "base64::Base64")]
    pub tx_bytes: Vec<u8>,
    #[schemars(with = "json_schema::Base64")]
    #[serde_as(as = "base64::Base64")]
    pub signature: Vec<u8>,
    #[schemars(with = "json_schema::Base64")]
    #[serde_as(as = "base64::Base64")]
    pub pub_key: Vec<u8>,
}

impl SignedTransaction {
    pub fn new(tx_bytes: Vec<u8>, signature: crypto::Signature) -> Self {
        Self {
            tx_bytes,
            signature: signature.signature_bytes().to_vec(),
            pub_key: signature.public_key_bytes().to_vec(),
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct TransactionBytes {
    #[schemars(with = "json_schema::Base64")]
    #[serde_as(as = "base64::Base64")]
    pub tx_bytes: Vec<u8>,
    pub gas: ObjectRef,
    pub input_objects: Vec<InputObjectKind>,
}

impl TransactionBytes {
    pub fn from_data(data: TransactionData) -> Result<Self, anyhow::Error> {
        Ok(Self {
            tx_bytes: data.to_bytes(),
            gas: data.gas(),
            input_objects: data.input_objects()?,
        })
    }

    pub fn to_data(self) -> Result<TransactionData, anyhow::Error> {
        TransactionData::from_signable_bytes(&self.tx_bytes)
    }
}
