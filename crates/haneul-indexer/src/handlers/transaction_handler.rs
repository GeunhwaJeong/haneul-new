// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use futures::future::join_all;
use haneul_json_rpc_types::{HaneulTransactionResponse, TransactionsPage};
use haneul_sdk::HaneulClient;
use haneul_types::base_types::TransactionDigest;
use haneul_types::query::TransactionQuery;
use tracing::info;

use haneul_indexer::errors::IndexerError;
use haneul_indexer::establish_connection;
use haneul_indexer::models::transaction_logs::{commit_transction_log, read_transaction_log};
use haneul_indexer::models::transactions::commit_transactions;
use haneul_indexer::utils::log_errors_to_pg;

const TRANSACTION_PAGE_SIZE: usize = 100;

pub struct TransactionHandler {
    rpc_client: HaneulClient,
}

impl TransactionHandler {
    pub fn new(rpc_client: HaneulClient) -> Self {
        Self { rpc_client }
    }

    pub async fn start(&self) -> Result<(), IndexerError> {
        info!("Indexer transaction handler started...");
        let mut pg_conn = establish_connection();
        let mut next_cursor = None;
        let txn_log = read_transaction_log(&mut pg_conn)?;
        if let Some(txn_digest) = txn_log.next_cursor_tx_digest {
            let bytes = txn_digest.as_bytes();
            let digest = TransactionDigest::try_from(bytes).map_err(|e| {
                IndexerError::TransactionDigestParsingError(format!(
                    "Failed parsing transaction digest {:?} with error: {:?}",
                    txn_digest, e
                ))
            })?;
            next_cursor = Some(digest);
        }

        loop {
            let page = self.get_transaction_page(next_cursor).await?;
            let txn_digest_vec = page.data;
            let txn_response_res_vec = join_all(
                txn_digest_vec
                    .into_iter()
                    .map(|tx_digest| self.get_transaction_response(tx_digest)),
            )
            .await;

            let mut errors = vec![];
            let resp_vec: Vec<HaneulTransactionResponse> = txn_response_res_vec
                .into_iter()
                .filter_map(|f| f.map_err(|e| errors.push(e)).ok())
                .collect();
            log_errors_to_pg(errors);

            commit_transactions(&mut pg_conn, resp_vec)?;
            commit_transction_log(&mut pg_conn, page.next_cursor.map(|d| d.to_string()))?;
            next_cursor = page.next_cursor;
        }
    }

    async fn get_transaction_page(
        &self,
        cursor: Option<TransactionDigest>,
    ) -> Result<TransactionsPage, IndexerError> {
        self.rpc_client
            .read_api()
            .get_transactions(
                TransactionQuery::All,
                cursor,
                Some(TRANSACTION_PAGE_SIZE),
                None,
            )
            .await
            .map_err(|e| {
                IndexerError::FullNodeReadingError(format!(
                    "Failed reading transaction page with cursor {:?} and err: {:?}",
                    cursor, e
                ))
            })
    }

    async fn get_transaction_response(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<HaneulTransactionResponse, IndexerError> {
        self.rpc_client
            .read_api()
            .get_transaction(tx_digest)
            .await
            .map_err(|e| {
                IndexerError::FullNodeReadingError(format!(
                    "Failed reading transaction response with tx digest {:?} and err: {:?}",
                    tx_digest, e
                ))
            })
    }
}
