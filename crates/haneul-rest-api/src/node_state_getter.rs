// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_core::authority::AuthorityState;
use haneul_types::{
    base_types::{ObjectID, VersionNumber},
    digests::{TransactionDigest, TransactionEventsDigest},
    effects::{TransactionEffects, TransactionEvents},
    error::{HaneulError, HaneulResult},
    messages_checkpoint::{
        CheckpointContents, CheckpointContentsDigest, CheckpointSequenceNumber, VerifiedCheckpoint,
    },
    object::Object,
    storage::{ObjectKey, ObjectStore},
    transaction::VerifiedTransaction,
};

/// Trait for getting data from the node state.
/// TODO: need a better name for this?
pub trait NodeStateGetter: Sync + Send {
    fn get_verified_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> HaneulResult<VerifiedCheckpoint>;

    fn get_latest_checkpoint_sequence_number(&self) -> HaneulResult<CheckpointSequenceNumber>;

    fn get_checkpoint_contents(
        &self,
        content_digest: CheckpointContentsDigest,
    ) -> HaneulResult<CheckpointContents>;

    fn multi_get_transaction_blocks(
        &self,
        tx_digests: &[TransactionDigest],
    ) -> HaneulResult<Vec<Option<VerifiedTransaction>>>;

    fn multi_get_executed_effects(
        &self,
        digests: &[TransactionDigest],
    ) -> HaneulResult<Vec<Option<TransactionEffects>>>;

    fn multi_get_events(
        &self,
        event_digests: &[TransactionEventsDigest],
    ) -> HaneulResult<Vec<Option<TransactionEvents>>>;

    fn multi_get_object_by_key(
        &self,
        object_keys: &[ObjectKey],
    ) -> Result<Vec<Option<Object>>, HaneulError>;

    fn get_object_by_key(
        &self,
        object_id: &ObjectID,
        version: VersionNumber,
    ) -> Result<Option<Object>, HaneulError>;

    fn get_object(&self, object_id: &ObjectID) -> Result<Option<Object>, HaneulError>;
}

impl NodeStateGetter for AuthorityState {
    fn get_verified_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> HaneulResult<VerifiedCheckpoint> {
        self.get_verified_checkpoint_by_sequence_number(sequence_number)
    }

    fn get_latest_checkpoint_sequence_number(&self) -> HaneulResult<CheckpointSequenceNumber> {
        self.get_latest_checkpoint_sequence_number()
    }

    fn get_checkpoint_contents(
        &self,
        content_digest: CheckpointContentsDigest,
    ) -> HaneulResult<CheckpointContents> {
        self.get_checkpoint_contents(content_digest)
    }

    fn multi_get_transaction_blocks(
        &self,
        tx_digests: &[TransactionDigest],
    ) -> HaneulResult<Vec<Option<VerifiedTransaction>>> {
        self.database.multi_get_transaction_blocks(tx_digests)
    }

    fn multi_get_executed_effects(
        &self,
        digests: &[TransactionDigest],
    ) -> HaneulResult<Vec<Option<TransactionEffects>>> {
        self.database.multi_get_executed_effects(digests)
    }

    fn multi_get_events(
        &self,
        event_digests: &[TransactionEventsDigest],
    ) -> HaneulResult<Vec<Option<TransactionEvents>>> {
        self.database.multi_get_events(event_digests)
    }

    fn multi_get_object_by_key(
        &self,
        object_keys: &[ObjectKey],
    ) -> Result<Vec<Option<Object>>, HaneulError> {
        self.database.multi_get_object_by_key(object_keys)
    }

    fn get_object_by_key(
        &self,
        object_id: &ObjectID,
        version: VersionNumber,
    ) -> Result<Option<Object>, HaneulError> {
        self.database.get_object_by_key(object_id, version)
    }

    fn get_object(&self, object_id: &ObjectID) -> Result<Option<Object>, HaneulError> {
        self.database.get_object(object_id)
    }
}
