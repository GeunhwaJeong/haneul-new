// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::io::{stderr, stdout, Write};
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::{fs, io};

use anyhow::{anyhow, bail};
use clap::*;
use fastcrypto::traits::KeyPair;
use move_package::BuildConfig;
use haneul_framework_build::compiled_package::HaneulPackageHooks;
use tracing::info;

use haneul_config::{builder::ConfigBuilder, NetworkConfig, HANEUL_KEYSTORE_FILENAME};
use haneul_config::{genesis_config::GenesisConfig, HANEUL_GENESIS_FILENAME};
use haneul_config::{
    haneul_config_dir, Config, PersistedConfig, FULL_NODE_DB_PATH, HANEUL_CLIENT_CONFIG,
    HANEUL_FULLNODE_CONFIG, HANEUL_NETWORK_CONFIG,
};
use haneul_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use haneul_swarm::memory::Swarm;
use haneul_types::crypto::{SignatureScheme, HaneulKeyPair};

use crate::client_commands::{HaneulClientCommands, WalletContext};
use crate::config::{HaneulClientConfig, HaneulEnv};
use crate::console::start_console;
use crate::fire_drill::{run_fire_drill, FireDrill};
use crate::genesis_ceremony::{run, Ceremony};
use crate::keytool::KeyToolCommand;
use haneul_move::{self, execute_move_command};

#[allow(clippy::large_enum_variant)]
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum HaneulCommand {
    /// Start haneul network.
    #[clap(name = "start")]
    Start {
        #[clap(long = "network.config")]
        config: Option<PathBuf>,
        #[clap(long = "no-full-node")]
        no_full_node: bool,
    },
    #[clap(name = "network")]
    Network {
        #[clap(long = "network.config")]
        config: Option<PathBuf>,
        #[clap(short, long, help = "Dump the public keys of all authorities")]
        dump_addresses: bool,
    },
    /// Bootstrap and initialize a new haneul network
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
        #[clap(long = "epoch-duration-ms")]
        epoch_duration_ms: Option<u64>,
    },
    GenesisCeremony(Ceremony),
    /// Haneul keystore tool.
    #[clap(name = "keytool")]
    KeyTool {
        #[clap(long)]
        keystore_path: Option<PathBuf>,
        /// Subcommands.
        #[clap(subcommand)]
        cmd: KeyToolCommand,
    },
    /// Start Haneul interactive console.
    #[clap(name = "console")]
    Console {
        /// Sets the file storing the state of our user accounts (an empty one will be created if missing)
        #[clap(long = "client.config")]
        config: Option<PathBuf>,
    },
    /// Client for interacting with the Haneul network.
    #[clap(name = "client")]
    Client {
        /// Sets the file storing the state of our user accounts (an empty one will be created if missing)
        #[clap(long = "client.config")]
        config: Option<PathBuf>,
        #[clap(subcommand)]
        cmd: Option<HaneulClientCommands>,
        /// Return command outputs in json format.
        #[clap(long, global = true)]
        json: bool,
        #[clap(short = 'y', long = "yes")]
        accept_defaults: bool,
    },

    /// Tool to build and test Move applications.
    #[clap(name = "move")]
    Move {
        /// Path to a package which the command should be run with respect to.
        #[clap(long = "path", short = 'p', global = true, parse(from_os_str))]
        package_path: Option<PathBuf>,
        /// Package build options
        #[clap(flatten)]
        build_config: BuildConfig,
        /// Subcommands.
        #[clap(subcommand)]
        cmd: haneul_move::Command,
    },

    /// Tool for Fire Drill
    FireDrill {
        #[clap(subcommand)]
        fire_drill: FireDrill,
    },
}

impl HaneulCommand {
    pub async fn execute(self) -> Result<(), anyhow::Error> {
        move_package::package_hooks::register_package_hooks(Box::new(HaneulPackageHooks {}));
        match self {
            HaneulCommand::Start {
                config,
                no_full_node,
            } => {
                // Auto genesis if path is none and haneul directory doesn't exists.
                if config.is_none() && !haneul_config_dir()?.join(HANEUL_NETWORK_CONFIG).exists() {
                    genesis(None, None, None, false, None).await?;
                }

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

                let mut swarm = if no_full_node {
                    Swarm::builder()
                } else {
                    Swarm::builder()
                        .with_fullnode_rpc_addr(haneul_config::node::default_json_rpc_address())
                        .with_event_store()
                }
                .from_network_config(haneul_config_dir()?, network_config);

                swarm.launch().await?;

                let mut interval = tokio::time::interval(std::time::Duration::from_secs(3));
                let mut unhealthy_cnt = 0;
                loop {
                    for node in swarm.validators() {
                        if let Err(err) = node.health_check(true).await {
                            unhealthy_cnt += 1;
                            if unhealthy_cnt > 3 {
                                // The network could temporarily go down during reconfiguration.
                                // If we detect a failed validator 3 times in a row, give up.
                                return Err(err.into());
                            }
                            // Break the inner loop so that we could retry latter.
                            break;
                        } else {
                            unhealthy_cnt = 0;
                        }
                    }

                    interval.tick().await;
                }
            }
            HaneulCommand::Network {
                config,
                dump_addresses,
            } => {
                let config_path = config.unwrap_or(haneul_config_dir()?.join(HANEUL_NETWORK_CONFIG));
                let config: NetworkConfig = PersistedConfig::read(&config_path).map_err(|err| {
                    err.context(format!(
                        "Cannot open Haneul network config file at {:?}",
                        config_path
                    ))
                })?;

                if dump_addresses {
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
                epoch_duration_ms,
            } => {
                genesis(
                    from_config,
                    write_config,
                    working_dir,
                    force,
                    epoch_duration_ms,
                )
                .await
            }
            HaneulCommand::GenesisCeremony(cmd) => run(cmd),
            HaneulCommand::KeyTool { keystore_path, cmd } => {
                let keystore_path =
                    keystore_path.unwrap_or(haneul_config_dir()?.join(HANEUL_KEYSTORE_FILENAME));
                let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
                cmd.execute(&mut keystore)
            }
            HaneulCommand::Console { config } => {
                let config = config.unwrap_or(haneul_config_dir()?.join(HANEUL_CLIENT_CONFIG));
                prompt_if_no_config(&config, false).await?;
                let context = WalletContext::new(&config, None).await?;
                start_console(context, &mut stdout(), &mut stderr()).await
            }
            HaneulCommand::Client {
                config,
                cmd,
                json,
                accept_defaults,
            } => {
                let config_path = config.unwrap_or(haneul_config_dir()?.join(HANEUL_CLIENT_CONFIG));
                prompt_if_no_config(&config_path, accept_defaults).await?;
                let mut context = WalletContext::new(&config_path, None).await?;
                if let Some(cmd) = cmd {
                    cmd.execute(&mut context).await?.print(!json);
                } else {
                    // Print help
                    let mut app: Command = HaneulCommand::command();
                    app.build();
                    app.find_subcommand_mut("client").unwrap().print_help()?;
                }
                Ok(())
            }
            HaneulCommand::Move {
                package_path,
                build_config,
                cmd,
            } => execute_move_command(package_path, build_config, cmd),
            HaneulCommand::FireDrill { fire_drill } => run_fire_drill(fire_drill).await,
        }
    }
}

async fn genesis(
    from_config: Option<PathBuf>,
    write_config: Option<PathBuf>,
    working_dir: Option<PathBuf>,
    force: bool,
    epoch_duration_ms: Option<u64>,
) -> Result<(), anyhow::Error> {
    let haneul_config_dir = &match working_dir {
        // if a directory is specified, it must exist (it
        // will not be created)
        Some(v) => v,
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
    let dir = haneul_config_dir.read_dir().map_err(|err| {
        anyhow!(err).context(format!("Cannot open Haneul config dir {:?}", haneul_config_dir))
    })?;
    let files = dir.collect::<Result<Vec<_>, _>>()?;

    let client_path = haneul_config_dir.join(HANEUL_CLIENT_CONFIG);
    let keystore_path = haneul_config_dir.join(HANEUL_KEYSTORE_FILENAME);

    if write_config.is_none() && !files.is_empty() {
        if force {
            // check old keystore and client.yaml is compatible
            let is_compatible = FileBasedKeystore::new(&keystore_path).is_ok()
                && PersistedConfig::<HaneulClientConfig>::read(&client_path).is_ok();
            // Keep keystore and client.yaml if they are compatible
            if is_compatible {
                for file in files {
                    let path = file.path();
                    if path != client_path && path != keystore_path {
                        if path.is_file() {
                            fs::remove_file(path)
                        } else {
                            fs::remove_dir_all(path)
                        }
                        .map_err(|err| {
                            anyhow!(err).context(format!("Cannot remove file {:?}", file.path()))
                        })?;
                    }
                }
            } else {
                fs::remove_dir_all(haneul_config_dir).map_err(|err| {
                    anyhow!(err)
                        .context(format!("Cannot remove Haneul config dir {:?}", haneul_config_dir))
                })?;
                fs::create_dir(haneul_config_dir).map_err(|err| {
                    anyhow!(err)
                        .context(format!("Cannot create Haneul config dir {:?}", haneul_config_dir))
                })?;
            }
        } else if files.len() != 2 || !client_path.exists() || !keystore_path.exists() {
            bail!("Cannot run genesis with non-empty Haneul config directory {}, please use --force/-f option to remove existing configuration", haneul_config_dir.to_str().unwrap());
        }
    }

    let network_path = haneul_config_dir.join(HANEUL_NETWORK_CONFIG);
    let genesis_path = haneul_config_dir.join(HANEUL_GENESIS_FILENAME);

    let mut genesis_conf = match from_config {
        Some(path) => PersistedConfig::read(&path)?,
        None => {
            if keystore_path.exists() {
                let existing_keys = FileBasedKeystore::new(&keystore_path)?.addresses();
                GenesisConfig::for_local_testing_with_addresses(existing_keys)
            } else {
                GenesisConfig::for_local_testing()
            }
        }
    };

    if let Some(path) = write_config {
        let persisted = genesis_conf.persisted(&path);
        persisted.save()?;
        return Ok(());
    }

    let validator_info = genesis_conf.validator_config_info.take();
    let builder = ConfigBuilder::new(haneul_config_dir);
    if let Some(epoch_duration_ms) = epoch_duration_ms {
        genesis_conf.parameters.epoch_duration_ms = epoch_duration_ms;
    }
    let mut network_config = if let Some(validators) = validator_info {
        builder
            .initial_accounts_config(genesis_conf)
            .with_validators(validators)
            .build()
    } else {
        builder
            .committee_size(NonZeroUsize::new(genesis_conf.committee_size).unwrap())
            .initial_accounts_config(genesis_conf)
            .build()
    };

    let mut keystore = FileBasedKeystore::new(&keystore_path)?;
    for key in &network_config.account_keys {
        keystore.add_key(HaneulKeyPair::Ed25519(key.copy()))?;
    }
    let active_address = keystore.addresses().pop();

    network_config.genesis.save(&genesis_path)?;
    for validator in &mut network_config.validator_configs {
        validator.genesis = haneul_config::node::Genesis::new_from_file(&genesis_path);
    }

    info!("Network genesis completed.");
    network_config.save(&network_path)?;
    info!("Network config file is stored in {:?}.", network_path);

    info!("Client keystore is stored in {:?}.", keystore_path);

    let mut fullnode_config = network_config
        .fullnode_config_builder()
        .with_event_store()
        .with_dir(FULL_NODE_DB_PATH.into())
        .build()?;

    fullnode_config.json_rpc_address = haneul_config::node::default_json_rpc_address();
    fullnode_config.save(haneul_config_dir.join(HANEUL_FULLNODE_CONFIG))?;

    for (i, validator) in network_config
        .into_validator_configs()
        .into_iter()
        .enumerate()
    {
        let path = haneul_config_dir.join(format!("validator-config-{}.yaml", i));
        validator.save(path)?;
    }

    let mut client_config = if client_path.exists() {
        PersistedConfig::read(&client_path)?
    } else {
        HaneulClientConfig::new(keystore.into())
    };

    if client_config.active_address.is_none() {
        client_config.active_address = active_address;
    }
    client_config.add_env(HaneulEnv {
        alias: "localnet".to_string(),
        rpc: format!("http://{}", fullnode_config.json_rpc_address),
        ws: None,
    });
    client_config.add_env(HaneulEnv::devnet());

    if client_config.active_env.is_none() {
        client_config.active_env = client_config.envs.first().map(|env| env.alias.clone());
    }

    client_config.save(&client_path)?;
    info!("Client config file is stored in {:?}.", client_path);

    Ok(())
}

async fn prompt_if_no_config(
    wallet_conf_path: &Path,
    accept_defaults: bool,
) -> Result<(), anyhow::Error> {
    // Prompt user for connect to devnet fullnode if config does not exist.
    if !wallet_conf_path.exists() {
        let env = match std::env::var_os("HANEUL_CONFIG_WITH_RPC_URL") {
            Some(v) => Some(HaneulEnv {
                alias: "custom".to_string(),
                rpc: v.into_string().unwrap(),
                ws: None,
            }),
            None => {
                if accept_defaults {
                    print!("Creating config file [{:?}] with default (devnet) full node server and ed25519 key scheme.", wallet_conf_path);
                } else {
                    print!(
                        "Config file [{:?}] doesn't exist, do you want to connect to a Haneul full node server [yN]?",
                        wallet_conf_path
                    );
                }
                if accept_defaults
                    || matches!(read_line(), Ok(line) if line.trim().to_lowercase() == "y")
                {
                    let url = if accept_defaults {
                        String::new()
                    } else {
                        print!(
                            "Haneul full node server url (Default to Haneul DevNet if not specified) : "
                        );
                        read_line()?
                    };
                    Some(if url.trim().is_empty() {
                        HaneulEnv::devnet()
                    } else {
                        print!("Environment alias for [{url}] : ");
                        let alias = read_line()?;
                        let alias = if alias.trim().is_empty() {
                            "custom".to_string()
                        } else {
                            alias
                        };
                        HaneulEnv {
                            alias,
                            rpc: url,
                            ws: None,
                        }
                    })
                } else {
                    None
                }
            }
        };

        if let Some(env) = env {
            let keystore_path = wallet_conf_path
                .parent()
                .unwrap_or(&haneul_config_dir()?)
                .join(HANEUL_KEYSTORE_FILENAME);
            let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
            let key_scheme = if accept_defaults {
                SignatureScheme::ED25519
            } else {
                println!("Select key scheme to generate keypair (0 for ed25519, 1 for secp256k1, 2: for secp256r1:");
                match SignatureScheme::from_flag(read_line()?.trim()) {
                    Ok(s) => s,
                    Err(e) => return Err(anyhow!("{e}")),
                }
            };
            let (new_address, phrase, scheme) =
                keystore.generate_and_add_new_key(key_scheme, None)?;
            println!(
                "Generated new keypair for address with scheme {:?} [{new_address}]",
                scheme.to_string()
            );
            println!("Secret Recovery Phrase : [{phrase}]");
            let alias = env.alias.clone();
            HaneulClientConfig {
                keystore,
                envs: vec![env],
                active_address: Some(new_address),
                active_env: Some(alias),
            }
            .persisted(wallet_conf_path)
            .save()?;
        }
    }
    Ok(())
}

fn read_line() -> Result<String, anyhow::Error> {
    let mut s = String::new();
    let _ = stdout().flush();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim_end().to_string())
}
