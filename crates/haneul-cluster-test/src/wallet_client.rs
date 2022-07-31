// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::Cluster;
use haneul::client_commands::WalletContext;
use haneul::config::{Config, HaneulClientConfig};
use haneul_config::HANEUL_KEYSTORE_FILENAME;
use haneul_sdk::crypto::KeystoreType;
use haneul_sdk::{ClientType, HaneulClient};
use haneul_types::base_types::HaneulAddress;
use haneul_types::crypto::{KeypairTraits, Signature};
use haneul_types::messages::TransactionData;
use tracing::info;

pub struct WalletClient {
    wallet_context: WalletContext,
    address: HaneulAddress,
    fullnode_client: HaneulClient,
}

#[allow(clippy::borrowed_box)]
impl WalletClient {
    pub async fn new_from_cluster(cluster: &Box<dyn Cluster + Sync + Send>) -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        let wallet_config_path = temp_dir.path().join("client.yaml");
        let rpc_url = cluster.rpc_url();
        info!("Use gateway: {}", &rpc_url);
        let keystore_path = temp_dir.path().join(HANEUL_KEYSTORE_FILENAME);
        let keystore = KeystoreType::File(keystore_path);
        let key_pair = cluster.user_key();
        let address: HaneulAddress = key_pair.public().into();
        keystore.init().unwrap().add_key(key_pair).unwrap();
        HaneulClientConfig {
            accounts: vec![address],
            keystore,
            gateway: ClientType::RPC(rpc_url.into()),
            active_address: Some(address),
        }
        .persisted(&wallet_config_path)
        .save()
        .unwrap();

        info!(
            "Initialize wallet from config path: {:?}",
            wallet_config_path
        );

        let wallet_context = WalletContext::new(&wallet_config_path)
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to init wallet context from path {:?}, error: {e}",
                    wallet_config_path
                )
            });

        let fullnode_url = String::from(cluster.fullnode_url());
        info!("Use fullnode: {}", &fullnode_url);
        let fullnode_client = HaneulClient::new_http_client(&fullnode_url).unwrap();

        Self {
            wallet_context,
            address,
            fullnode_client,
        }
    }

    pub fn get_wallet(&self) -> &WalletContext {
        &self.wallet_context
    }

    pub fn get_wallet_mut(&mut self) -> &mut WalletContext {
        &mut self.wallet_context
    }

    pub fn get_wallet_address(&self) -> HaneulAddress {
        self.address
    }

    pub fn get_gateway(&self) -> &HaneulClient {
        &self.wallet_context.gateway
    }

    pub fn get_fullnode(&self) -> &HaneulClient {
        &self.fullnode_client
    }

    pub async fn sync_account_state(&self) -> Result<(), anyhow::Error> {
        self.get_gateway()
            .sync_account_state(self.get_wallet_address())
            .await
    }

    pub fn sign(&self, txn_data: &TransactionData, desc: &str) -> Signature {
        self.get_wallet()
            .keystore
            .sign(&self.address, &txn_data.to_bytes())
            .unwrap_or_else(|e| panic!("Failed to sign transaction for {}. {}", desc, e))
    }
}
