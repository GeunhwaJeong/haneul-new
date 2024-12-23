// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_macros::sim_test;
use haneul_rpc_api::types::ExecuteTransactionOptions;
use haneul_rpc_api::Client;
use haneul_sdk_types::BalanceChange;
use haneul_test_transaction_builder::make_transfer_haneul_transaction;
use haneul_types::base_types::HaneulAddress;
use haneul_types::effects::TransactionEffectsAPI;
use haneul_types::transaction::TransactionDataAPI;
use test_cluster::TestClusterBuilder;

#[sim_test]
async fn execute_transaction_transfer() {
    let test_cluster = TestClusterBuilder::new().build().await;

    let client = Client::new(test_cluster.rpc_url()).unwrap();
    let address = HaneulAddress::random_for_testing_only();
    let amount = 9;

    let txn =
        make_transfer_haneul_transaction(&test_cluster.wallet, Some(address), Some(amount)).await;
    let sender = txn.transaction_data().sender();

    let options = ExecuteTransactionOptions {
        balance_changes: Some(true),
        ..Default::default()
    };

    let response = client.execute_transaction(&options, &txn).await.unwrap();

    let gas = response.effects.gas_cost_summary().net_gas_usage();

    let coin_type = haneul_types::haneul_sdk_types_conversions::type_tag_core_to_sdk(
        haneul_types::gas_coin::GAS::type_tag(),
    )
    .unwrap();
    let mut expected = vec![
        BalanceChange {
            address: sender.into(),
            coin_type: coin_type.clone(),
            amount: -(amount as i128 + gas as i128),
        },
        BalanceChange {
            address: address.into(),
            coin_type,
            amount: amount as i128,
        },
    ];
    expected.sort_by_key(|e| e.address);

    let mut actual = response.balance_changes.unwrap();
    actual.sort_by_key(|e| e.address);

    assert_eq!(actual, expected);
}
