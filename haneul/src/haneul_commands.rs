// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::{
    config::{
        haneul_config_dir, Config, GatewayConfig, GatewayType, PersistedConfig, WalletConfig,
        HANEUL_GATEWAY_CONFIG, HANEUL_NETWORK_CONFIG, HANEUL_WALLET_CONFIG,
    },
    keystore::{Keystore, KeystoreType, HaneulKeystore},
};
use anyhow::{anyhow, bail};
use base64ct::{Base64, Encoding};
use clap::*;

use std::fs;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use haneul_config::GenesisConfig;
use haneul_config::{builder::ConfigBuilder, NetworkConfig};
use haneul_types::base_types::decode_bytes_hex;
use haneul_types::base_types::HaneulAddress;
use tracing::info;

pub use haneul_core::make::{make_authority, make_server, HaneulNetwork};

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum HaneulCommand {
    /// Start haneul network.
    #[clap(name = "start")]
    Start {
        #[clap(long)]
        config: Option<PathBuf>,
    },
    #[clap(name = "network")]
    Network {
        #[clap(long)]
        config: Option<PathBuf>,
        #[clap(short, long, help = "Dump the public keys of all authorities")]
        dump_addresses: bool,
    },
    #[clap(name = "genesis")]
    Genesis {
        #[clap(long, help = "Start genesis with a given config file")]
        from_config: Option<PathBuf>,
        #[clap(
            long,
            help = "Build a genesis config, write it to the specified path, and exit"
        )]
        write_config: Option<PathBuf>,
        #[clap(long)]
        working_dir: Option<PathBuf>,
        #[clap(short, long, help = "Forces overwriting existing configuration")]
        force: bool,
    },
    #[clap(name = "signtool")]
    SignTool {
        #[clap(long)]
        keystore_path: Option<PathBuf>,
        #[clap(long, parse(try_from_str = decode_bytes_hex))]
        address: HaneulAddress,
        #[clap(long)]
        data: String,
    },
}

impl HaneulCommand {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        match self {
            HaneulCommand::Start { config } => {
                // Load the config of the Haneul authority.
                let network_config_path = config
                    .clone()
                    .unwrap_or(haneul_config_dir()?.join(HANEUL_NETWORK_CONFIG));
                let network_config: NetworkConfig = PersistedConfig::read(&network_config_path)
                    .map_err(|err| {
                        err.context(format!(
                            "Cannot open Haneul network config file at {:?}",
                            network_config_path
                        ))
                    })?;

                // Start a haneul validator (including its consensus node).
                HaneulNetwork::start(&network_config)
                    .await?
                    .wait_for_completion()
                    .await
            }
            HaneulCommand::Network {
                config,
                dump_addresses,
            } => {
                let config_path = config
                    .clone()
                    .unwrap_or(haneul_config_dir()?.join(HANEUL_NETWORK_CONFIG));
                let config: NetworkConfig = PersistedConfig::read(&config_path).map_err(|err| {
                    err.context(format!(
                        "Cannot open Haneul network config file at {:?}",
                        config_path
                    ))
                })?;

                if *dump_addresses {
                    for validator in config.validator_configs() {
                        println!(
                            "{} - {}",
                            validator.network_address(),
                            validator.haneul_address()
                        );
                    }
                }
                Ok(())
            }
            HaneulCommand::Genesis {
                working_dir,
                force,
                from_config,
                write_config,
            } => {
                let haneul_config_dir = &match working_dir {
                    // if a directory is specified, it must exist (it
                    // will not be created)
                    Some(v) => v.clone(),
                    // create default Haneul config dir if not specified
                    // on the command line and if it does not exist
                    // yet
                    None => {
                        let config_path = haneul_config_dir()?;
                        fs::create_dir_all(&config_path)?;
                        config_path
                    }
                };

                // if Haneul config dir is not empty then either clean it
                // up (if --force/-f option was specified or report an
                // error
                if write_config.is_none()
                    && haneul_config_dir
                        .read_dir()
                        .map_err(|err| {
                            anyhow!(err)
                                .context(format!("Cannot open Haneul config dir {:?}", haneul_config_dir))
                        })?
                        .next()
                        .is_some()
                {
                    if *force {
                        fs::remove_dir_all(haneul_config_dir).map_err(|err| {
                            anyhow!(err).context(format!(
                                "Cannot remove Haneul config dir {:?}",
                                haneul_config_dir
                            ))
                        })?;
                        fs::create_dir(haneul_config_dir).map_err(|err| {
                            anyhow!(err).context(format!(
                                "Cannot create Haneul config dir {:?}",
                                haneul_config_dir
                            ))
                        })?;
                    } else {
                        bail!("Cannot run genesis with non-empty Haneul config directory {}, please use --force/-f option to remove existing configuration", haneul_config_dir.to_str().unwrap());
                    }
                }

                let network_path = haneul_config_dir.join(HANEUL_NETWORK_CONFIG);
                let wallet_path = haneul_config_dir.join(HANEUL_WALLET_CONFIG);
                let gateway_path = haneul_config_dir.join(HANEUL_GATEWAY_CONFIG);
                let keystore_path = haneul_config_dir.join("wallet.key");
                let db_folder_path = haneul_config_dir.join("client_db");
                let gateway_db_folder_path = haneul_config_dir.join("gateway_client_db");

                let genesis_conf = match from_config {
                    Some(q) => PersistedConfig::read(q)?,
                    None => GenesisConfig::for_local_testing()?,
                };

                if let Some(path) = write_config {
                    let persisted = genesis_conf.persisted(path);
                    persisted.save()?;
                    return Ok(());
                }

                let network_config = ConfigBuilder::new(haneul_config_dir)
                    .committee_size(NonZeroUsize::new(genesis_conf.committee_size).unwrap())
                    .initial_accounts_config(genesis_conf)
                    .build();

                let mut accounts = Vec::new();
                let mut keystore = HaneulKeystore::default();

                for key in &network_config.account_keys {
                    let address = HaneulAddress::from(key.public_key_bytes());
                    accounts.push(address);
                    keystore.add_key(address, key.copy())?;
                }

                info!("Network genesis completed.");
                let network_config = network_config.persisted(&network_path);
                network_config.save()?;
                info!("Network config file is stored in {:?}.", network_path);
                keystore.set_path(&keystore_path);
                keystore.save()?;
                info!("Wallet keystore is stored in {:?}.", keystore_path);

                // Use the first address if any
                let active_address = accounts.get(0).copied();

                let validator_set = network_config.validator_configs()[0]
                    .committee_config()
                    .validator_set();

                GatewayConfig {
                    db_folder_path: gateway_db_folder_path,
                    validator_set: validator_set.to_owned(),
                    ..Default::default()
                }
                .persisted(&gateway_path)
                .save()?;
                info!("Gateway config file is stored in {:?}.", gateway_path);

                let wallet_gateway_config = GatewayConfig {
                    db_folder_path,
                    validator_set: validator_set.to_owned(),
                    ..Default::default()
                };

                let wallet_config = WalletConfig {
                    accounts,
                    keystore: KeystoreType::File(keystore_path),
                    gateway: GatewayType::Embedded(wallet_gateway_config),
                    active_address,
                };

                let wallet_config = wallet_config.persisted(&wallet_path);
                wallet_config.save()?;
                info!("Wallet config file is stored in {:?}.", wallet_path);

                Ok(())
            }
            HaneulCommand::SignTool {
                keystore_path,
                address,
                data,
            } => {
                let keystore_path = keystore_path
                    .clone()
                    .unwrap_or(haneul_config_dir()?.join("wallet.key"));
                let keystore = HaneulKeystore::load_or_create(&keystore_path)?;
                info!("Data to sign : {}", data);
                info!("Address : {}", address);
                let message = Base64::decode_vec(data).map_err(|e| anyhow!(e))?;
                let signature = keystore.sign(address, &message)?;
                // Separate pub key and signature string, signature and pub key are concatenated with an '@' symbol.
                let signature_string = format!("{:?}", signature);
                let sig_split = signature_string.split('@').collect::<Vec<_>>();
                let signature = sig_split
                    .first()
                    .ok_or_else(|| anyhow!("Error creating signature."))?;
                let pub_key = sig_split
                    .last()
                    .ok_or_else(|| anyhow!("Error creating signature."))?;
                info!("Public Key Base64: {}", pub_key);
                info!("Signature : {}", signature);
                Ok(())
            }
        }
    }
}
