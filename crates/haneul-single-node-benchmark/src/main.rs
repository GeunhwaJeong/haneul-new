// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use haneul_single_node_benchmark::command::Command;
use haneul_single_node_benchmark::run_benchmark;
use haneul_single_node_benchmark::workload::Workload;

#[tokio::main]
async fn main() {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_log_level("off,haneul_single_node_benchmark=info")
        .with_env()
        .init();

    let args = Command::parse();
    run_benchmark(
        Workload::new(args.tx_count, args.workload, args.num_input_objects),
        args.component,
        args.checkpoint_size,
    )
    .await;
}
