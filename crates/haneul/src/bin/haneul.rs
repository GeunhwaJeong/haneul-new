// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
extern crate core;

use clap::*;
use haneul::haneul_commands::HaneulCommand;

#[cfg(test)]
#[path = "../unit_tests/cli_tests.rs"]
mod cli_tests;

#[derive(Parser)]
#[clap(
    name = "Haneul Local",
    about = "A Byzantine fault tolerant chain with low-latency finality and high throughput",
    rename_all = "kebab-case"
)]
struct HaneulOpt {
    #[clap(subcommand)]
    command: HaneulCommand,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _guard = telemetry_subscribers::TelemetryConfig::new(env!("CARGO_BIN_NAME"))
        .with_env()
        .init();

    let options: HaneulOpt = HaneulOpt::parse();
    options.command.execute().await
}
