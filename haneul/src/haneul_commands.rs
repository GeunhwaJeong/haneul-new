// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::{
    config::{
        make_default_narwhal_committee, haneul_config_dir, AuthorityInfo, AuthorityPrivateInfo,
        Config, GatewayConfig, GatewayType, GenesisConfig, NetworkConfig, PersistedConfig,
        WalletConfig, CONSENSUS_DB_NAME, HANEUL_GATEWAY_CONFIG, HANEUL_NETWORK_CONFIG, HANEUL_WALLET_CONFIG,
    },
    keystore::{Keystore, KeystoreType, HaneulKeystore},
};
use anyhow::{anyhow, bail};
use base64ct::{Base64, Encoding};
use clap::*;
use futures::future::join_all;
use move_binary_format::CompiledModule;
use move_package::BuildConfig;
use narwhal_config::{Committee as ConsensusCommittee, Parameters as ConsensusParameters};
use narwhal_crypto::ed25519::Ed25519PublicKey;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use haneul_adapter::{adapter::generate_package_id, genesis};
use haneul_core::{
    authority::{AuthorityState, AuthorityStore},
    authority_active::ActiveAuthority,
    authority_client::NetworkAuthorityClient,
    authority_server::{AuthorityServer, AuthorityServerHandle},
    consensus_adapter::ConsensusListener,
};
use haneul_network::network::NetworkClient;
use haneul_types::{
    base_types::{decode_bytes_hex, encode_bytes_hex, SequenceNumber, HaneulAddress, TxContext},
    committee::Committee,
    crypto::{random_key_pairs, KeyPair},
    error::HaneulResult,
    object::Object,
};
use tokio::sync::mpsc::channel;
use tracing::{error, info};

pub const HANEUL_AUTHORITY_KEYS: &str = "authorities.key";

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

                let authority_key_path = config
                    .clone()
                    .unwrap_or(haneul_config_dir()?.join(HANEUL_AUTHORITY_KEYS));
                assert!(
                    authority_key_path.exists(),
                    "{:?} does not exist, you may want to re-genesis from scratch",
                    authority_key_path
                );
                let authority_keys = HaneulKeystore::load_or_create(&authority_key_path)?;

                // Start a haneul validator (including its consensus node).
                HaneulNetwork::start(&network_config, authority_keys.key_pairs())
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
                    for auth in config.authorities.iter() {
                        let addr = HaneulAddress::from(&auth.public_key);
                        println!("{}:{} - {}", auth.host, auth.port, addr);
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
                    None => create_genesis_config_from_scratch(haneul_config_dir)?,
                };

                if let Some(path) = write_config {
                    let persisted = genesis_conf.persisted(path);
                    persisted.save()?;
                    return Ok(());
                }

                let (network_config, accounts, mut keystore) = genesis(genesis_conf).await?;
                info!("Network genesis completed.");
                let network_config = network_config.persisted(&network_path);
                network_config.save()?;
                info!("Network config file is stored in {:?}.", network_path);
                keystore.set_path(&keystore_path);
                keystore.save()?;
                info!("Wallet keystore is stored in {:?}.", keystore_path);

                // Use the first address if any
                let active_address = accounts.get(0).copied();

                GatewayConfig {
                    db_folder_path: gateway_db_folder_path,
                    authorities: network_config.get_authority_infos(),
                    ..Default::default()
                }
                .persisted(&gateway_path)
                .save()?;
                info!("Gateway config file is stored in {:?}.", gateway_path);

                let wallet_gateway_config = GatewayConfig {
                    db_folder_path,
                    authorities: network_config.get_authority_infos(),
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

pub struct HaneulNetwork {
    pub spawned_authorities: Vec<AuthorityServerHandle>,
}

impl HaneulNetwork {
    pub async fn start(
        config: &NetworkConfig,
        key_pairs: Vec<&KeyPair>,
    ) -> Result<Self, anyhow::Error> {
        if config.authorities.is_empty() {
            return Err(anyhow!(
                "No authority configured for the network, please run genesis."
            ));
        }
        if config.authorities.len() != key_pairs.len() {
            return Err(anyhow!(
                "Num of authorities does not match num of key_pairs."
            ));
        }

        let key_pairs = key_pairs
            .iter()
            .map(|kp| (kp.public_key_bytes(), kp))
            .collect::<HashMap<_, _>>();

        info!(
            "Starting network with {} authorities",
            config.authorities.len()
        );

        let committee = Committee::new(
            config.epoch,
            config
                .authorities
                .iter()
                .map(|info| (info.public_key, info.stake))
                .collect(),
        );

        let consensus_committee = make_default_narwhal_committee(&config.authorities)?;
        let consensus_parameters = ConsensusParameters::default();

        // Pass in the newtwork parameters of all authorities
        let net = config.get_authority_infos();

        let mut spawned_authorities = Vec::new();

        for authority in &config.authorities {
            let key_pair = key_pairs.get(&authority.public_key).unwrap_or_else(|| {
                panic!(
                    "Can't find key pair for authority {:?}",
                    &authority.public_key
                )
            });
            let consensus_store_path = haneul_config_dir()?
                .join(CONSENSUS_DB_NAME)
                .join(encode_bytes_hex(&authority.public_key));

            let server = make_server(
                authority,
                key_pair,
                &committee,
                config.buffer_size,
                &consensus_committee,
                &consensus_store_path,
                &consensus_parameters,
                Some(net.clone()),
            )
            .await?;
            spawned_authorities.push(server.spawn().await?);
        }
        info!("Started {} authorities", spawned_authorities.len());

        Ok(Self {
            spawned_authorities,
        })
    }

    pub async fn kill(self) -> Result<(), anyhow::Error> {
        for spawned_server in self.spawned_authorities {
            spawned_server.kill().await?;
        }
        Ok(())
    }

    pub async fn wait_for_completion(self) -> Result<(), anyhow::Error> {
        let mut handles = Vec::new();
        for spawned_server in self.spawned_authorities {
            handles.push(async move {
                if let Err(err) = spawned_server.join().await {
                    error!("Server ended with an error: {err}");
                }
            });
        }
        join_all(handles).await;
        info!("All servers stopped.");
        Ok(())
    }
}

pub async fn genesis(
    genesis_conf: GenesisConfig,
) -> Result<(NetworkConfig, Vec<HaneulAddress>, HaneulKeystore), anyhow::Error> {
    info!(
        "Creating {} new authorities...",
        genesis_conf.authorities.len()
    );

    let mut network_config = NetworkConfig {
        epoch: 0,
        authorities: vec![],
        buffer_size: 650000,
        loaded_move_packages: vec![],
        key_pair: genesis_conf.key_pair,
    };
    let mut voting_right = BTreeMap::new();
    for authority in genesis_conf.authorities {
        voting_right.insert(authority.public_key, authority.stake);
        network_config.authorities.push(authority);
    }

    let mut addresses = Vec::new();
    let mut preload_modules: Vec<Vec<CompiledModule>> = Vec::new();
    let mut preload_objects = Vec::new();

    info!("Creating accounts and gas objects...",);

    let mut keystore = HaneulKeystore::default();
    for account in genesis_conf.accounts {
        let address = if let Some(address) = account.address {
            address
        } else {
            keystore.add_random_key()?
        };

        addresses.push(address);

        for object_conf in account.gas_objects {
            let new_object = Object::with_id_owner_gas_coin_object_for_testing(
                object_conf.object_id,
                SequenceNumber::new(),
                address,
                object_conf.gas_value,
            );
            preload_objects.push(new_object);
        }
    }

    info!(
        "Loading Move framework lib from {:?}",
        genesis_conf.move_framework_lib_path
    );
    let move_lib = haneul_framework::get_move_stdlib_modules(&genesis_conf.move_framework_lib_path)?;
    preload_modules.push(move_lib);

    // Load Haneul and Move framework lib
    info!(
        "Loading Haneul framework lib from {:?}",
        genesis_conf.haneul_framework_lib_path
    );
    let haneul_lib = haneul_framework::get_haneul_framework_modules(&genesis_conf.haneul_framework_lib_path)?;
    preload_modules.push(haneul_lib);

    // TODO: allow custom address to be used after the Gateway refactoring
    // Default to use the last address in the wallet config for initializing modules.
    // If there's no address in wallet config, then use 0x0
    let null_address = HaneulAddress::default();
    let module_init_address = addresses.last().unwrap_or(&null_address);
    let mut genesis_ctx = genesis::get_genesis_context_with_custom_address(module_init_address);
    // Build custom move packages
    if !genesis_conf.move_packages.is_empty() {
        info!(
            "Loading {} Move packages from {:?}",
            &genesis_conf.move_packages.len(),
            &genesis_conf.move_packages
        );

        for path in genesis_conf.move_packages {
            let mut modules =
                haneul_framework::build_move_package(&path, BuildConfig::default(), false)?;

            let package_id = generate_package_id(&mut modules, &mut genesis_ctx)?;

            info!("Loaded package [{}] from {:?}.", package_id, path);
            // Writing package id to network config for user to retrieve later.
            network_config.loaded_move_packages.push((path, package_id));
            preload_modules.push(modules)
        }
    }

    let committee = Committee::new(network_config.epoch, voting_right);
    for authority in &network_config.authorities {
        make_server_with_genesis_ctx(
            authority,
            &network_config.key_pair,
            &committee,
            preload_modules.clone(),
            &preload_objects,
            network_config.buffer_size,
            &mut genesis_ctx.clone(),
        )
        .await?;
    }

    Ok((network_config, addresses, keystore))
}

pub async fn make_server(
    authority: &AuthorityPrivateInfo,
    key_pair: &KeyPair,
    committee: &Committee,
    buffer_size: usize,
    consensus_committee: &ConsensusCommittee<Ed25519PublicKey>,
    consensus_store_path: &Path,
    consensus_parameters: &ConsensusParameters,
    net_parameters: Option<Vec<AuthorityInfo>>,
) -> HaneulResult<AuthorityServer> {
    let store = Arc::new(AuthorityStore::open(&authority.db_path, None));
    let name = authority.public_key;
    let state = AuthorityState::new_without_genesis(
        committee.clone(),
        name,
        Arc::pin(key_pair.copy()),
        store,
    )
    .await;

    make_authority(
        authority,
        key_pair,
        buffer_size,
        state,
        consensus_committee,
        consensus_store_path,
        consensus_parameters,
        net_parameters,
    )
    .await
}

async fn make_server_with_genesis_ctx(
    authority: &AuthorityPrivateInfo,
    key_pair: &KeyPair,
    committee: &Committee,
    preload_modules: Vec<Vec<CompiledModule>>,
    preload_objects: &[Object],
    buffer_size: usize,
    genesis_ctx: &mut TxContext,
) -> HaneulResult<AuthorityServer> {
    let store = Arc::new(AuthorityStore::open(&authority.db_path, None));
    let name = authority.public_key;

    let state = AuthorityState::new(
        committee.clone(),
        name,
        Arc::pin(key_pair.copy()),
        store,
        preload_modules,
        genesis_ctx,
    )
    .await;

    for object in preload_objects {
        state.insert_genesis_object(object.clone()).await;
    }

    let (tx_haneul_to_consensus, _rx_haneul_to_consensus) = channel(1);
    Ok(AuthorityServer::new(
        authority.host.clone(),
        authority.port,
        buffer_size,
        Arc::new(state),
        authority.consensus_address,
        /* tx_consensus_listener */ tx_haneul_to_consensus,
    ))
}

/// Spawn all the subsystems run by a Haneul authority: a consensus node, a haneul authority server,
/// and a consensus listener bridging the consensus node and the haneul authority.
pub async fn make_authority(
    authority: &AuthorityPrivateInfo,
    key_pair: &KeyPair,
    buffer_size: usize,
    state: AuthorityState,
    consensus_committee: &ConsensusCommittee<Ed25519PublicKey>,
    consensus_store_path: &Path,
    consensus_parameters: &ConsensusParameters,
    net_parameters: Option<Vec<AuthorityInfo>>,
) -> HaneulResult<AuthorityServer> {
    let (tx_consensus_to_haneul, rx_consensus_to_haneul) = channel(1_000);
    let (tx_haneul_to_consensus, rx_haneul_to_consensus) = channel(1_000);

    let authority_state = Arc::new(state);

    // Spawn the consensus node of this authority.
    let consensus_keypair = key_pair.make_narwhal_keypair();
    let consensus_name = consensus_keypair.name.clone();
    let consensus_store = narwhal_node::NodeStorage::reopen(consensus_store_path);
    narwhal_node::Node::spawn_primary(
        consensus_keypair,
        consensus_committee.clone(),
        &consensus_store,
        consensus_parameters.clone(),
        /* consensus */ true, // Indicate that we want to run consensus.
        /* execution_state */ authority_state.clone(),
        /* tx_confirmation */ tx_consensus_to_haneul,
    )
    .await?;
    narwhal_node::Node::spawn_workers(
        consensus_name,
        /* ids */ vec![0], // We run a single worker with id '0'.
        consensus_committee.clone(),
        &consensus_store,
        consensus_parameters.clone(),
    );

    // Spawn a consensus listener. It listen for consensus outputs and notifies the
    // authority server when a sequenced transaction is ready for execution.
    ConsensusListener::spawn(
        rx_haneul_to_consensus,
        rx_consensus_to_haneul,
        /* max_pending_transactions */ 1_000_000,
    );

    // If we have network information make authority clients
    // to all authorities in the system.
    let _active_authority = if let Some(network) = net_parameters {
        let mut authority_clients = BTreeMap::new();
        for info in &network {
            let client = NetworkAuthorityClient::new(NetworkClient::new(
                info.host.clone(),
                info.base_port,
                buffer_size,
                Duration::from_secs(5),
                Duration::from_secs(5),
            ));
            authority_clients.insert(info.name, client);
        }

        let active_authority = ActiveAuthority::new(authority_state.clone(), authority_clients)?;

        let join_handle = active_authority.spawn_all_active_processes().await;
        Some(join_handle)
    } else {
        None
    };

    // Return new authority server. It listen to users transactions and send back replies.
    Ok(AuthorityServer::new(
        authority.host.clone(),
        authority.port,
        buffer_size,
        authority_state,
        authority.consensus_address,
        /* tx_consensus_listener */ tx_haneul_to_consensus,
    ))
}

/// Generate a genesis config
/// Side effect: create an authorities.key file that contains all authority key pairs.
///              the file is only for local testing's convenience, and not supposed to
///              exist in testnet/mainnet.
fn create_genesis_config_from_scratch(
    haneul_config_dir: &Path,
) -> Result<GenesisConfig, anyhow::Error> {
    let authority_key_pairs_path = haneul_config_dir.join(HANEUL_AUTHORITY_KEYS);
    let key_pairs = random_key_pairs(4);
    let mut authority_key_store = HaneulKeystore::default();
    authority_key_store.set_path(&authority_key_pairs_path);
    let key_pair = key_pairs[0].copy();
    let public_keys = key_pairs
        .iter()
        .map(|kp| *kp.public_key_bytes())
        .collect::<Vec<_>>();
    for key_pair in key_pairs {
        authority_key_store.add_key(HaneulAddress::from(key_pair.public_key_bytes()), key_pair)?;
    }
    authority_key_store.save()?;
    info!(
        "Authority keystore is stored in {:?}.",
        authority_key_pairs_path
    );
    GenesisConfig::default_genesis(haneul_config_dir, Some((public_keys, key_pair)))
}
