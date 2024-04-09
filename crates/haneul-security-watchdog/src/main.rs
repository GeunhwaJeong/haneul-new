// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use prometheus::Registry;

use anyhow::Result;
use clap::*;
use haneul_security_watchdog::scheduler::SchedulerService;
use haneul_security_watchdog::SecurityWatchdogConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let config = SecurityWatchdogConfig::parse();
    let registry_service = haneullabs_metrics::start_prometheus_server(
        format!(
            "{}:{}",
            config.client_metric_host, config.client_metric_port
        )
        .parse()
        .unwrap(),
    );
    let registry: Registry = registry_service.default_registry();
    haneullabs_metrics::init_metrics(&registry);
    let service = SchedulerService::new(&config, &registry).await?;
    service.schedule().await?;
    service.start().await?;
    tokio::signal::ctrl_c().await?;
    Ok(())
}
