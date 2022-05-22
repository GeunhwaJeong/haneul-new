// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};
use haneul::{
    config::{haneul_config_dir, FULL_NODE_DB_PATH},
    haneul_full_node::HaneulFullNode,
};
use haneul_gateway::api::{RpcGatewayOpenRpc, RpcGatewayServer};
use haneul_gateway::json_rpc::JsonRpcServerBuilder;
use tracing::info;

const DEFAULT_NODE_SERVER_PORT: &str = "5002";
const DEFAULT_NODE_SERVER_ADDR_IPV4: &str = "127.0.0.1";

#[derive(Parser)]
#[clap(name = "Haneul Full Node", about = "TODO", rename_all = "kebab-case")]
struct HaneulNodeOpt {
    #[clap(long)]
    db_path: Option<String>,

    #[clap(long)]
    config: Option<PathBuf>,

    #[clap(long, default_value = DEFAULT_NODE_SERVER_PORT)]
    port: u16,

    #[clap(long, default_value = DEFAULT_NODE_SERVER_ADDR_IPV4)]
    host: Ipv4Addr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = telemetry_subscribers::TelemetryConfig {
        service_name: "haneul_node".into(),
        enable_tracing: std::env::var("HANEUL_TRACING_ENABLE").is_ok(),
        json_log_output: std::env::var("HANEUL_JSON_SPAN_LOGS").is_ok(),
        ..Default::default()
    };
    #[allow(unused)]
    let guard = telemetry_subscribers::init(config);

    let options: HaneulNodeOpt = HaneulNodeOpt::parse();
    let db_path = options
        .db_path
        .map(PathBuf::from)
        .unwrap_or(haneul_config_dir()?.join(FULL_NODE_DB_PATH));

    let config_path = options
        .config
        .unwrap_or(haneul_config_dir()?.join("network.conf"));
    info!("Node config file path: {:?}", config_path);

    let address = SocketAddr::new(IpAddr::V4(options.host), options.port);
    let mut server = JsonRpcServerBuilder::new()?;
    server.register_open_rpc(RpcGatewayOpenRpc::open_rpc())?;
    server.register_methods(
        HaneulFullNode::start_with_genesis(&config_path, &db_path)
            .await?
            .into_rpc(),
    )?;

    let server_handle = server.start(address).await?;

    server_handle.await;
    Ok(())
}
