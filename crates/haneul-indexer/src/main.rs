// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use tracing::info;

use haneul_indexer::errors::IndexerError;
use haneul_indexer::store::PgIndexerStore;
use haneul_indexer::utils::reset_database;
use haneul_indexer::{get_pg_pool_connection, new_pg_connection_pool, Indexer, IndexerConfig};
use haneul_node::metrics::start_prometheus_server;

#[tokio::main]
async fn main() -> Result<(), IndexerError> {
    // NOTE: this is to print out tracing like info, warn & error.
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let indexer_config = IndexerConfig::parse();
    info!("indexer config: {:#?}", indexer_config);
    let registry_service = start_prometheus_server(
        // NOTE: this parses the input host addr and port number for socket addr,
        // so unwrap() is safe here.
        format!(
            "{}:{}",
            indexer_config.client_metric_host, indexer_config.client_metric_port
        )
        .parse()
        .unwrap(),
    );

    let registry = registry_service.default_registry();
    let pg_connection_pool = new_pg_connection_pool(&indexer_config.db_url)?;
    if indexer_config.reset_db {
        let mut conn = get_pg_pool_connection(&pg_connection_pool)?;
        reset_database(&mut conn, /* drop_all */ true).map_err(|e| {
            IndexerError::PostgresResetError(format!(
                "unable to reset database with url: {:?} and err: {:?}",
                indexer_config.db_url.clone(),
                e
            ))
        })?;
    }
    let store = PgIndexerStore::new(pg_connection_pool);

    Indexer::start(&indexer_config, &registry, store).await
}
