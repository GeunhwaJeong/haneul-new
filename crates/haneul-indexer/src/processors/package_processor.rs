// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_indexer::errors::IndexerError;
use haneul_indexer::establish_connection;
use haneul_indexer::models::events::{events_to_haneul_events, read_events};
use haneul_indexer::models::package_logs::{commit_package_log, read_package_log};
use haneul_indexer::models::packages::commit_packages_from_events;
use haneul_sdk::HaneulClient;

use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

const PACKAGE_EVENT_BATCH_SIZE: usize = 100;

pub struct PackageProcessor {
    rpc_client: HaneulClient,
    db_url: String,
}

impl PackageProcessor {
    pub fn new(rpc_client: HaneulClient, db_url: String) -> PackageProcessor {
        Self { rpc_client, db_url }
    }

    pub async fn start(&self) -> Result<(), IndexerError> {
        info!("Indexer package processor started...");
        let mut pg_conn = establish_connection(self.db_url.clone());

        let pkg_log = read_package_log(&mut pg_conn)?;
        let mut last_processed_id = pkg_log.last_processed_id;
        loop {
            let events_to_process =
                read_events(&mut pg_conn, last_processed_id, PACKAGE_EVENT_BATCH_SIZE)?;
            let haneul_events_to_process = events_to_haneul_events(&mut pg_conn, events_to_process);

            let event_count = haneul_events_to_process.len();

            commit_packages_from_events(
                self.rpc_client.clone(),
                &mut pg_conn,
                haneul_events_to_process,
            )
            .await?;

            last_processed_id += event_count as i64;
            commit_package_log(&mut pg_conn, last_processed_id)?;
            if event_count < PACKAGE_EVENT_BATCH_SIZE {
                sleep(Duration::from_secs_f32(0.1)).await;
            }
        }
    }
}
