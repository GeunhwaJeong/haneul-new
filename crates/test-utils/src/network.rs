// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::num::NonZeroUsize;
use std::sync::Arc;

use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use jsonrpsee::ws_client::WsClient;
use jsonrpsee::ws_client::WsClientBuilder;
use prometheus::Registry;

use haneullabs_metrics::RegistryService;
use haneul::config::HaneulEnv;
use haneul::{client_commands::WalletContext, config::HaneulClientConfig};
use haneul_config::genesis_config::GenesisConfig;
use haneul_config::{Config, HANEUL_CLIENT_CONFIG, HANEUL_NETWORK_CONFIG};
use haneul_config::{FullnodeConfigBuilder, NodeConfig, PersistedConfig, HANEUL_KEYSTORE_FILENAME};
use haneul_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use haneul_node::HaneulNode;
use haneul_sdk::HaneulClient;
use haneul_swarm::memory::{Swarm, SwarmBuilder};
use haneul_types::base_types::HaneulAddress;
use haneul_types::crypto::KeypairTraits;
use haneul_types::crypto::HaneulKeyPair;

const NUM_VALIDAOTR: usize = 4;

pub struct FullNodeHandle {
    pub haneul_node: Arc<HaneulNode>,
    pub haneul_client: HaneulClient,
    pub rpc_client: HttpClient,
    pub rpc_url: String,
    pub ws_client: WsClient,
    pub ws_url: String,
}

pub struct TestCluster {
    pub swarm: Swarm,
    pub accounts: Vec<HaneulAddress>,
    pub wallet: WalletContext,
    pub fullnode_handle: FullNodeHandle,
}

impl TestCluster {
    pub fn rpc_client(&self) -> &HttpClient {
        &self.fullnode_handle.rpc_client
    }

    pub fn rpc_url(&self) -> &str {
        &self.fullnode_handle.rpc_url
    }

    pub fn wallet_mut(&mut self) -> &mut WalletContext {
        &mut self.wallet
    }

    // Helper function to get the 0th address in WalletContext
    pub fn get_address_0(&self) -> HaneulAddress {
        self.wallet
            .config
            .keystore
            .addresses()
            .get(0)
            .cloned()
            .unwrap()
    }

    // Helper function to get the 1st address in WalletContext
    pub fn get_address_1(&self) -> HaneulAddress {
        self.wallet
            .config
            .keystore
            .addresses()
            .get(1)
            .cloned()
            .unwrap()
    }

    pub fn fullnode_config_builder(&self) -> FullnodeConfigBuilder {
        self.swarm.config().fullnode_config_builder()
    }

    /// Convenience method to start a new fullnode in the test cluster.
    pub async fn start_fullnode(&self) -> Result<FullNodeHandle, anyhow::Error> {
        let config = self.fullnode_config_builder().build().unwrap();
        start_fullnode_from_config(config).await
    }

    pub fn get_validator_addresses(&self) -> Vec<HaneulAddress> {
        self.swarm.validators().map(|v| v.name()).collect()
    }

    pub fn stop_validator(&mut self, name: HaneulAddress) {
        self.swarm.validator_mut(name).unwrap().stop();
    }

    pub async fn start_validator(&mut self, name: HaneulAddress) {
        let node = self.swarm.validator_mut(name).unwrap();
        if node.is_running() {
            return;
        }
        node.start().await.unwrap();
    }
}

pub struct TestClusterBuilder {
    genesis_config: Option<GenesisConfig>,
    num_validators: Option<usize>,
    fullnode_rpc_port: Option<u16>,
    enable_fullnode_events: bool,
}

impl TestClusterBuilder {
    pub fn new() -> Self {
        TestClusterBuilder {
            genesis_config: None,
            fullnode_rpc_port: None,
            num_validators: None,
            enable_fullnode_events: false,
        }
    }

    pub fn set_fullnode_rpc_port(mut self, rpc_port: u16) -> Self {
        self.fullnode_rpc_port = Some(rpc_port);
        self
    }

    pub fn set_genesis_config(mut self, genesis_config: GenesisConfig) -> Self {
        self.genesis_config = Some(genesis_config);
        self
    }

    pub fn with_num_validators(mut self, num: usize) -> Self {
        self.num_validators = Some(num);
        self
    }

    pub fn enable_fullnode_events(mut self) -> Self {
        self.enable_fullnode_events = true;
        self
    }

    pub async fn build(self) -> anyhow::Result<TestCluster> {
        let cluster = self.start_test_network_with_customized_ports().await?;
        Ok(cluster)
    }

    async fn start_test_network_with_customized_ports(
        mut self,
    ) -> Result<TestCluster, anyhow::Error> {
        let swarm = self.start_swarm().await?;
        let working_dir = swarm.dir();

        let mut wallet_conf: HaneulClientConfig =
            PersistedConfig::read(&working_dir.join(HANEUL_CLIENT_CONFIG))?;

        let fullnode_config = swarm
            .config()
            .fullnode_config_builder()
            .set_event_store(self.enable_fullnode_events)
            .set_rpc_port(self.fullnode_rpc_port)
            .build()
            .unwrap();

        let fullnode_handle = start_fullnode_from_config(fullnode_config).await?;

        wallet_conf.envs.push(HaneulEnv {
            alias: "localnet".to_string(),
            rpc: fullnode_handle.rpc_url.clone(),
            ws: Some(fullnode_handle.ws_url.clone()),
        });
        wallet_conf.active_env = Some("localnet".to_string());

        let accounts = wallet_conf.keystore.addresses();

        wallet_conf
            .persisted(&working_dir.join(HANEUL_CLIENT_CONFIG))
            .save()?;

        let wallet_conf = swarm.dir().join(HANEUL_CLIENT_CONFIG);
        let wallet = WalletContext::new(&wallet_conf, None).await?;

        Ok(TestCluster {
            swarm,
            accounts,
            wallet,
            fullnode_handle,
        })
    }

    /// Start a Swarm and set up WalletConfig
    async fn start_swarm(&mut self) -> Result<Swarm, anyhow::Error> {
        let mut builder: SwarmBuilder = Swarm::builder().committee_size(
            NonZeroUsize::new(self.num_validators.unwrap_or(NUM_VALIDAOTR)).unwrap(),
        );

        if let Some(genesis_config) = self.genesis_config.take() {
            builder = builder.initial_accounts_config(genesis_config);
        }

        let mut swarm = builder.build();
        swarm.launch().await?;

        let dir = swarm.dir();

        let network_path = dir.join(HANEUL_NETWORK_CONFIG);
        let wallet_path = dir.join(HANEUL_CLIENT_CONFIG);
        let keystore_path = dir.join(HANEUL_KEYSTORE_FILENAME);

        swarm.config().save(&network_path)?;
        let mut keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
        for key in &swarm.config().account_keys {
            keystore.add_key(HaneulKeyPair::Ed25519(key.copy()))?;
        }

        let active_address = keystore.addresses().first().cloned();

        // Create wallet config with stated authorities port
        HaneulClientConfig {
            keystore: Keystore::from(FileBasedKeystore::new(&keystore_path)?),
            envs: Default::default(),
            active_address,
            active_env: Default::default(),
        }
        .save(&wallet_path)?;

        // Return network handle
        Ok(swarm)
    }
}

impl Default for TestClusterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn start_fullnode_from_config(
    config: NodeConfig,
) -> Result<FullNodeHandle, anyhow::Error> {
    let registry_service = RegistryService::new(Registry::new());
    let haneul_node = HaneulNode::start(&config, registry_service).await?;

    let rpc_url = format!("http://{}", config.json_rpc_address);
    let rpc_client = HttpClientBuilder::default().build(&rpc_url)?;

    let ws_url = format!("ws://{}", config.json_rpc_address);
    let ws_client = WsClientBuilder::default().build(&ws_url).await?;
    let haneul_client = HaneulClient::new(&rpc_url, Some(&ws_url), None).await?;

    Ok(FullNodeHandle {
        haneul_node,
        haneul_client,
        rpc_client,
        rpc_url,
        ws_client,
        ws_url,
    })
}
