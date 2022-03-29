// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
extern crate core;

use structopt::StructOpt;
use haneul::haneul_commands::HaneulCommand;

#[cfg(test)]
#[path = "unit_tests/cli_tests.rs"]
mod cli_tests;

#[derive(StructOpt)]
#[structopt(
    name = "Haneul Local",
    about = "A Byzantine fault tolerant chain with low-latency finality and high throughput",
    rename_all = "kebab-case"
)]
struct HaneulOpt {
    #[structopt(subcommand)]
    command: HaneulCommand,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = telemetry_subscribers::TelemetryConfig {
        service_name: "haneul".into(),
        enable_tracing: std::env::var("HANEUL_TRACING_ENABLE").is_ok(),
        json_log_output: std::env::var("HANEUL_JSON_SPAN_LOGS").is_ok(),
        ..Default::default()
    };
    #[allow(unused)]
    let guard = telemetry_subscribers::init(config);

    let options: HaneulOpt = HaneulOpt::from_args();
    options.command.execute().await
}
