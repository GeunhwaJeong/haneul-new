// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee_proc_macros::rpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use haneul_core::gateway_state::GatewayTxSeqNumber;
use haneul_core::gateway_types::{GetObjectInfoResponse, HaneulInputObjectKind, HaneulObjectRef};
use haneul_core::gateway_types::{TransactionEffectsResponse, TransactionResponse};
use haneul_core::haneul_json::HaneulJsonValue;
use haneul_open_rpc_macros::open_rpc;
use haneul_types::haneul_serde::Base64;
use haneul_types::{
    base_types::{ObjectID, HaneulAddress, TransactionDigest},
    crypto::SignableBytes,
    messages::TransactionData,
};

use crate::rpc_gateway::responses::ObjectResponse;
use crate::rpc_gateway::responses::HaneulTypeTag;

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
        module: String,
        function: String,
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
        tx_bytes: Base64,
        signature: Base64,
        pub_key: Base64,
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

    #[method(name = "getTransactionsByInputObject")]
    async fn get_transactions_by_input_object(
        &self,
        object: ObjectID,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>>;

    #[method(name = "getTransactionsByMutatedObject")]
    async fn get_transactions_by_mutated_object(
        &self,
        object: ObjectID,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>>;

    #[method(name = "getTransactionsFromAddress")]
    async fn get_transactions_from_addr(
        &self,
        addr: HaneulAddress,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>>;

    #[method(name = "getTransactionsToAddress")]
    async fn get_transactions_to_addr(
        &self,
        addr: HaneulAddress,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>>;

    /// Return the object information for a specified object
    #[method(name = "getObjectInfo")]
    async fn get_object_info(&self, object_id: ObjectID) -> RpcResult<GetObjectInfoResponse>;
}

#[serde_as]
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBytes {
    pub tx_bytes: Base64,
    pub gas: HaneulObjectRef,
    pub input_objects: Vec<HaneulInputObjectKind>,
}

impl TransactionBytes {
    pub fn from_data(data: TransactionData) -> Result<Self, anyhow::Error> {
        Ok(Self {
            tx_bytes: Base64::from_bytes(&data.to_bytes()),
            gas: data.gas().into(),
            input_objects: data
                .input_objects()?
                .into_iter()
                .map(HaneulInputObjectKind::from)
                .collect(),
        })
    }

    pub fn to_data(self) -> Result<TransactionData, anyhow::Error> {
        TransactionData::from_signable_bytes(&self.tx_bytes.to_vec()?)
    }
}
