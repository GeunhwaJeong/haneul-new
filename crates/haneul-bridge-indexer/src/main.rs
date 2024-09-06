// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::*;
use std::collections::HashSet;
use std::env;
use std::net::IpAddr;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use haneul_bridge_indexer::eth_bridge_indexer::EthSubscriptionDatasource;
use haneul_bridge_indexer::eth_bridge_indexer::EthSyncDatasource;
use tokio::task::JoinHandle;
use tracing::info;

use haneullabs_metrics::metered_channel::channel;
use haneullabs_metrics::spawn_logged_monitored_task;
use haneullabs_metrics::start_prometheus_server;
use haneul_bridge::eth_client::EthClient;
use haneul_bridge::metered_eth_provider::MeteredEthHttpProvier;
use haneul_bridge::metrics::BridgeMetrics;
use haneul_bridge_indexer::config::IndexerConfig;
use haneul_bridge_indexer::eth_bridge_indexer::EthDataMapper;
use haneul_bridge_indexer::metrics::BridgeIndexerMetrics;
use haneul_bridge_indexer::postgres_manager::{get_connection_pool, read_haneul_progress_store};
use haneul_bridge_indexer::haneul_bridge_indexer::{PgBridgePersistent, HaneulBridgeDataMapper};
use haneul_bridge_indexer::haneul_datasource::HaneulCheckpointDatasource;
use haneul_bridge_indexer::haneul_transaction_handler::handle_haneul_transactions_loop;
use haneul_bridge_indexer::haneul_transaction_queries::start_haneul_tx_polling_task;
use haneul_config::Config;
use haneul_data_ingestion_core::DataIngestionMetrics;
use haneul_indexer_builder::indexer_builder::{BackfillStrategy, IndexerBuilder};
use haneul_sdk::HaneulClientBuilder;

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

    let indexer_meterics = BridgeIndexerMetrics::new(&registry);
    let ingestion_metrics = DataIngestionMetrics::new(&registry);
    let bridge_metrics = Arc::new(BridgeMetrics::new(&registry));

    let db_url = config.db_url.clone();
    let datastore = PgBridgePersistent::new(get_connection_pool(db_url.clone()).await);

    let eth_client: Arc<EthClient<MeteredEthHttpProvier>> = Arc::new(
        EthClient::<MeteredEthHttpProvier>::new(
            &config.eth_rpc_url,
            HashSet::from_iter(vec![]), // dummy
            bridge_metrics.clone(),
        )
        .await?,
    );

    // Start the eth subscription indexer
    let eth_subscription_datasource = EthSubscriptionDatasource::new(
        config.eth_haneul_bridge_contract_address.clone(),
        eth_client.clone(),
        config.eth_ws_url.clone(),
        indexer_meterics.clone(),
        config.eth_bridge_genesis_block,
    )
    .await?;
    let eth_subscription_indexer = IndexerBuilder::new(
        "EthBridgeSubscriptionIndexer",
        eth_subscription_datasource,
        EthDataMapper {
            metrics: indexer_meterics.clone(),
        },
        datastore.clone(),
    )
    .with_backfill_strategy(BackfillStrategy::Disabled)
    .build();
    let subscription_indexer_fut = spawn_logged_monitored_task!(eth_subscription_indexer.start());

    // Start the eth sync data source
    let eth_sync_datasource = EthSyncDatasource::new(
        config.eth_haneul_bridge_contract_address.clone(),
        config.eth_rpc_url.clone(),
        indexer_meterics.clone(),
        bridge_metrics.clone(),
        config.eth_bridge_genesis_block,
    )
    .await?;
    let eth_sync_indexer = IndexerBuilder::new(
        "EthBridgeSyncIndexer",
        eth_sync_datasource,
        EthDataMapper {
            metrics: indexer_meterics.clone(),
        },
        datastore.clone(),
    )
    .with_backfill_strategy(BackfillStrategy::Partitioned { task_size: 1000 })
    .disable_live_task()
    .build();
    let sync_indexer_fut = spawn_logged_monitored_task!(eth_sync_indexer.start());

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
        config.haneul_bridge_genesis_checkpoint,
        ingestion_metrics.clone(),
        indexer_meterics.clone(),
    );
    let indexer = IndexerBuilder::new(
        "HaneulBridgeIndexer",
        haneul_checkpoint_datasource,
        HaneulBridgeDataMapper {
            metrics: indexer_meterics.clone(),
        },
        datastore,
    )
    .build();
    indexer.start().await?;

    // These tasks should not finish
    subscription_indexer_fut.await.unwrap().unwrap();
    sync_indexer_fut.await.unwrap().unwrap();
    Ok(())
}

#[allow(unused)]
async fn start_processing_haneul_checkpoints_by_querying_txns(
    haneul_rpc_url: String,
    db_url: String,
    indexer_metrics: BridgeIndexerMetrics,
    bridge_metrics: Arc<BridgeMetrics>,
) -> Result<Vec<JoinHandle<()>>> {
    let pg_pool = get_connection_pool(db_url.clone()).await;
    let (tx, rx) = channel(
        100,
        &haneullabs_metrics::get_metrics()
            .unwrap()
            .channel_inflight
            .with_label_values(&["haneul_transaction_processing_queue"]),
    );
    let mut handles = vec![];
    let cursor = read_haneul_progress_store(&pg_pool)
        .await
        .expect("Failed to read cursor from haneul progress store");
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
