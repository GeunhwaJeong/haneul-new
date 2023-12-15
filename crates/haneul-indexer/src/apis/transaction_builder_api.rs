// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use haneul_json::HaneulJsonValue;
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_api::{TransactionBuilderClient, TransactionBuilderServer};
use haneul_json_rpc_types::{
    RPCTransactionRequestParams, HaneulTransactionBlockBuilderMode, HaneulTypeTag, TransactionBlockBytes,
};
use haneul_open_rpc::Module;
use haneul_types::base_types::{ObjectID, HaneulAddress};
use haneul_types::haneul_serde::BigInt;

pub(crate) struct TransactionBuilderApi {
    fullnode: HttpClient,
}

impl TransactionBuilderApi {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self {
            fullnode: fullnode_client,
        }
    }
}

#[async_trait]
impl TransactionBuilderServer for TransactionBuilderApi {
    async fn transfer_object(
        &self,
        signer: HaneulAddress,
        object_id: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
        recipient: HaneulAddress,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .transfer_object(signer, object_id, gas, gas_budget, recipient)
            .await
    }

    async fn transfer_haneul(
        &self,
        signer: HaneulAddress,
        haneul_object_id: ObjectID,
        gas_budget: BigInt<u64>,
        recipient: HaneulAddress,
        amount: Option<BigInt<u64>>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .transfer_haneul(signer, haneul_object_id, gas_budget, recipient, amount)
            .await
    }

    async fn pay(
        &self,
        signer: HaneulAddress,
        input_coins: Vec<ObjectID>,
        recipients: Vec<HaneulAddress>,
        amounts: Vec<BigInt<u64>>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .pay(signer, input_coins, recipients, amounts, gas, gas_budget)
            .await
    }

    async fn pay_haneul(
        &self,
        signer: HaneulAddress,
        input_coins: Vec<ObjectID>,
        recipients: Vec<HaneulAddress>,
        amounts: Vec<BigInt<u64>>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .pay_haneul(signer, input_coins, recipients, amounts, gas_budget)
            .await
    }

    async fn pay_all_haneul(
        &self,
        signer: HaneulAddress,
        input_coins: Vec<ObjectID>,
        recipient: HaneulAddress,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .pay_all_haneul(signer, input_coins, recipient, gas_budget)
            .await
    }

    async fn publish(
        &self,
        sender: HaneulAddress,
        compiled_modules: Vec<Base64>,
        dep_ids: Vec<ObjectID>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .publish(sender, compiled_modules, dep_ids, gas, gas_budget)
            .await
    }

    async fn split_coin(
        &self,
        signer: HaneulAddress,
        coin_object_id: ObjectID,
        split_amounts: Vec<BigInt<u64>>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .split_coin(signer, coin_object_id, split_amounts, gas, gas_budget)
            .await
    }

    async fn split_coin_equal(
        &self,
        signer: HaneulAddress,
        coin_object_id: ObjectID,
        split_count: BigInt<u64>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .split_coin_equal(signer, coin_object_id, split_count, gas, gas_budget)
            .await
    }

    async fn merge_coin(
        &self,
        signer: HaneulAddress,
        primary_coin: ObjectID,
        coin_to_merge: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .merge_coin(signer, primary_coin, coin_to_merge, gas, gas_budget)
            .await
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
        gas_budget: BigInt<u64>,
        tx_builder_mode: Option<HaneulTransactionBlockBuilderMode>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .move_call(
                signer,
                package_object_id,
                module,
                function,
                type_arguments,
                rpc_arguments,
                gas,
                gas_budget,
                tx_builder_mode,
            )
            .await
    }

    async fn batch_transaction(
        &self,
        signer: HaneulAddress,
        params: Vec<RPCTransactionRequestParams>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
        tx_builder_mode: Option<HaneulTransactionBlockBuilderMode>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .batch_transaction(signer, params, gas, gas_budget, tx_builder_mode)
            .await
    }

    async fn request_add_stake(
        &self,
        signer: HaneulAddress,
        coins: Vec<ObjectID>,
        amount: Option<BigInt<u64>>,
        validator: HaneulAddress,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .request_add_stake(signer, coins, amount, validator, gas, gas_budget)
            .await
    }

    async fn request_withdraw_stake(
        &self,
        signer: HaneulAddress,
        staked_haneul: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .request_withdraw_stake(signer, staked_haneul, gas, gas_budget)
            .await
    }
}

impl HaneulRpcModule for TransactionBuilderApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc_api::TransactionBuilderOpenRpc::module_doc()
    }
}
