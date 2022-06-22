// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::Parser;
use multiaddr::Multiaddr;
use std::path::PathBuf;
use haneul_config::{Config, NodeConfig};

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
struct Args {
    #[clap(long)]
    pub config_path: PathBuf,

    #[clap(long, help = "Specify address to listen on")]
    listen_address: Option<Multiaddr>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let _guard = telemetry_subscribers::TelemetryConfig::new(env!("CARGO_BIN_NAME"))
        .with_env()
        .init();

    let args = Args::parse();

    let mut config = NodeConfig::load(&args.config_path)?;

    if let Some(listen_address) = args.listen_address {
        config.network_address = listen_address;
    }

    let node = haneul_node::HaneulNode::start(&config).await?;
    node.wait().await?;

    Ok(())
}
