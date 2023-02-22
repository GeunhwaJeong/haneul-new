// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use haneul_types::base_types::TransactionDigest;
use haneul_types::committee::Committee;
use haneul_types::committee::EpochId;
use haneul_types::digests::TransactionEffectsDigest;
use haneul_types::messages::TransactionEffects;
use haneul_types::messages::VerifiedTransaction;
use haneul_types::messages_checkpoint::CheckpointContents;
use haneul_types::messages_checkpoint::CheckpointContentsDigest;
use haneul_types::messages_checkpoint::CheckpointDigest;
use haneul_types::messages_checkpoint::CheckpointSequenceNumber;
use haneul_types::messages_checkpoint::EndOfEpochData;
use haneul_types::messages_checkpoint::VerifiedCheckpoint;
use haneul_types::storage::ReadStore;
use haneul_types::storage::WriteStore;
use typed_store::Map;

use crate::authority::AuthorityStore;
use crate::checkpoints::CheckpointStore;
use crate::epoch::committee_store::CommitteeStore;

#[derive(Clone)]
pub struct RocksDbStore {
    authority_store: Arc<AuthorityStore>,
    committee_store: Arc<CommitteeStore>,
    checkpoint_store: Arc<CheckpointStore>,
}

impl RocksDbStore {
    pub fn new(
        authority_store: Arc<AuthorityStore>,
        committee_store: Arc<CommitteeStore>,
        checkpoint_store: Arc<CheckpointStore>,
    ) -> Self {
        Self {
            authority_store,
            committee_store,
            checkpoint_store,
        }
    }
}

impl ReadStore for RocksDbStore {
    type Error = typed_store::rocks::TypedStoreError;

    fn get_checkpoint_by_digest(
        &self,
        digest: &CheckpointDigest,
    ) -> Result<Option<VerifiedCheckpoint>, Self::Error> {
        self.checkpoint_store.get_checkpoint_by_digest(digest)
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Result<Option<VerifiedCheckpoint>, Self::Error> {
        self.checkpoint_store
            .get_checkpoint_by_sequence_number(sequence_number)
    }

    fn get_highest_verified_checkpoint(&self) -> Result<VerifiedCheckpoint, Self::Error> {
        self.checkpoint_store
            .get_highest_verified_checkpoint()
            .map(|maybe_checkpoint| {
                maybe_checkpoint
                    .expect("storage should have been initialized with genesis checkpoint")
            })
    }

    fn get_highest_synced_checkpoint(&self) -> Result<VerifiedCheckpoint, Self::Error> {
        self.checkpoint_store
            .get_highest_synced_checkpoint()
            .map(|maybe_checkpoint| {
                maybe_checkpoint
                    .expect("storage should have been initialized with genesis checkpoint")
            })
    }

    fn get_checkpoint_contents(
        &self,
        digest: &CheckpointContentsDigest,
    ) -> Result<Option<CheckpointContents>, Self::Error> {
        self.checkpoint_store.get_checkpoint_contents(digest)
    }

    fn get_committee(&self, epoch: EpochId) -> Result<Option<Committee>, Self::Error> {
        Ok(self.committee_store.get_committee(&epoch).unwrap())
    }

    fn get_transaction(
        &self,
        digest: &TransactionDigest,
    ) -> Result<Option<VerifiedTransaction>, Self::Error> {
        self.authority_store.get_transaction(digest)
    }

    fn get_transaction_effects(
        &self,
        digest: &TransactionEffectsDigest,
    ) -> Result<Option<TransactionEffects>, Self::Error> {
        self.authority_store.perpetual_tables.effects.get(digest)
    }
}

impl WriteStore for RocksDbStore {
    fn insert_checkpoint(&self, checkpoint: VerifiedCheckpoint) -> Result<(), Self::Error> {
        if let Some(EndOfEpochData {
            next_epoch_committee,
            next_epoch_protocol_version,
            ..
        }) = checkpoint.summary.end_of_epoch_data.as_ref()
        {
            let next_committee = next_epoch_committee.iter().cloned().collect();
            let committee = Committee::new(
                checkpoint.epoch().saturating_add(1),
                *next_epoch_protocol_version,
                next_committee,
            )
            .expect("new committee from consensus should be constructable");
            self.insert_committee(committee)?;
        }

        self.checkpoint_store.insert_verified_checkpoint(checkpoint)
    }

    fn update_highest_synced_checkpoint(
        &self,
        checkpoint: &VerifiedCheckpoint,
    ) -> Result<(), Self::Error> {
        self.checkpoint_store
            .update_highest_synced_checkpoint(checkpoint)
    }

    fn insert_checkpoint_contents(&self, contents: CheckpointContents) -> Result<(), Self::Error> {
        self.checkpoint_store.insert_checkpoint_contents(contents)
    }

    fn insert_committee(&self, new_committee: Committee) -> Result<(), Self::Error> {
        self.committee_store
            .insert_new_committee(&new_committee)
            .unwrap();
        Ok(())
    }

    fn insert_transaction_and_effects(
        &self,
        transaction: VerifiedTransaction,
        transaction_effects: TransactionEffects,
    ) -> Result<(), Self::Error> {
        self.authority_store
            .insert_transaction_and_effects(&transaction, &transaction_effects)
    }
}
