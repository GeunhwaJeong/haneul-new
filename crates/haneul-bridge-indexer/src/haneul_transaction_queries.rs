// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;
use haneul_json_rpc_types::HaneulTransactionBlockResponseOptions;
use haneul_json_rpc_types::HaneulTransactionBlockResponseQuery;
use haneul_json_rpc_types::TransactionFilter;
use haneul_sdk::HaneulClient;
use haneul_types::digests::TransactionDigest;
use haneul_types::HANEUL_BRIDGE_OBJECT_ID;

use haneul_bridge::retry_with_max_elapsed_time;
use tracing::{error, info};

use crate::types::RetrievedTransaction;

const QUERY_DURATION: Duration = Duration::from_secs(1);
const SLEEP_DURATION: Duration = Duration::from_secs(5);

pub async fn start_haneul_tx_polling_task(
    haneul_client: HaneulClient,
    mut cursor: Option<TransactionDigest>,
    tx: haneullabs_metrics::metered_channel::Sender<(
        Vec<RetrievedTransaction>,
        Option<TransactionDigest>,
    )>,
) {
    info!("Starting HANEUL transaction polling task from {:?}", cursor);
    loop {
        let Ok(Ok(results)) = retry_with_max_elapsed_time!(
            haneul_client.read_api().query_transaction_blocks(
                HaneulTransactionBlockResponseQuery {
                    filter: Some(TransactionFilter::InputObject(HANEUL_BRIDGE_OBJECT_ID)),
                    options: Some(HaneulTransactionBlockResponseOptions::full_content()),
                },
                cursor,
                None,
                false,
            ),
            Duration::from_secs(600)
        ) else {
            error!("Failed to query bridge transactions after retry");
            continue;
        };
        info!("Retrieved {} bridge transactions", results.data.len());
        let txes = match results
            .data
            .into_iter()
            .map(RetrievedTransaction::try_from)
            .collect::<anyhow::Result<Vec<_>>>()
        {
            Ok(data) => data,
            Err(e) => {
                // TOOD: Sometimes fullnode does not return checkpoint strangely. We retry instead of
                // panicking.
                error!(
                    "Failed to convert retrieved transactions to sanitized format: {}",
                    e
                );
                tokio::time::sleep(SLEEP_DURATION).await;
                continue;
            }
        };
        if txes.is_empty() {
            // When there is no more new data, we are caught up, no need to stress the fullnode
            tokio::time::sleep(QUERY_DURATION).await;
            continue;
        }
        tx.send((txes, results.next_cursor))
            .await
            .expect("Failed to send transaction block to process");
        cursor = results.next_cursor;
    }
}
