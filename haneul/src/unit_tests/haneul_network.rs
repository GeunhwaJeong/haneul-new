// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use haneul::config::{AuthorityPrivateInfo, Config, GenesisConfig, WalletConfig};
use haneul::gateway_config::{GatewayConfig, GatewayType};
use haneul::keystore::KeystoreType;
use haneul::haneul_commands::{genesis, HaneulNetwork};
use haneul::{HANEUL_GATEWAY_CONFIG, HANEUL_NETWORK_CONFIG, HANEUL_WALLET_CONFIG};

pub async fn start_test_network(
    working_dir: &Path,
    genesis_config: Option<GenesisConfig>,
) -> Result<HaneulNetwork, anyhow::Error> {
    let working_dir = working_dir.to_path_buf();
    let network_path = working_dir.join(HANEUL_NETWORK_CONFIG);
    let wallet_path = working_dir.join(HANEUL_WALLET_CONFIG);
    let keystore_path = working_dir.join("wallet.key");
    let db_folder_path = working_dir.join("client_db");

    let mut genesis_config =
        genesis_config.unwrap_or(GenesisConfig::default_genesis(&working_dir)?);
    let authorities = genesis_config
        .authorities
        .iter()
        .map(|info| AuthorityPrivateInfo {
            key_pair: info.key_pair.copy(),
            host: info.host.clone(),
            port: 0,
            db_path: info.db_path.clone(),
            stake: info.stake,
            consensus_address: info.consensus_address,
        })
        .collect();
    genesis_config.authorities = authorities;

    let (network_config, accounts, mut keystore) = genesis(genesis_config).await?;
    let network = HaneulNetwork::start(&network_config).await?;

    let network_config = network_config.persisted(&network_path);
    network_config.save()?;
    keystore.set_path(&keystore_path);
    keystore.save()?;

    let authorities = network_config.get_authority_infos();
    let authorities = authorities
        .into_iter()
        .zip(&network.spawned_authorities)
        .map(|(mut info, server)| {
            info.base_port = server.get_port();
            info
        })
        .collect::<Vec<_>>();
    let active_address = accounts.get(0).copied();

    GatewayConfig {
        db_folder_path: db_folder_path.clone(),
        authorities: authorities.clone(),
        ..Default::default()
    }
    .persisted(&working_dir.join(HANEUL_GATEWAY_CONFIG))
    .save()?;

    // Create wallet config with stated authorities port
    WalletConfig {
        accounts,
        keystore: KeystoreType::File(keystore_path),
        gateway: GatewayType::Embedded(GatewayConfig {
            db_folder_path,
            authorities,
            ..Default::default()
        }),
        active_address,
    }
    .persisted(&wallet_path)
    .save()?;

    // Return network handle
    Ok(network)
}
