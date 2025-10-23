// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::handlers::is_bridge_txn;
use async_trait::async_trait;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use haneul_bridge_schema::models::HaneulErrorTransactions;
use haneul_bridge_schema::schema::haneul_error_transactions;
use haneul_indexer_alt_framework::pipeline::Processor;
use haneul_indexer_alt_framework::pipeline::concurrent::Handler;
use haneul_indexer_alt_framework::postgres::Db;
use haneul_indexer_alt_framework::store::Store;
use haneul_indexer_alt_framework::types::effects::TransactionEffectsAPI;
use haneul_indexer_alt_framework::types::execution_status::ExecutionStatus;
use haneul_indexer_alt_framework::types::full_checkpoint_content::CheckpointData;

pub struct ErrorTransactionHandler;

#[async_trait]
impl Processor for ErrorTransactionHandler {
    const NAME: &'static str = "error_transactions";
    type Value = HaneulErrorTransactions;

    async fn process(&self, checkpoint: &Arc<CheckpointData>) -> anyhow::Result<Vec<Self::Value>> {
        let timestamp_ms = checkpoint.checkpoint_summary.timestamp_ms as i64;
        let mut results = vec![];

        for tx in &checkpoint.transactions {
            if !is_bridge_txn(tx) {
                continue;
            }
            if let ExecutionStatus::Failure { error, command } = tx.effects.status() {
                results.push(HaneulErrorTransactions {
                    txn_digest: tx.transaction.digest().inner().to_vec(),
                    timestamp_ms,
                    failure_status: error.to_string(),
                    cmd_idx: command.map(|idx| idx as i64),
                    sender_address: tx.transaction.sender_address().to_vec(),
                })
            }
        }
        Ok(results)
    }
}

#[async_trait]
impl Handler for ErrorTransactionHandler {
    type Store = Db;

    async fn commit<'a>(
        values: &[Self::Value],
        conn: &mut <Self::Store as Store>::Connection<'a>,
    ) -> anyhow::Result<usize> {
        Ok(diesel::insert_into(haneul_error_transactions::table)
            .values(values)
            .on_conflict_do_nothing()
            .execute(conn)
            .await?)
    }
}
