// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::cluster::new_wallet_context_from_cluster;

use super::Cluster;
use haneul::client_commands::WalletContext;
use haneul_keys::keystore::AccountKeystore;
use haneul_sdk::HaneulClient;
use haneul_types::base_types::HaneulAddress;
use haneul_types::crypto::{KeypairTraits, Signature};
use haneul_types::messages::TransactionData;
use tracing::{info, info_span, Instrument};

pub struct WalletClient {
    wallet_context: WalletContext,
    address: HaneulAddress,
    fullnode_client: HaneulClient,
}

#[allow(clippy::borrowed_box)]
impl WalletClient {
    pub async fn new_from_cluster(cluster: &(dyn Cluster + Sync + Send)) -> Self {
        let key = cluster.user_key();
        let address: HaneulAddress = key.public().into();
        let wallet_context = new_wallet_context_from_cluster(cluster, key)
            .instrument(info_span!("init_wallet_context_for_test_user"))
            .await;

        let rpc_url = String::from(cluster.fullnode_url());
        info!("Use fullnode rpc: {}", &rpc_url);
        let fullnode_client = HaneulClient::new_rpc_client(&rpc_url, None).await.unwrap();

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

    pub fn get_fullnode_client(&self) -> &HaneulClient {
        &self.fullnode_client
    }

    pub fn sign(&self, txn_data: &TransactionData, desc: &str) -> Signature {
        self.get_wallet()
            .config
            .keystore
            .sign(&self.address, &txn_data.to_bytes())
            .unwrap_or_else(|e| panic!("Failed to sign transaction for {}. {}", desc, e))
    }
}
