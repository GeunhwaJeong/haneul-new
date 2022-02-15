extern crate core;

// Copyright (c) Haneul Labs
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use structopt::StructOpt;
use haneul::config::NetworkConfig;
use haneul::haneul_commands::HaneulCommand;
use haneul::utils::Config;

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
    #[structopt(long, default_value = "./network.conf")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().init();

    let options: HaneulOpt = HaneulOpt::from_args();
    let network_conf_path = options.config;
    let mut config = NetworkConfig::read_or_create(&network_conf_path)?;

    options.command.execute(&mut config).await
}
