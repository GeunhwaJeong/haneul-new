// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
mod bigtable;
use anyhow::Result;
use async_trait::async_trait;
pub use bigtable::client::BigTableClient;
pub use bigtable::progress_store::BigTableProgressStore;
pub use bigtable::worker::KvWorker;
use serde::{Deserialize, Serialize};
use haneul_types::base_types::ObjectID;
use haneul_types::committee::EpochId;
use haneul_types::crypto::AuthorityStrongQuorumSignInfo;
use haneul_types::digests::{CheckpointDigest, TransactionDigest};
use haneul_types::effects::{TransactionEffects, TransactionEvents};
use haneul_types::full_checkpoint_content::CheckpointData;
use haneul_types::messages_checkpoint::{
    CheckpointContents, CheckpointSequenceNumber, CheckpointSummary,
};
use haneul_types::messages_consensus::TimestampMs;
use haneul_types::object::Object;
use haneul_types::storage::{EpochInfo, ObjectKey};
use haneul_types::transaction::Transaction;

#[async_trait]
pub trait KeyValueStoreReader {
    async fn get_objects(&mut self, objects: &[ObjectKey]) -> Result<Vec<Object>>;
    async fn get_transactions(
        &mut self,
        transactions: &[TransactionDigest],
    ) -> Result<Vec<TransactionData>>;
    async fn get_checkpoints(
        &mut self,
        sequence_numbers: &[CheckpointSequenceNumber],
    ) -> Result<Vec<Checkpoint>>;
    async fn get_checkpoint_by_digest(
        &mut self,
        digest: CheckpointDigest,
    ) -> Result<Option<Checkpoint>>;
    async fn get_latest_checkpoint(&mut self) -> Result<CheckpointSequenceNumber>;
    async fn get_latest_checkpoint_summary(&mut self) -> Result<Option<CheckpointSummary>>;
    async fn get_latest_object(&mut self, object_id: &ObjectID) -> Result<Option<Object>>;
    async fn get_epoch(&mut self, epoch_id: EpochId) -> Result<Option<EpochInfo>>;
    async fn get_latest_epoch(&mut self) -> Result<Option<EpochInfo>>;
}

#[async_trait]
pub trait KeyValueStoreWriter {
    async fn save_objects(&mut self, objects: &[&Object], timestamp_ms: TimestampMs) -> Result<()>;
    async fn save_transactions(&mut self, transactions: &[TransactionData]) -> Result<()>;
    async fn save_checkpoint(&mut self, checkpoint: &CheckpointData) -> Result<()>;
    async fn save_watermark(&mut self, watermark: CheckpointSequenceNumber) -> Result<()>;
    async fn save_epoch(&mut self, epoch: EpochInfo) -> Result<()>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checkpoint {
    pub summary: CheckpointSummary,
    pub contents: CheckpointContents,
    pub signatures: AuthorityStrongQuorumSignInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionData {
    pub transaction: Transaction,
    pub effects: TransactionEffects,
    pub events: Option<TransactionEvents>,
    pub checkpoint_number: CheckpointSequenceNumber,
    pub timestamp: u64,
}
