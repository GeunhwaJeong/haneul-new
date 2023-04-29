// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use jsonrpsee::core::client::{Subscription, SubscriptionClientT};
use jsonrpsee::rpc_params;
use tokio::time::timeout;

use haneul_core::test_utils::wait_for_tx;
use haneul_json_rpc_types::{
    HaneulTransactionBlockEffects, HaneulTransactionBlockEffectsAPI, TransactionFilter,
};
use test_utils::network::TestClusterBuilder;
use test_utils::transaction::{create_devnet_nft, publish_nfts_package};

#[tokio::test]
async fn test_subscribe_transaction() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await.unwrap();

    let address = &cluster.accounts[0];
    let mut wallet = cluster.wallet;

    let ws_client = cluster.fullnode_handle.ws_client;

    let package_id = publish_nfts_package(&mut wallet).await.0;

    let mut sub: Subscription<HaneulTransactionBlockEffects> = ws_client
        .subscribe(
            "haneulx_subscribeTransaction",
            rpc_params![TransactionFilter::FromAddress(*address)],
            "haneulx_unsubscribeTransaction",
        )
        .await
        .unwrap();

    let (_, _, digest) = create_devnet_nft(&mut wallet, package_id).await?;
    wait_for_tx(digest, cluster.fullnode_handle.haneul_node.state().clone()).await;

    // Wait for streaming
    let effects = match timeout(Duration::from_secs(5), sub.next()).await {
        Ok(Some(Ok(tx))) => tx,
        _ => panic!("Failed to get tx"),
    };

    assert_eq!(&digest, effects.transaction_digest());
    Ok(())
}
