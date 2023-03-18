// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_json_rpc_types::{
    BalanceChange, ObjectChange, HaneulTransaction, HaneulTransactionEffects, HaneulTransactionEvents,
    HaneulTransactionResponse,
};
use haneul_types::digests::TransactionDigest;
use haneul_types::messages_checkpoint::CheckpointSequenceNumber;

#[derive(Debug, Clone)]
pub struct HaneulTransactionFullResponse {
    pub digest: TransactionDigest,
    /// Transaction input data
    pub transaction: HaneulTransaction,
    pub raw_transaction: Vec<u8>,
    pub effects: HaneulTransactionEffects,
    pub events: HaneulTransactionEvents,
    pub object_changes: Option<Vec<ObjectChange>>,
    pub balance_changes: Option<Vec<BalanceChange>>,
    pub timestamp_ms: u64,
    pub confirmed_local_execution: Option<bool>,
    pub checkpoint: CheckpointSequenceNumber,
}

impl TryFrom<HaneulTransactionResponse> for HaneulTransactionFullResponse {
    type Error = anyhow::Error;

    fn try_from(response: HaneulTransactionResponse) -> Result<Self, Self::Error> {
        let HaneulTransactionResponse {
            digest,
            transaction,
            raw_transaction,
            effects,
            events,
            object_changes,
            balance_changes,
            timestamp_ms,
            confirmed_local_execution,
            checkpoint,
            errors,
        } = response;

        let transaction = transaction.ok_or_else(|| {
            anyhow::anyhow!(
                "Transaction is None in HaneulTransactionFullResponse of digest {:?}.",
                digest
            )
        })?;
        let effects = effects.ok_or_else(|| {
            anyhow::anyhow!(
                "Effects is None in HaneulTransactionFullResponse of digest {:?}.",
                digest
            )
        })?;
        let events = events.ok_or_else(|| {
            anyhow::anyhow!(
                "Events is None in HaneulTransactionFullResponse of digest {:?}.",
                digest
            )
        })?;
        let timestamp_ms = timestamp_ms.ok_or_else(|| {
            anyhow::anyhow!(
                "TimestampMs is None in HaneulTransactionFullResponse of digest {:?}.",
                digest
            )
        })?;
        let checkpoint = checkpoint.ok_or_else(|| {
            anyhow::anyhow!(
                "Checkpoint is None in HaneulTransactionFullResponse of digest {:?}.",
                digest
            )
        })?;
        if !errors.is_empty() {
            return Err(anyhow::anyhow!(
                "Errors in HaneulTransactionFullResponse of digest {:?}: {:?}",
                digest,
                errors
            ));
        }

        Ok(HaneulTransactionFullResponse {
            digest,
            transaction,
            raw_transaction,
            effects,
            events,
            object_changes,
            balance_changes,
            timestamp_ms,
            confirmed_local_execution,
            checkpoint,
        })
    }
}

impl From<HaneulTransactionFullResponse> for HaneulTransactionResponse {
    fn from(response: HaneulTransactionFullResponse) -> Self {
        let HaneulTransactionFullResponse {
            digest,
            transaction,
            effects,
            events,
            object_changes,
            balance_changes,
            timestamp_ms,
            confirmed_local_execution,
            checkpoint,
            raw_transaction,
        } = response;

        HaneulTransactionResponse {
            digest,
            transaction: Some(transaction),
            raw_transaction,
            effects: Some(effects),
            events: Some(events),
            object_changes,
            balance_changes,
            timestamp_ms: Some(timestamp_ms),
            confirmed_local_execution,
            checkpoint: Some(checkpoint),
            errors: vec![],
        }
    }
}
