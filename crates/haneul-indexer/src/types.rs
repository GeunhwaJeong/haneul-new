// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_json_rpc_types::{
    BalanceChange, ObjectChange, HaneulTransactionBlock, HaneulTransactionBlockEffects,
    HaneulTransactionBlockEvents, HaneulTransactionBlockResponse, HaneulTransactionBlockResponseOptions,
};
use haneul_types::digests::TransactionDigest;
use haneul_types::messages_checkpoint::CheckpointSequenceNumber;
use haneul_types::storage::WriteKind;

const CREATED_OBJECT_CHANGE_TYPE: &str = "created";
const MUTATED_OBJECT_CHANGE_TYPE: &str = "mutated";
const UNWRAPPED_OBJECT_CHANGE_TYPE: &str = "unwrapped";

pub fn write_kind_to_str(write_kind: WriteKind) -> &'static str {
    match write_kind {
        WriteKind::Mutate => MUTATED_OBJECT_CHANGE_TYPE,
        WriteKind::Create => CREATED_OBJECT_CHANGE_TYPE,
        WriteKind::Unwrap => UNWRAPPED_OBJECT_CHANGE_TYPE,
    }
}

#[derive(Debug, Clone)]
pub struct CheckpointTransactionBlockResponse {
    pub digest: TransactionDigest,
    /// Transaction input data
    pub transaction: HaneulTransactionBlock,
    pub raw_transaction: Vec<u8>,
    pub effects: HaneulTransactionBlockEffects,
    pub events: HaneulTransactionBlockEvents,
    pub timestamp_ms: u64,
    pub confirmed_local_execution: Option<bool>,
    pub checkpoint: CheckpointSequenceNumber,
}

impl TryFrom<HaneulTransactionBlockResponse> for CheckpointTransactionBlockResponse {
    type Error = anyhow::Error;

    fn try_from(response: HaneulTransactionBlockResponse) -> Result<Self, Self::Error> {
        let HaneulTransactionBlockResponse {
            digest,
            transaction,
            raw_transaction,
            effects,
            events,
            object_changes: _,
            balance_changes: _,
            timestamp_ms,
            confirmed_local_execution,
            checkpoint,
            errors,
            raw_effects: _,
        } = response;

        let transaction = transaction.ok_or_else(|| {
            anyhow::anyhow!(
                "Transaction is None in HaneulTransactionBlockFullResponse of digest {:?}.",
                digest
            )
        })?;
        let effects = effects.ok_or_else(|| {
            anyhow::anyhow!(
                "Effects is None in HaneulTransactionBlockFullResponse of digest {:?}.",
                digest
            )
        })?;
        let events = events.ok_or_else(|| {
            anyhow::anyhow!(
                "Events is None in HaneulTransactionBlockFullResponse of digest {:?}.",
                digest
            )
        })?;
        let timestamp_ms = timestamp_ms.ok_or_else(|| {
            anyhow::anyhow!(
                "TimestampMs is None in HaneulTransactionBlockFullResponse of digest {:?}.",
                digest
            )
        })?;
        let checkpoint = checkpoint.ok_or_else(|| {
            anyhow::anyhow!(
                "Checkpoint is None in HaneulTransactionBlockFullResponse of digest {:?}.",
                digest
            )
        })?;
        if raw_transaction.is_empty() {
            return Err(anyhow::anyhow!(
                "Unexpected empty RawTransaction in HaneulTransactionBlockFullResponse of digest {:?}.",
                digest
            ));
        }
        if !errors.is_empty() {
            return Err(anyhow::anyhow!(
                "Errors in HaneulTransactionBlockFullResponse of digest {:?}: {:?}",
                digest,
                errors
            ));
        }

        Ok(CheckpointTransactionBlockResponse {
            digest,
            transaction,
            raw_transaction,
            effects,
            events,
            timestamp_ms,
            confirmed_local_execution,
            checkpoint,
        })
    }
}

pub struct AddressData {
    pub account_address: String,
    pub transaction_digest: String,
    pub timestamp_ms: i64,
}
pub struct TemporaryTransactionBlockResponseStore {
    pub digest: TransactionDigest,
    /// Transaction input data
    pub transaction: HaneulTransactionBlock,
    pub raw_transaction: Vec<u8>,
    pub effects: HaneulTransactionBlockEffects,
    pub events: HaneulTransactionBlockEvents,
    pub object_changes: Option<Vec<ObjectChange>>,
    pub balance_changes: Option<Vec<BalanceChange>>,
    pub timestamp_ms: Option<u64>,
    pub confirmed_local_execution: Option<bool>,
    pub checkpoint: Option<CheckpointSequenceNumber>,
}

impl From<CheckpointTransactionBlockResponse> for TemporaryTransactionBlockResponseStore {
    fn from(value: CheckpointTransactionBlockResponse) -> Self {
        let CheckpointTransactionBlockResponse {
            digest,
            transaction,
            raw_transaction,
            effects,
            events,
            timestamp_ms,
            confirmed_local_execution,
            checkpoint,
        } = value;

        TemporaryTransactionBlockResponseStore {
            digest,
            transaction,
            raw_transaction,
            effects,
            events,
            object_changes: None,
            balance_changes: None,
            timestamp_ms: Some(timestamp_ms),
            confirmed_local_execution,
            checkpoint: Some(checkpoint),
        }
    }
}

// HaneulTransactionBlockResponseWithOptions is only used on the reading path
pub struct HaneulTransactionBlockResponseWithOptions {
    pub response: HaneulTransactionBlockResponse,
    pub options: HaneulTransactionBlockResponseOptions,
}

impl From<HaneulTransactionBlockResponseWithOptions> for HaneulTransactionBlockResponse {
    fn from(value: HaneulTransactionBlockResponseWithOptions) -> Self {
        let HaneulTransactionBlockResponseWithOptions { response, options } = value;

        HaneulTransactionBlockResponse {
            digest: response.digest,
            transaction: options.show_input.then_some(response.transaction).flatten(),
            raw_transaction: options
                .show_raw_input
                .then_some(response.raw_transaction)
                .unwrap_or_default(),
            effects: options.show_effects.then_some(response.effects).flatten(),
            events: options.show_events.then_some(response.events).flatten(),
            object_changes: options
                .show_object_changes
                .then_some(response.object_changes)
                .flatten(),
            balance_changes: options
                .show_balance_changes
                .then_some(response.balance_changes)
                .flatten(),
            timestamp_ms: response.timestamp_ms,
            confirmed_local_execution: response.confirmed_local_execution,
            checkpoint: response.checkpoint,
            errors: vec![],
            raw_effects: options
                .show_raw_effects
                .then_some(response.raw_effects)
                .unwrap_or_default(),
        }
    }
}
