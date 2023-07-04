// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_core::authority_client::AuthorityAPI;
use haneul_macros::sim_test;
use haneul_types::error::{HaneulError, HaneulResult};
use haneul_types::utils::make_upgraded_multisig_tx;
use test_cluster::TestClusterBuilder;

async fn do_upgraded_multisig_test() -> HaneulResult {
    let test_cluster = TestClusterBuilder::new().build().await;
    let tx = make_upgraded_multisig_tx();

    test_cluster
        .authority_aggregator()
        .authority_clients
        .values()
        .next()
        .unwrap()
        .authority_client()
        .handle_transaction(tx)
        .await
        .map(|_| ())
}

#[sim_test]
async fn test_upgraded_multisig_feature_deny() {
    use haneul_protocol_config::ProtocolConfig;

    let _guard = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
        config.set_upgraded_multisig_for_testing(false);
        config
    });

    let err = do_upgraded_multisig_test().await.unwrap_err();

    assert!(matches!(err, HaneulError::UnsupportedFeatureError { .. }));
}

#[sim_test]
async fn test_upgraded_multisig_feature_allow() {
    use haneul_protocol_config::ProtocolConfig;

    let _guard = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
        config.set_upgraded_multisig_for_testing(true);
        config
    });

    let res = do_upgraded_multisig_test().await;

    // we didn't make a real transaction with a valid object, but we verify that we pass the
    // feature gate.
    assert!(matches!(res.unwrap_err(), HaneulError::UserInputError { .. }));
}
