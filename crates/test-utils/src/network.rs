// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee_http_client::{HttpClient, HttpClientBuilder};
use jsonrpsee_http_server::{HttpServerBuilder, HttpServerHandle, RpcModule};
use std::net::SocketAddr;
use std::num::NonZeroUsize;
use std::path::Path;
use haneul::{
    client_commands::{HaneulClientCommands, WalletContext},
    config::{GatewayConfig, GatewayType, HaneulClientConfig},
};
use haneul_config::genesis_config::GenesisConfig;
use haneul_config::{Config, HANEUL_CLIENT_CONFIG, HANEUL_GATEWAY_CONFIG, HANEUL_NETWORK_CONFIG};
use haneul_config::{PersistedConfig, HANEUL_KEYSTORE_FILENAME};
use haneul_core::gateway_state::GatewayMetrics;
use haneul_gateway::create_client;
use haneul_json_rpc::gateway_api::{
    GatewayReadApiImpl, GatewayWalletSyncApiImpl, RpcGatewayImpl, TransactionBuilderImpl,
};
use haneul_json_rpc_api::keystore::{KeystoreType, HaneulKeystore};
use haneul_json_rpc_api::QuorumDriverApiServer;
use haneul_json_rpc_api::RpcReadApiServer;
use haneul_json_rpc_api::RpcTransactionBuilderServer;
use haneul_json_rpc_api::WalletSyncApiServer;
use haneul_swarm::memory::Swarm;
use haneul_types::base_types::HaneulAddress;
const NUM_VALIDAOTR: usize = 4;

pub async fn start_test_network(
    genesis_config: Option<GenesisConfig>,
) -> Result<Swarm, anyhow::Error> {
    let mut builder = Swarm::builder().committee_size(NonZeroUsize::new(NUM_VALIDAOTR).unwrap());
    if let Some(genesis_config) = genesis_config {
        builder = builder.initial_accounts_config(genesis_config);
    }

    let mut swarm = builder.build();
    swarm.launch().await?;

    let accounts = swarm
        .config()
        .account_keys
        .iter()
        .map(|key| HaneulAddress::from(key.public_key_bytes()))
        .collect::<Vec<_>>();

    let dir = swarm.dir();

    let network_path = dir.join(HANEUL_NETWORK_CONFIG);
    let wallet_path = dir.join(HANEUL_CLIENT_CONFIG);
    let keystore_path = dir.join(HANEUL_KEYSTORE_FILENAME);
    let db_folder_path = dir.join("client_db");
    let gateway_path = dir.join(HANEUL_GATEWAY_CONFIG);

    swarm.config().save(&network_path)?;
    let mut keystore = HaneulKeystore::default();
    for key in &swarm.config().account_keys {
        keystore.add_key(HaneulAddress::from(key.public_key_bytes()), key.copy())?;
    }
    keystore.set_path(&keystore_path);
    keystore.save()?;

    let validators = swarm.config().validator_set().to_owned();
    let active_address = accounts.get(0).copied();

    GatewayConfig {
        db_folder_path: db_folder_path.clone(),
        validator_set: validators.clone(),
        ..Default::default()
    }
    .save(gateway_path)?;

    // Create wallet config with stated authorities port
    HaneulClientConfig {
        accounts,
        keystore: KeystoreType::File(keystore_path),
        gateway: GatewayType::Embedded(GatewayConfig {
            db_folder_path,
            validator_set: validators,
            ..Default::default()
        }),
        active_address,
    }
    .save(&wallet_path)?;

    // Return network handle
    Ok(swarm)
}

pub async fn setup_network_and_wallet() -> Result<(Swarm, WalletContext, HaneulAddress), anyhow::Error>
{
    let swarm = start_test_network(None).await?;

    // Create Wallet context.
    let wallet_conf = swarm.dir().join(HANEUL_CLIENT_CONFIG);
    let mut context = WalletContext::new(&wallet_conf)?;
    let address = context.config.accounts.first().cloned().unwrap();

    // Sync client to retrieve objects from the network.
    HaneulClientCommands::SyncClientState {
        address: Some(address),
    }
    .execute(&mut context)
    .await?;
    Ok((swarm, context, address))
}

async fn start_rpc_gateway(
    config_path: &Path,
) -> Result<(SocketAddr, HttpServerHandle), anyhow::Error> {
    let server = HttpServerBuilder::default().build("127.0.0.1:0").await?;
    let addr = server.local_addr()?;
    let metrics = GatewayMetrics::new(&prometheus::Registry::new());
    let client = create_client(config_path, metrics)?;
    let mut module = RpcModule::new(());
    module.merge(RpcGatewayImpl::new(client.clone()).into_rpc())?;
    module.merge(GatewayReadApiImpl::new(client.clone()).into_rpc())?;
    module.merge(TransactionBuilderImpl::new(client.clone()).into_rpc())?;
    module.merge(GatewayWalletSyncApiImpl::new(client.clone()).into_rpc())?;

    let handle = server.start(module)?;
    Ok((addr, handle))
}

pub async fn start_rpc_test_network(
    genesis_config: Option<GenesisConfig>,
) -> Result<TestNetwork, anyhow::Error> {
    let network = start_test_network(genesis_config).await?;
    let working_dir = network.dir();
    let (server_addr, rpc_server_handle) =
        start_rpc_gateway(&working_dir.join(HANEUL_GATEWAY_CONFIG)).await?;
    let mut wallet_conf: HaneulClientConfig =
        PersistedConfig::read(&working_dir.join(HANEUL_CLIENT_CONFIG))?;
    let rpc_url = format!("http://{}", server_addr);
    let accounts = wallet_conf.accounts.clone();
    wallet_conf.gateway = GatewayType::RPC(rpc_url.clone());
    wallet_conf
        .persisted(&working_dir.join(HANEUL_CLIENT_CONFIG))
        .save()?;

    let http_client = HttpClientBuilder::default().build(rpc_url.clone())?;
    Ok(TestNetwork {
        network,
        _rpc_server: rpc_server_handle,
        accounts,
        http_client,
        rpc_url,
    })
}

pub struct TestNetwork {
    pub network: Swarm,
    _rpc_server: HttpServerHandle,
    pub accounts: Vec<HaneulAddress>,
    pub http_client: HttpClient,
    pub rpc_url: String,
}
