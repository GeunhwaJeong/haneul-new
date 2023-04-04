// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use futures::future::join_all;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use haneul_json_rpc::api::{ReadApiClient, ReadApiServer};
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_types::{
    BigInt, Checkpoint, CheckpointId, CheckpointPage, HaneulCheckpointSequenceNumber, HaneulEvent,
    HaneulGetPastObjectRequest, HaneulObjectDataOptions, HaneulObjectResponse, HaneulPastObjectResponse,
    HaneulTransactionBlockResponse, HaneulTransactionBlockResponseOptions,
};
use haneul_open_rpc::Module;
use haneul_types::base_types::{ObjectID, SequenceNumber};
use haneul_types::digests::TransactionDigest;

use crate::errors::IndexerError;
use crate::store::IndexerStore;
use crate::types::HaneulTransactionBlockResponseWithOptions;

pub(crate) struct ReadApi<S> {
    fullnode: HttpClient,
    state: S,
    migrated_methods: Vec<String>,
}

impl<S: IndexerStore> ReadApi<S> {
    pub fn new(state: S, fullnode_client: HttpClient, migrated_methods: Vec<String>) -> Self {
        Self {
            state,
            fullnode: fullnode_client,
            migrated_methods,
        }
    }

    async fn get_total_transaction_blocks_internal(&self) -> Result<u64, IndexerError> {
        self.state
            .get_total_transaction_number_from_checkpoints()
            .await
            .map(|n| n as u64)
    }

    async fn get_transaction_block_internal(
        &self,
        digest: &TransactionDigest,
        options: Option<HaneulTransactionBlockResponseOptions>,
    ) -> Result<HaneulTransactionBlockResponse, IndexerError> {
        let tx = self
            .state
            .get_transaction_by_digest(&digest.base58_encode())
            .await?;
        let haneul_tx_resp = self
            .state
            .compose_haneul_transaction_block_response(tx, options.as_ref())
            .await?;
        let haneul_transaction_response = HaneulTransactionBlockResponseWithOptions {
            response: haneul_tx_resp,
            options: options.unwrap_or_default(),
        }
        .into();
        Ok(haneul_transaction_response)
    }

    async fn multi_get_transaction_blocks_internal(
        &self,
        digests: &[TransactionDigest],
        options: Option<HaneulTransactionBlockResponseOptions>,
    ) -> Result<Vec<HaneulTransactionBlockResponse>, IndexerError> {
        let digest_strs = digests
            .iter()
            .map(|digest| digest.base58_encode())
            .collect::<Vec<_>>();
        let tx_vec = self
            .state
            .multi_get_transactions_by_digests(&digest_strs)
            .await?;
        let ordered_tx_vec = digest_strs
            .iter()
            .filter_map(|digest| {
                tx_vec
                    .iter()
                    .find(|tx| tx.transaction_digest == *digest)
                    .cloned()
            })
            .collect::<Vec<_>>();
        if ordered_tx_vec.len() != tx_vec.len() {
            return Err(IndexerError::PostgresReadError(
                "Transaction count changed after reorder, this should never happen.".to_string(),
            ));
        }
        let haneul_tx_resp_futures = ordered_tx_vec.into_iter().map(|tx| {
            self.state
                .compose_haneul_transaction_block_response(tx, options.as_ref())
        });
        let haneul_tx_resp_vec = join_all(haneul_tx_resp_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(haneul_tx_resp_vec)
    }

    async fn get_object_internal(
        &self,
        object_id: ObjectID,
        options: Option<HaneulObjectDataOptions>,
    ) -> Result<HaneulObjectResponse, IndexerError> {
        let read = self.state.get_object(object_id, None).await?;
        Ok((read, options.unwrap_or_default()).try_into()?)
    }

    async fn get_latest_checkpoint_sequence_number_internal(&self) -> Result<u64, IndexerError> {
        self.state
            .get_latest_checkpoint_sequence_number()
            .await
            .map(|n| n as u64)
    }
}

#[async_trait]
impl<S> ReadApiServer for ReadApi<S>
where
    S: IndexerStore + Sync + Send + 'static,
{
    async fn get_object(
        &self,
        object_id: ObjectID,
        options: Option<HaneulObjectDataOptions>,
    ) -> RpcResult<HaneulObjectResponse> {
        if !self.migrated_methods.contains(&"get_object".into()) {
            return self.fullnode.get_object(object_id, options).await;
        }

        Ok(self.get_object_internal(object_id, options).await?)
    }

    async fn multi_get_objects(
        &self,
        object_ids: Vec<ObjectID>,
        options: Option<HaneulObjectDataOptions>,
    ) -> RpcResult<Vec<HaneulObjectResponse>> {
        return self.fullnode.multi_get_objects(object_ids, options).await;
    }

    async fn get_total_transaction_blocks(&self) -> RpcResult<BigInt> {
        if !self
            .migrated_methods
            .contains(&"get_total_transaction_blocks".to_string())
        {
            return self.fullnode.get_total_transaction_blocks().await;
        }
        Ok(self.get_total_transaction_blocks_internal().await?.into())
    }

    async fn get_transaction_block(
        &self,
        digest: TransactionDigest,
        options: Option<HaneulTransactionBlockResponseOptions>,
    ) -> RpcResult<HaneulTransactionBlockResponse> {
        if !self
            .migrated_methods
            .contains(&"get_transaction_block".to_string())
        {
            return self.fullnode.get_transaction_block(digest, options).await;
        }
        Ok(self
            .get_transaction_block_internal(&digest, options)
            .await?)
    }

    async fn multi_get_transaction_blocks(
        &self,
        digests: Vec<TransactionDigest>,
        options: Option<HaneulTransactionBlockResponseOptions>,
    ) -> RpcResult<Vec<HaneulTransactionBlockResponse>> {
        if !self
            .migrated_methods
            .contains(&"multi_get_transaction_blocks".to_string())
        {
            return self
                .fullnode
                .multi_get_transaction_blocks(digests, options)
                .await;
        }
        Ok(self
            .multi_get_transaction_blocks_internal(&digests, options)
            .await?)
    }

    async fn try_get_past_object(
        &self,
        object_id: ObjectID,
        version: SequenceNumber,
        options: Option<HaneulObjectDataOptions>,
    ) -> RpcResult<HaneulPastObjectResponse> {
        self.fullnode
            .try_get_past_object(object_id, version, options)
            .await
    }

    async fn try_multi_get_past_objects(
        &self,
        past_objects: Vec<HaneulGetPastObjectRequest>,
        options: Option<HaneulObjectDataOptions>,
    ) -> RpcResult<Vec<HaneulPastObjectResponse>> {
        self.fullnode
            .try_multi_get_past_objects(past_objects, options)
            .await
    }

    async fn get_latest_checkpoint_sequence_number(
        &self,
    ) -> RpcResult<HaneulCheckpointSequenceNumber> {
        if !self
            .migrated_methods
            .contains(&"get_latest_checkpoint_sequence_number".to_string())
        {
            return self.fullnode.get_latest_checkpoint_sequence_number().await;
        }
        Ok(self
            .get_latest_checkpoint_sequence_number_internal()
            .await?
            .into())
    }

    async fn get_checkpoint(&self, id: CheckpointId) -> RpcResult<Checkpoint> {
        if !self
            .migrated_methods
            .contains(&"get_checkpoint".to_string())
        {
            return self.fullnode.get_checkpoint(id).await;
        }
        Ok(self.state.get_checkpoint(id).await?)
    }

    async fn get_checkpoints(
        &self,
        cursor: Option<HaneulCheckpointSequenceNumber>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        return self
            .fullnode
            .get_checkpoints(cursor, limit, descending_order)
            .await;
    }

    async fn get_events(&self, transaction_digest: TransactionDigest) -> RpcResult<Vec<HaneulEvent>> {
        self.fullnode.get_events(transaction_digest).await
    }
}

impl<S> HaneulRpcModule for ReadApi<S>
where
    S: IndexerStore + Sync + Send + 'static,
{
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc::api::ReadApiOpenRpc::module_doc()
    }
}
