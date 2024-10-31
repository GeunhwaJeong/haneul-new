// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
mod bigtable;
use anyhow::Result;
use async_trait::async_trait;
pub use bigtable::client::BigTableClient;
pub use bigtable::worker::KvWorker;
use haneul_types::crypto::AuthorityStrongQuorumSignInfo;
use haneul_types::digests::{CheckpointDigest, TransactionDigest};
use haneul_types::effects::{TransactionEffects, TransactionEvents};
use haneul_types::full_checkpoint_content::CheckpointData;
use haneul_types::messages_checkpoint::{
    CheckpointContents, CheckpointSequenceNumber, CheckpointSummary,
};
use haneul_types::object::Object;
use haneul_types::storage::ObjectKey;
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
}

#[async_trait]
pub trait KeyValueStoreWriter {
    async fn save_objects(&mut self, objects: &[&Object]) -> Result<()>;
    async fn save_transactions(&mut self, transactions: &[TransactionData]) -> Result<()>;
    async fn save_checkpoint(&mut self, checkpoint: &CheckpointData) -> Result<()>;
}

#[derive(Clone, Debug)]
pub struct Checkpoint {
    pub summary: CheckpointSummary,
    pub contents: CheckpointContents,
    pub signatures: AuthorityStrongQuorumSignInfo,
}

#[derive(Clone, Debug)]
pub struct TransactionData {
    pub transaction: Transaction,
    pub effects: TransactionEffects,
    pub events: Option<TransactionEvents>,
    pub checkpoint_number: CheckpointSequenceNumber,
    pub timestamp: u64,
}
