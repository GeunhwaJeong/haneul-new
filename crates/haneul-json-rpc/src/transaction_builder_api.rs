// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::api::TransactionBuilderServer;
use crate::HaneulRpcModule;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use std::sync::Arc;
use haneul_core::authority::AuthorityState;
use haneul_json_rpc_types::{
    GetRawObjectDataResponse, HaneulObjectInfo, HaneulTransactionBuilderMode, HaneulTypeTag,
    TransactionBytes,
};
use haneul_open_rpc::Module;
use haneul_transaction_builder::{DataReader, TransactionBuilder};
use haneul_types::{
    base_types::{ObjectID, HaneulAddress},
    messages::TransactionData,
};

use fastcrypto::encoding::Base64;
use jsonrpsee::RpcModule;
use haneul_adapter::execution_mode::{DevInspect, Normal};

use haneul_json::HaneulJsonValue;
use haneul_json_rpc_types::RPCTransactionRequestParams;

pub struct TransactionBuilderApi {
    builder: TransactionBuilder<Normal>,
    dev_inspect_builder: TransactionBuilder<DevInspect>,
}

impl TransactionBuilderApi {
    pub fn new(state: Arc<AuthorityState>) -> Self {
        let reader = Arc::new(AuthorityStateDataReader::new(state));
        Self {
            builder: TransactionBuilder::new(reader.clone()),
            dev_inspect_builder: TransactionBuilder::new(reader),
        }
    }
}

pub struct AuthorityStateDataReader(Arc<AuthorityState>);

impl AuthorityStateDataReader {
    pub fn new(state: Arc<AuthorityState>) -> Self {
        Self(state)
    }
}

#[async_trait]
impl DataReader for AuthorityStateDataReader {
    async fn get_objects_owned_by_address(
        &self,
        address: HaneulAddress,
    ) -> Result<Vec<HaneulObjectInfo>, anyhow::Error> {
        let refs: Vec<HaneulObjectInfo> = self
            .0
            .get_owner_objects(address)?
            .into_iter()
            .map(HaneulObjectInfo::from)
            .collect();
        Ok(refs)
    }

    async fn get_object(
        &self,
        object_id: ObjectID,
    ) -> Result<GetRawObjectDataResponse, anyhow::Error> {
        let result = self.0.get_object_read(&object_id).await?;
        Ok(result.try_into()?)
    }

    async fn get_reference_gas_price(&self) -> Result<u64, anyhow::Error> {
        let epoch_store = self.0.load_epoch_store_one_call_per_task();
        Ok(epoch_store.reference_gas_price())
    }
}

#[async_trait]
impl TransactionBuilderServer for TransactionBuilderApi {
    async fn transfer_object(
        &self,
        signer: HaneulAddress,
        object_id: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
        recipient: HaneulAddress,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .builder
            .transfer_object(signer, object_id, gas, gas_budget, recipient)
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
            .builder
            .transfer_haneul(signer, haneul_object_id, gas_budget, recipient, amount)
            .await?;
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn pay(
        &self,
        signer: HaneulAddress,
        input_coins: Vec<ObjectID>,
        recipients: Vec<HaneulAddress>,
        amounts: Vec<u64>,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .builder
            .pay(signer, input_coins, recipients, amounts, gas, gas_budget)
            .await?;
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn pay_haneul(
        &self,
        signer: HaneulAddress,
        input_coins: Vec<ObjectID>,
        recipients: Vec<HaneulAddress>,
        amounts: Vec<u64>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .builder
            .pay_haneul(signer, input_coins, recipients, amounts, gas_budget)
            .await?;
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn pay_all_haneul(
        &self,
        signer: HaneulAddress,
        input_coins: Vec<ObjectID>,
        recipient: HaneulAddress,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .builder
            .pay_all_haneul(signer, input_coins, recipient, gas_budget)
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
            .map(|data| data.to_vec().map_err(|e| anyhow::anyhow!(e)))
            .collect::<Result<Vec<_>, _>>()?;
        let data = self
            .builder
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
            .builder
            .split_coin(signer, coin_object_id, split_amounts, gas, gas_budget)
            .await?;
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn split_coin_equal(
        &self,
        signer: HaneulAddress,
        coin_object_id: ObjectID,
        split_count: u64,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        let data = self
            .builder
            .split_coin_equal(signer, coin_object_id, split_count, gas, gas_budget)
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
            .builder
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
        txn_builder_mode: Option<HaneulTransactionBuilderMode>,
    ) -> RpcResult<TransactionBytes> {
        let mode = txn_builder_mode.unwrap_or(HaneulTransactionBuilderMode::Commit);
        let data: TransactionData = match mode {
            HaneulTransactionBuilderMode::DevInspect => {
                self.dev_inspect_builder
                    .move_call(
                        signer,
                        package_object_id,
                        &module,
                        &function,
                        type_arguments,
                        rpc_arguments,
                        gas,
                        gas_budget,
                    )
                    .await?
            }
            HaneulTransactionBuilderMode::Commit => {
                self.builder
                    .move_call(
                        signer,
                        package_object_id,
                        &module,
                        &function,
                        type_arguments,
                        rpc_arguments,
                        gas,
                        gas_budget,
                    )
                    .await?
            }
        };
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn batch_transaction(
        &self,
        signer: HaneulAddress,
        params: Vec<RPCTransactionRequestParams>,
        gas: Option<ObjectID>,
        gas_budget: u64,
        txn_builder_mode: Option<HaneulTransactionBuilderMode>,
    ) -> RpcResult<TransactionBytes> {
        let mode = txn_builder_mode.unwrap_or(HaneulTransactionBuilderMode::Commit);
        let data = match mode {
            HaneulTransactionBuilderMode::DevInspect => {
                self.dev_inspect_builder
                    .batch_transaction(signer, params, gas, gas_budget)
                    .await?
            }
            HaneulTransactionBuilderMode::Commit => {
                self.builder
                    .batch_transaction(signer, params, gas, gas_budget)
                    .await?
            }
        };
        Ok(TransactionBytes::from_data(data)?)
    }

    async fn request_add_delegation(
        &self,
        signer: HaneulAddress,
        coins: Vec<ObjectID>,
        amount: Option<u64>,
        validator: HaneulAddress,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        Ok(TransactionBytes::from_data(
            self.builder
                .request_add_delegation(signer, coins, amount, validator, gas, gas_budget)
                .await?,
        )?)
    }

    async fn request_withdraw_delegation(
        &self,
        signer: HaneulAddress,
        delegation: ObjectID,
        staked_haneul: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        Ok(TransactionBytes::from_data(
            self.builder
                .request_withdraw_delegation(signer, delegation, staked_haneul, gas, gas_budget)
                .await?,
        )?)
    }
}

impl HaneulRpcModule for TransactionBuilderApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        crate::api::TransactionBuilderOpenRpc::module_doc()
    }
}
