---
title: Haneul Rust SDK
---

## Overview
The Haneul SDK is a collection of rust JSON-RPC wrapper and crypto utilities that you can use to interact with the Haneul Gateway and Haneul Full Node.
The `HaneulClient` can be used to create a http(`HaneulClient::new_http_client`) or a websocket client(`HaneulClient::new_ws_client`).  
See [JSON-RPC doc](json-rpc.md#haneul-json-rpc-methods) for list of available methods.

> Note: As of v0.6.0, the web socket client is for subscription only, please use http client for other api methods.

## Examples
Add the haneul-sdk crate in your Cargo.toml:
```toml
[dependencies]
haneul-sdk = { git = "https://github.com/GeunhwaJeong/haneul" }
```
Use the devnet branch if you are connecting to the devnet. 
```toml
[dependencies]
haneul-sdk = { git = "https://github.com/GeunhwaJeong/haneul", branch = "devnet" }
```

### Example 1 - Get all objects owned by an address
```rust
use std::str::FromStr;
use haneul_sdk::types::base_types::HaneulAddress;
use haneul_sdk::HaneulClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClient::new_http_client("https://gateway.devnet.haneul.io:443")?;
    let address = HaneulAddress::from_str("0xec11cad080d0496a53bafcea629fcbcfff2a9866")?;
    let objects = haneul.get_objects_owned_by_address(address).await?;
    println!("{:?}", objects);
    Ok(())
}
```
This will print a list of object summaries owned by the address "0xec11cad080d0496a53bafcea629fcbcfff2a9866".
You can verify the result with the [Haneul explorer](https://explorer.devnet.haneul.io/) if you are using the Haneul devnet.

### Example 2 - Create and execute transaction
```rust
use std::str::FromStr;
use haneul_sdk::crypto::{Keystore, HaneulKeystore};
use haneul_sdk::types::base_types::{ObjectID, HaneulAddress};
use haneul_sdk::types::haneul_serde::Base64;
use haneul_sdk::HaneulClient;

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
            Base64::from_bytes(signature.signature_bytes()),
            Base64::from_bytes(signature.public_key_bytes()),
        )
        .await?;

    println!("{:?}", transaction_response);

    Ok(())
}
```

### Example 3 - Event subscription
```rust
use futures::StreamExt;
use haneul_sdk::rpc_types::HaneulEventFilter;
use haneul_sdk::HaneulClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClient::new_ws_client("ws://127.0.0.1:9001").await?;
    let mut subscribe_all = haneul.subscribe_event(HaneulEventFilter::All(vec![])).await?;
    loop {
        println!("{:?}", subscribe_all.next().await);
    }
}
```
> Note: You will need to connect to a fullnode for the Event subscription service, see [Fullnode setup](fullnode.md#fullnode-setup) if you want to run a fullnode.


## Larger Examples
[Tic Tac Toe](../../../crates/haneul-sdk/README.md)