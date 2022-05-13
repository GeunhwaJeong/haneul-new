// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul::{
    config::HANEUL_WALLET_CONFIG,
    haneul_commands::HaneulNetwork,
    wallet_commands::{WalletCommands, WalletContext},
};
use haneul_types::base_types::HaneulAddress;
use test_utils::network::start_test_network;

pub async fn setup_network_and_wallet(
) -> Result<(HaneulNetwork, WalletContext, HaneulAddress), anyhow::Error> {
    let working_dir = tempfile::tempdir()?;

    let network = start_test_network(working_dir.path(), None).await?;

    // Create Wallet context.
    let wallet_conf = working_dir.path().join(HANEUL_WALLET_CONFIG);
    let mut context = WalletContext::new(&wallet_conf)?;
    let address = context.config.accounts.first().cloned().unwrap();

    // Sync client to retrieve objects from the network.
    WalletCommands::SyncClientState {
        address: Some(address),
    }
    .execute(&mut context)
    .await?;
    Ok((network, context, address))
}
