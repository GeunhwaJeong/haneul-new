// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use haneul_sdk::crypto::{Keystore, HaneulKeystore};
use haneul_sdk::types::base_types::{ObjectID, HaneulAddress};
use haneul_sdk::types::haneul_serde::Base64;
use haneul_sdk::HaneulClient;
use haneul_types::crypto::HaneulSignature;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClient::new_http_client("https://gateway.devnet.haneul.io:443")?;
    // Load keystore from ~/.haneul/haneul_config/haneul.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".haneul").join("haneul_config").join("haneul.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };
    let keystore = HaneulKeystore::load_or_create(&keystore_path)?;

    let my_address = HaneulAddress::from_str("0x47722589dc23d63e82862f7814070002ffaaa465")?;
    let gas_object_id = ObjectID::from_str("0x273b2a83f1af1fda3ddbc02ad31367fcb146a814")?;
    let recipient = HaneulAddress::from_str("0xbd42a850e81ebb8f80283266951d4f4f5722e301")?;

    // Create a haneul transfer transaction
    let transfer_tx = haneul
        .transfer_haneul(my_address, gas_object_id, 1000, recipient, Some(1000))
        .await?;

    // Sign the transaction
    let signature = keystore.sign(&my_address, &transfer_tx.tx_bytes.to_vec()?)?;

    // Execute the transaction
    let transaction_response = haneul
        .execute_transaction(
            transfer_tx.tx_bytes,
            Base64::from_bytes(&[signature.flag_byte()]),
            Base64::from_bytes(signature.signature_bytes()),
            Base64::from_bytes(signature.public_key_bytes()),
        )
        .await?;

    println!("{:?}", transaction_response);

    Ok(())
}
