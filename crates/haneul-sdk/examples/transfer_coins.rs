// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use haneul_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use haneul_sdk::{
    types::{
        base_types::{ObjectID, HaneulAddress},
        messages::Transaction,
    },
    HaneulClient,
};
use haneul_types::messages::ExecuteTransactionRequestType;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClient::new("https://fullnode.devnet.haneul.io:443", None).await?;
    // Load keystore from ~/.haneul/haneul_config/haneul.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".haneul").join("haneul_config").join("haneul.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };

    let my_address = HaneulAddress::from_str("0x47722589dc23d63e82862f7814070002ffaaa465")?;
    let gas_object_id = ObjectID::from_str("0x273b2a83f1af1fda3ddbc02ad31367fcb146a814")?;
    let recipient = HaneulAddress::from_str("0xbd42a850e81ebb8f80283266951d4f4f5722e301")?;

    // Create a haneul transfer transaction
    let transfer_tx = haneul
        .transaction_builder()
        .transfer_haneul(my_address, gas_object_id, 1000, recipient, Some(1000))
        .await?;

    // Sign transaction
    let keystore = Keystore::from(FileBasedKeystore::new(&keystore_path)?);
    let signature = keystore.sign(&my_address, &transfer_tx.to_bytes())?;

    // Execute the transaction
    let transaction_response = haneul
        .quorum_driver()
        .execute_transaction(
            Transaction::new(transfer_tx, signature).verify()?,
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    println!("{:?}", transaction_response);

    Ok(())
}
