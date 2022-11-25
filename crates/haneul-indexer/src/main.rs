// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use backoff::future::retry;
use backoff::ExponentialBackoff;
use std::time::Duration;
use haneul::config::{PersistedConfig, HaneulClientConfig};
use haneul_config::{haneul_config_dir, HANEUL_CLIENT_CONFIG};
use haneul_sdk::HaneulClient;
use tracing::info;

pub mod handlers;

use handlers::handler_orchestrator::HandlerOrchestrator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = telemetry_subscribers::TelemetryConfig::new(env!("CARGO_BIN_NAME"))
        .with_env()
        .init();
    info!("Haneul indexer started...");

    retry(ExponentialBackoff::default(), || async {
        let rpc_client = new_rpc_client().await?;
        // NOTE: Each handler is responsible for one type of data from nodes,like transactions and events;
        // Handler orchestrator runs these handlers in parallel and manage them upon errors etc.
        HandlerOrchestrator::new(rpc_client).run_forever().await;
        Ok(())
    })
    .await
}

async fn new_rpc_client() -> Result<HaneulClient, anyhow::Error> {
    info!("Getting new rpc client...");
    let config_path = haneul_config_dir()?.join(HANEUL_CLIENT_CONFIG);
    let config: HaneulClientConfig = PersistedConfig::read(&config_path)?;
    config
        .get_active_env()?
        .create_rpc_client(Some(Duration::from_secs(10)))
        .await
}
