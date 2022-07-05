// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
extern crate core;

use clap::*;
use colored::Colorize;
use haneul::haneul_commands::HaneulCommand;
use haneul_types::exit_main;
use tracing::debug;
#[cfg(test)]
#[path = "unit_tests/cli_tests.rs"]
mod cli_tests;

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let bin_name = env!("CARGO_BIN_NAME");
    let cmd: HaneulCommand = HaneulCommand::parse();
    let _guard = match cmd {
        HaneulCommand::Console { .. } | HaneulCommand::Client { .. } => {
            telemetry_subscribers::TelemetryConfig::new(bin_name)
                .with_log_file(&format!("{bin_name}.log"))
                .with_env()
                .init()
        }
        _ => telemetry_subscribers::TelemetryConfig::new(bin_name)
            .with_env()
            .init(),
    };

    if let Some(git_rev) = option_env!("GIT_REVISION") {
        debug!("Haneul CLI built at git revision {git_rev}");
    }
    exit_main!(cmd.execute().await);
}
