// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::*;
use haneullabs_metrics::spawn_logged_monitored_task;
use haneullabs_metrics::start_prometheus_server;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use haneul_bridge::eth_client::EthClient;
use haneul_bridge::metered_eth_provider::MeteredEthHttpProvier;
use haneul_bridge::metrics::BridgeMetrics;
use haneul_bridge_indexer::eth_worker::EthBridgeWorker;
use haneul_bridge_indexer::metrics::BridgeIndexerMetrics;
use haneul_bridge_indexer::postgres_manager::{get_connection_pool, read_haneul_progress_store};
use haneul_bridge_indexer::haneul_transaction_handler::handle_haneul_transactions_loop;
use haneul_bridge_indexer::haneul_transaction_queries::start_haneul_tx_polling_task;
use haneul_data_ingestion_core::DataIngestionMetrics;
use haneul_sdk::HaneulClientBuilder;
use tokio::task::JoinHandle;

use haneullabs_metrics::metered_channel::channel;
use haneul_bridge_indexer::config::IndexerConfig;
use haneul_bridge_indexer::haneul_checkpoint_ingestion::HaneulCheckpointSyncer;
use haneul_config::Config;
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
    let config_clone = config.clone();

    // Init metrics server
    let registry_service = start_prometheus_server(
        format!("{}:{}", config.metric_url, config.metric_port,)
            .parse()
            .unwrap_or_else(|err| panic!("Failed to parse metric address: {}", err)),
    );
    let registry = registry_service.default_registry();

    haneullabs_metrics::init_metrics(&registry);

    info!(
        "Metrics server started at {}::{}",
        config.metric_url, config.metric_port
    );
    let indexer_meterics = BridgeIndexerMetrics::new(&registry);
    let ingestion_metrics = DataIngestionMetrics::new(&registry);
    let bridge_metrics = Arc::new(BridgeMetrics::new(&registry));

    // unwrap safe: db_url must be set in `load_config` above
    let db_url = config.db_url.clone();

    // TODO: retry_with_max_elapsed_time
    let eth_worker = EthBridgeWorker::new(
        get_connection_pool(db_url.clone()),
        bridge_metrics.clone(),
        indexer_meterics.clone(),
        config.clone(),
    )?;

    let eth_client = Arc::new(
        EthClient::<MeteredEthHttpProvier>::new(
            &config.eth_rpc_url,
            HashSet::from_iter(vec![eth_worker.bridge_address()]),
            bridge_metrics.clone(),
        )
        .await?,
    );

    let unfinalized_handle = eth_worker
        .start_indexing_unfinalized_events(eth_client.clone())
        .await?;
    let finalized_handle = eth_worker
        .start_indexing_finalized_events(eth_client.clone())
        .await?;
    let handles = vec![unfinalized_handle, finalized_handle];

    if let Some(haneul_rpc_url) = config.haneul_rpc_url.clone() {
        start_processing_haneul_checkpoints_by_querying_txns(
            haneul_rpc_url,
            db_url.clone(),
            indexer_meterics.clone(),
            bridge_metrics,
        )
        .await?;
    } else {
        let pg_pool = get_connection_pool(db_url.clone());
        HaneulCheckpointSyncer::new(pg_pool, config.bridge_genesis_checkpoint)
            .start(&config_clone, indexer_meterics, ingestion_metrics)
            .await?;
    }
    // We are not waiting for the haneul tasks to finish here, which is ok.
    futures::future::join_all(handles).await;

    Ok(())
}

async fn start_processing_haneul_checkpoints_by_querying_txns(
    haneul_rpc_url: String,
    db_url: String,
    indexer_metrics: BridgeIndexerMetrics,
    bridge_metrics: Arc<BridgeMetrics>,
) -> Result<Vec<JoinHandle<()>>> {
    let pg_pool = get_connection_pool(db_url.clone());
    let (tx, rx) = channel(
        100,
        &haneullabs_metrics::get_metrics()
            .unwrap()
            .channel_inflight
            .with_label_values(&["haneul_transaction_processing_queue"]),
    );
    let mut handles = vec![];
    let cursor =
        read_haneul_progress_store(&pg_pool).expect("Failed to read cursor from haneul progress store");
    let haneul_client = HaneulClientBuilder::default().build(haneul_rpc_url).await?;
    handles.push(spawn_logged_monitored_task!(
        start_haneul_tx_polling_task(haneul_client, cursor, tx, bridge_metrics),
        "start_haneul_tx_polling_task"
    ));
    handles.push(spawn_logged_monitored_task!(
        handle_haneul_transactions_loop(pg_pool.clone(), rx, indexer_metrics.clone()),
        "handle_haneul_transcations_loop"
    ));
    Ok(handles)
}
