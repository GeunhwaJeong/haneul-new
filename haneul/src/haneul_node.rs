// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use crate::{
    api::{RpcGatewayServer, TransactionBytes},
    rpc_gateway::responses::{ObjectResponse, HaneulTypeTag},
};
use anyhow::anyhow;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use haneul_core::gateway_types::{TransactionEffectsResponse, TransactionResponse};

use haneul_core::gateway_state::GatewayTxSeqNumber;
use haneul_core::gateway_types::GetObjectInfoResponse;
use haneul_core::haneul_json::HaneulJsonValue;
use haneul_types::base_types::{ObjectID, HaneulAddress, TransactionDigest};
use haneul_types::haneul_serde::Base64;

pub struct HaneulNode {}

impl HaneulNode {
    pub fn new(_config_path: &Path) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl RpcGatewayServer for HaneulNode {
    async fn transfer_coin(
        &self,
        _signer: HaneulAddress,
        _object_id: ObjectID,
        _gas: Option<ObjectID>,
        _gas_budget: u64,
        _recipient: HaneulAddress,
    ) -> RpcResult<TransactionBytes> {
        Err(anyhow!("Haneul Node only supports read-only methods").into())
    }

    async fn publish(
        &self,
        _sender: HaneulAddress,
        _compiled_modules: Vec<Base64>,
        _gas: Option<ObjectID>,
        _gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        Err(anyhow!("Haneul Node only supports read-only methods").into())
    }

    async fn split_coin(
        &self,
        _signer: HaneulAddress,
        _coin_object_id: ObjectID,
        _split_amounts: Vec<u64>,
        _gas: Option<ObjectID>,
        _gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        Err(anyhow!("Haneul Node only supports read-only methods").into())
    }

    async fn merge_coin(
        &self,
        _signer: HaneulAddress,
        _primary_coin: ObjectID,
        _coin_to_merge: ObjectID,
        _gas: Option<ObjectID>,
        _gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        Err(anyhow!("Haneul Node only supports read-only methods").into())
    }

    async fn get_owned_objects(&self, _owner: HaneulAddress) -> RpcResult<ObjectResponse> {
        todo!()
    }

    async fn get_object_info(&self, _object_id: ObjectID) -> RpcResult<GetObjectInfoResponse> {
        todo!()
    }

    async fn execute_transaction(
        &self,
        _tx_bytes: Base64,
        _signature: Base64,
        _pub_key: Base64,
    ) -> RpcResult<TransactionResponse> {
        Err(anyhow!("Haneul Node only supports read-only methods").into())
    }

    async fn move_call(
        &self,
        _signer: HaneulAddress,
        _package_object_id: ObjectID,
        _module: String,
        _function: String,
        _type_arguments: Vec<HaneulTypeTag>,
        _rpc_arguments: Vec<HaneulJsonValue>,
        _gas: Option<ObjectID>,
        _gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        Err(anyhow!("Haneul Node only supports read-only methods").into())
    }

    async fn sync_account_state(&self, _address: HaneulAddress) -> RpcResult<()> {
        todo!()
    }

    async fn get_total_transaction_number(&self) -> RpcResult<u64> {
        todo!()
    }

    async fn get_transactions_in_range(
        &self,
        _start: GatewayTxSeqNumber,
        _end: GatewayTxSeqNumber,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        todo!()
    }

    async fn get_recent_transactions(
        &self,
        _count: u64,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        todo!()
    }

    async fn get_transaction(
        &self,
        _digest: TransactionDigest,
    ) -> RpcResult<TransactionEffectsResponse> {
        todo!()
    }
}
