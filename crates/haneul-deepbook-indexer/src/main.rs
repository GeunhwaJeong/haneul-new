// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::*;
use haneullabs_metrics::start_prometheus_server;
use std::env;
use std::net::IpAddr;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use haneul_config::Config;
use haneul_data_ingestion_core::DataIngestionMetrics;
use haneul_deepbook_indexer::config::IndexerConfig;
use haneul_deepbook_indexer::metrics::DeepBookIndexerMetrics;
use haneul_deepbook_indexer::postgres_manager::get_connection_pool;
use haneul_deepbook_indexer::haneul_datasource::HaneulCheckpointDatasource;
use haneul_deepbook_indexer::haneul_deepbook_indexer::PgDeepbookPersistent;
use haneul_deepbook_indexer::haneul_deepbook_indexer::HaneulDeepBookDataMapper;
use haneul_indexer_builder::indexer_builder::IndexerBuilder;
use haneul_indexer_builder::progress::{OutOfOrderSaveAfterDurationPolicy, ProgressSavingPolicy};
use haneul_sdk::HaneulClientBuilder;
use haneul_types::base_types::ObjectID;
use tracing::info;

#[derive(Parser, Clone, Debug)]
struct Args {
    /// Path to a yaml config
    #[clap(long, short)]
    config_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let args = Args::parse();

    // load config
    let config_path = if let Some(path) = args.config_path {
        path
    } else {
        env::current_dir()
            .expect("Couldn't get current directory")
            .join("config.yaml")
    };
    let config = IndexerConfig::load(&config_path)?;

    // Init metrics server
    let metrics_address =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.metric_port);
    let registry_service = start_prometheus_server(metrics_address);
    let registry = registry_service.default_registry();
    haneullabs_metrics::init_metrics(&registry);
    info!("Metrics server started at port {}", config.metric_port);

    let indexer_meterics = DeepBookIndexerMetrics::new(&registry);
    let ingestion_metrics = DataIngestionMetrics::new(&registry);

    let db_url = config.db_url.clone();
    let datastore = PgDeepbookPersistent::new(
        get_connection_pool(db_url.clone()).await,
        ProgressSavingPolicy::OutOfOrderSaveAfterDuration(OutOfOrderSaveAfterDurationPolicy::new(
            tokio::time::Duration::from_secs(30),
        )),
    );

    let haneul_client = Arc::new(
        HaneulClientBuilder::default()
            .build(config.haneul_rpc_url.clone())
            .await?,
    );
    let haneul_checkpoint_datasource = HaneulCheckpointDatasource::new(
        config.remote_store_url,
        haneul_client,
        config.concurrency as usize,
        config.checkpoints_path.clone().into(),
        config.deepbook_genesis_checkpoint,
        ingestion_metrics.clone(),
        indexer_meterics.clone(),
    );
    let indexer = IndexerBuilder::new(
        "HaneulDeepBookIndexer",
        haneul_checkpoint_datasource,
        HaneulDeepBookDataMapper {
            metrics: indexer_meterics.clone(),
            package_id: ObjectID::from_hex_literal(&config.deepbook_package_id.clone())
                .unwrap_or_else(|err| panic!("Failed to parse deepbook package ID: {}", err)),
        },
        datastore,
    )
    .build();
    indexer.start().await?;

    Ok(())
}
