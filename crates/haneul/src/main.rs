// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::*;
use colored::Colorize;
use haneul::haneul_commands::HaneulCommand;
use haneul_types::exit_main;
use haneul_types::software_version::VERSION;
use tracing::debug;

#[cfg(test)]
#[path = "unit_tests/cli_tests.rs"]
mod cli_tests;

#[derive(Parser)]
#[clap(
    name = env!("CARGO_BIN_NAME"),
    about = "A Byzantine fault tolerant chain with low-latency finality and high throughput",
    rename_all = "kebab-case",
    author,
    version = VERSION,
)]
struct Args {
    #[clap(subcommand)]
    command: HaneulCommand,
}

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let bin_name = env!("CARGO_BIN_NAME");
    let args = Args::parse();
    let _guard = match args.command {
        HaneulCommand::Console { .. } | HaneulCommand::Client { .. } => {
            telemetry_subscribers::TelemetryConfig::new()
                .with_log_file(&format!("{bin_name}.log"))
                .with_env()
                .init()
        }
        _ => telemetry_subscribers::TelemetryConfig::new()
            .with_env()
            .init(),
    };

    debug!("Haneul CLI version: {VERSION}");

    exit_main!(args.command.execute().await);
}
