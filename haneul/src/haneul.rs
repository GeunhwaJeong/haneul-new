// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
extern crate core;

use std::path::PathBuf;
use structopt::StructOpt;
use haneul::config::{Config, NetworkConfig};
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
