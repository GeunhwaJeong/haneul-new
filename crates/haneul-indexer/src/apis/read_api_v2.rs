// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// TODO remove after the functions are implemented
#![allow(unused_variables)]
#![allow(dead_code)]

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use crate::store::PgIndexerStoreV2;
use haneul_json_rpc::api::ReadApiServer;
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_types::{
    Checkpoint, CheckpointId, CheckpointPage, ProtocolConfigResponse, HaneulEvent,
    HaneulGetPastObjectRequest, HaneulObjectDataOptions, HaneulObjectResponse, HaneulPastObjectResponse,
    HaneulTransactionBlockResponse, HaneulTransactionBlockResponseOptions,
};
use haneul_open_rpc::Module;
use haneul_types::base_types::{ObjectID, SequenceNumber};
use haneul_types::digests::TransactionDigest;
use haneul_types::haneul_serde::BigInt;

use haneul_json_rpc_types::HaneulLoadedChildObjectsResponse;

pub(crate) struct ReadApiV2 {
    pg_store: PgIndexerStoreV2,
}

impl ReadApiV2 {
    pub fn new(pg_store: PgIndexerStoreV2) -> Self {
        Self { pg_store }
    }
}

#[async_trait]
impl ReadApiServer for ReadApiV2 {
    async fn get_object(
        &self,
        object_id: ObjectID,
        options: Option<HaneulObjectDataOptions>,
    ) -> RpcResult<HaneulObjectResponse> {
        unimplemented!()
    }

    async fn multi_get_objects(
        &self,
        object_ids: Vec<ObjectID>,
        options: Option<HaneulObjectDataOptions>,
    ) -> RpcResult<Vec<HaneulObjectResponse>> {
        unimplemented!()
    }

    async fn get_total_transaction_blocks(&self) -> RpcResult<BigInt<u64>> {
        unimplemented!()
    }

    async fn get_transaction_block(
        &self,
        digest: TransactionDigest,
        options: Option<HaneulTransactionBlockResponseOptions>,
    ) -> RpcResult<HaneulTransactionBlockResponse> {
        unimplemented!()
    }

    async fn multi_get_transaction_blocks(
        &self,
        digests: Vec<TransactionDigest>,
        options: Option<HaneulTransactionBlockResponseOptions>,
    ) -> RpcResult<Vec<HaneulTransactionBlockResponse>> {
        unimplemented!()
    }

    async fn try_get_past_object(
        &self,
        object_id: ObjectID,
        version: SequenceNumber,
        options: Option<HaneulObjectDataOptions>,
    ) -> RpcResult<HaneulPastObjectResponse> {
        unimplemented!()
    }

    async fn try_multi_get_past_objects(
        &self,
        past_objects: Vec<HaneulGetPastObjectRequest>,
        options: Option<HaneulObjectDataOptions>,
    ) -> RpcResult<Vec<HaneulPastObjectResponse>> {
        unimplemented!()
    }

    async fn get_latest_checkpoint_sequence_number(&self) -> RpcResult<BigInt<u64>> {
        unimplemented!()
    }

    async fn get_checkpoint(&self, id: CheckpointId) -> RpcResult<Checkpoint> {
        unimplemented!()
    }

    async fn get_checkpoints(
        &self,
        cursor: Option<BigInt<u64>>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        unimplemented!()
    }

    async fn get_checkpoints_deprecated_limit(
        &self,
        cursor: Option<BigInt<u64>>,
        limit: Option<BigInt<u64>>,
        descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        unimplemented!()
    }

    async fn get_events(&self, transaction_digest: TransactionDigest) -> RpcResult<Vec<HaneulEvent>> {
        unimplemented!()
    }

    async fn get_loaded_child_objects(
        &self,
        digest: TransactionDigest,
    ) -> RpcResult<HaneulLoadedChildObjectsResponse> {
        unimplemented!()
    }

    async fn get_protocol_config(
        &self,
        version: Option<BigInt<u64>>,
    ) -> RpcResult<ProtocolConfigResponse> {
        unimplemented!()
    }

    async fn get_chain_identifier(&self) -> RpcResult<String> {
        unimplemented!()
    }
}

impl HaneulRpcModule for ReadApiV2 {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc::api::ReadApiOpenRpc::module_doc()
    }
}
