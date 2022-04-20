// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use clap::*;
use std::path::PathBuf;
use haneul::{
    config::{GenesisConfig, NetworkConfig, PersistedConfig},
    haneul_commands::{genesis, make_server},
    haneul_config_dir, HANEUL_NETWORK_CONFIG,
};
use haneul_types::base_types::{decode_bytes_hex, HaneulAddress};
use haneul_types::committee::Committee;
use tracing::{error, info};

#[derive(Parser)]
#[clap(
    name = "Haneul Validator",
    about = "Validator for Haneul Network",
    rename_all = "kebab-case"
)]
struct ValidatorOpt {
    /// The genesis config file location
    #[clap(long)]
    pub genesis_config_path: PathBuf,
    #[clap(long, help = "If set, run genesis even if network.conf already exists")]
    pub force_genesis: bool,

    #[clap(long)]
    pub network_config_path: Option<PathBuf>,

    /// Public key/address of the validator to start
    #[clap(long, parse(try_from_str = decode_bytes_hex))]
    address: Option<HaneulAddress>,

    /// Index in validator array of validator to start
    #[clap(long)]
    validator_idx: Option<usize>,

    #[clap(long, help = "Specify host:port to listen on")]
    listen_address: Option<String>,
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

    let cfg = ValidatorOpt::parse();

    let network_config_path = haneul_config_dir()?.join(HANEUL_NETWORK_CONFIG);

    let network_config = match (network_config_path.exists(), cfg.force_genesis) {
        (true, false) => PersistedConfig::<NetworkConfig>::read(&network_config_path)?,

        // If network.conf is missing, or if --force-genesis is true, we run genesis.
        _ => {
            let genesis_conf: GenesisConfig = PersistedConfig::read(&cfg.genesis_config_path)?;
            let (network_config, _, _) = genesis(genesis_conf).await?;
            network_config
        }
    };

    let net_cfg = if let Some(address) = cfg.address {
        // Find the network config for this validator
        network_config
            .authorities
            .iter()
            .find(|x| HaneulAddress::from(x.key_pair.public_key_bytes()) == address)
            .ok_or_else(|| {
                anyhow!(
                    "Network configs must include config for address {}",
                    address
                )
            })?
    } else if let Some(index) = cfg.validator_idx {
        &network_config.authorities[index]
    } else {
        return Err(anyhow!("Must supply either --address of --validator-idx"));
    };

    let listen_address = cfg
        .listen_address
        .unwrap_or(format!("{}:{}", net_cfg.host, net_cfg.port));

    info!(
        "authority {:?} listening on {} (public addr: {}:{})",
        net_cfg.key_pair.public_key_bytes(),
        listen_address,
        net_cfg.host,
        net_cfg.port
    );

    if let Err(e) = make_server(
        net_cfg,
        &Committee::from(&network_config),
        network_config.buffer_size,
    )
    .await
    .unwrap()
    .spawn_with_bind_address(&listen_address)
    .await
    .unwrap()
    .join()
    .await
    {
        error!("Validator server ended with an error: {e}");
    }

    Ok(())
}
