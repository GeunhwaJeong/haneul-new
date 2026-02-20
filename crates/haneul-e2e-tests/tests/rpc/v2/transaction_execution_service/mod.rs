// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use prost_types::FieldMask;
use haneul_macros::sim_test;
use haneul_rpc::field::FieldMaskUtil;
use haneul_rpc::proto::haneul::rpc::v2::Bcs;
use haneul_rpc::proto::haneul::rpc::v2::ExecuteTransactionRequest;
use haneul_rpc::proto::haneul::rpc::v2::ExecuteTransactionResponse;
use haneul_rpc::proto::haneul::rpc::v2::Transaction;
use haneul_rpc::proto::haneul::rpc::v2::UserSignature;
use haneul_rpc::proto::haneul::rpc::v2::transaction_execution_service_client::TransactionExecutionServiceClient;
use haneul_sdk_types::BalanceChange;
use haneul_test_transaction_builder::make_transfer_haneul_transaction;
use haneul_types::base_types::HaneulAddress;
use haneul_types::transaction::TransactionDataAPI;
use test_cluster::TestClusterBuilder;

mod resolve;

#[sim_test]
async fn execute_transaction_transfer() {
    let test_cluster = TestClusterBuilder::new()
        .with_num_validators(1)
        .build()
        .await;

    let mut client = TransactionExecutionServiceClient::connect(test_cluster.rpc_url().to_owned())
        .await
        .unwrap();
    let address = HaneulAddress::random_for_testing_only();
    let amount = 9;

    let txn =
        make_transfer_haneul_transaction(&test_cluster.wallet, Some(address), Some(amount)).await;
    let sender = txn.transaction_data().sender();

    let ExecuteTransactionResponse { transaction, .. } = client
        .execute_transaction({
            let mut message = ExecuteTransactionRequest::default();
            message.transaction = Some({
                let mut message = Transaction::default();
                message.bcs = Some(Bcs::serialize(txn.transaction_data()).unwrap());
                message
            });
            message.signatures = txn
                .tx_signatures()
                .iter()
                .map(|s| {
                    let mut message = UserSignature::default();
                    message.bcs = Some(Bcs::from(s.as_ref().to_owned()));
                    message
                })
                .collect();
            message.read_mask = Some(FieldMask::from_paths(["*"]));
            message
        })
        .await
        .unwrap()
        .into_inner();

    let transaction = transaction.unwrap();
    let gas_summary =
        haneul_sdk_types::GasCostSummary::try_from(&transaction.effects.unwrap().gas_used.unwrap())
            .unwrap();
    let gas = gas_summary.net_gas_usage();

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

    let mut actual = transaction
        .balance_changes
        .into_iter()
        .map(|bc| BalanceChange::try_from(&bc).unwrap())
        .collect::<Vec<_>>();
    actual.sort_by_key(|e| e.address);

    assert_eq!(actual, expected);
}
