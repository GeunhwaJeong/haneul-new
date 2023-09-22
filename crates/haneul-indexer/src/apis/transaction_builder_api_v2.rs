// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// TODO remove after the functions are implemented
#![allow(unused_variables)]
#![allow(dead_code)]

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use haneul_json::HaneulJsonValue;
use haneul_json_rpc::api::TransactionBuilderServer;
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_types::{
    RPCTransactionRequestParams, HaneulTransactionBlockBuilderMode, HaneulTypeTag, TransactionBlockBytes,
};
use haneul_open_rpc::Module;
use haneul_types::base_types::{ObjectID, HaneulAddress};
use haneul_types::haneul_serde::BigInt;

use crate::store::PgIndexerStoreV2;

pub(crate) struct TransactionBuilderApiV2 {
    pg_store: PgIndexerStoreV2,
}

impl TransactionBuilderApiV2 {
    pub fn new(pg_store: PgIndexerStoreV2) -> Self {
        Self { pg_store }
    }
}

#[async_trait]
impl TransactionBuilderServer for TransactionBuilderApiV2 {
    async fn transfer_object(
        &self,
        signer: HaneulAddress,
        object_id: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
        recipient: HaneulAddress,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn transfer_haneul(
        &self,
        signer: HaneulAddress,
        haneul_object_id: ObjectID,
        gas_budget: BigInt<u64>,
        recipient: HaneulAddress,
        amount: Option<BigInt<u64>>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
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
        unimplemented!()
    }

    async fn pay_haneul(
        &self,
        signer: HaneulAddress,
        input_coins: Vec<ObjectID>,
        recipients: Vec<HaneulAddress>,
        amounts: Vec<BigInt<u64>>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn pay_all_haneul(
        &self,
        signer: HaneulAddress,
        input_coins: Vec<ObjectID>,
        recipient: HaneulAddress,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn publish(
        &self,
        sender: HaneulAddress,
        compiled_modules: Vec<Base64>,
        dep_ids: Vec<ObjectID>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn split_coin(
        &self,
        signer: HaneulAddress,
        coin_object_id: ObjectID,
        split_amounts: Vec<BigInt<u64>>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn split_coin_equal(
        &self,
        signer: HaneulAddress,
        coin_object_id: ObjectID,
        split_count: BigInt<u64>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn merge_coin(
        &self,
        signer: HaneulAddress,
        primary_coin: ObjectID,
        coin_to_merge: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
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
        unimplemented!()
    }

    async fn batch_transaction(
        &self,
        signer: HaneulAddress,
        params: Vec<RPCTransactionRequestParams>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
        tx_builder_mode: Option<HaneulTransactionBlockBuilderMode>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
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
        unimplemented!()
    }

    async fn request_withdraw_stake(
        &self,
        signer: HaneulAddress,
        staked_haneul: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }
}

impl HaneulRpcModule for TransactionBuilderApiV2 {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc::api::TransactionBuilderOpenRpc::module_doc()
    }
}
