// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use shared_crypto::intent::Intent;
use haneul_json_rpc_types::HaneulTransactionBlockResponseOptions;
use haneul_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use haneul_sdk::{
    types::{
        base_types::{ObjectID, HaneulAddress},
        messages::Transaction,
    },
    HaneulClientBuilder,
};
use haneul_types::messages::ExecuteTransactionRequestType;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClientBuilder::default()
        .build("https://fullnode.devnet.haneul.io:443")
        .await?;
    // Load keystore from ~/.haneul/haneul_config/haneul.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".haneul").join("haneul_config").join("haneul.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };

    let my_address = HaneulAddress::random_for_testing_only();
    let gas_object_id = ObjectID::random();
    let recipient = HaneulAddress::random_for_testing_only();

    // Create a haneul transfer transaction
    let transfer_tx = haneul
        .transaction_builder()
        .transfer_haneul(my_address, gas_object_id, 1000, recipient, Some(1000))
        .await?;

    // Sign transaction
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
    let signature = keystore.sign_secure(&my_address, &transfer_tx, Intent::default())?;

    // Execute the transaction
    let transaction_response = haneul
        .quorum_driver()
        .execute_transaction_block(
            Transaction::from_data(transfer_tx, Intent::default(), vec![signature]).verify()?,
            HaneulTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    println!("{:?}", transaction_response);

    Ok(())
}
