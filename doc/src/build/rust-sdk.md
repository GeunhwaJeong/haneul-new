---
title: Interact with Haneul using the Rust SDK
---

## Overview

The [Haneul SDK](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-sdk) is a collection of Rust language JSON-RPC wrapper and crypto utilities you can use to interact with Haneul.

Use the [`HaneulClient`](cli-client.md) to create an HTTP or a WebSocket client (`HaneulClient::new`). See the [JSON-RPC](json-rpc.md#haneul-json-rpc-methods) documentation for the list of available methods.

**Note:** The WebSocket client supports only [subscription](event_api.md#subscribe-to-haneul-events); use the HTTP client for other API methods.

## References

View the documentation for the [crates used in Haneul](https://haneullabs.github.io/haneul/).

## Configuration

Add the `haneul-sdk` crate in your [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) file:

```bash
[dependencies]
haneul-sdk = { git = "https://github.com/GeunhwaJeong/haneul" }
```

Include the `branch` argument to use a specific branch of the Haneul repository:

```bash
[dependencies]
haneul-sdk = { git = "https://github.com/GeunhwaJeong/haneul", branch = "devnet" }
```

## Example 1 - Get all objects owned by an address

This code example prints a list of object summaries owned by the specified address.

```rust
use std::str::FromStr;
use haneul_sdk::types::base_types::HaneulAddress;
use haneul_sdk::{HaneulClient, HaneulClientBuilder};

// TODO: (jian) update example after pagination changes
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClientBuilder::default().build(
      "https://fullnode.devnet.haneul.io:443",
    ).await.unwrap();
    let address = HaneulAddress::from_str("0xbcab7526033aa0e014f634bf51316715dda0907a7fab5a8d7e3bd44e634a4d44")?;
    let objects = haneul.read_api().get_owned_objects(address).await?;
    println!("{:?}", objects);
    Ok(())
}
```

You can verify the result with the [Haneul Explorer](https://explorer.haneul.io/) if you are using the Haneul Devnet Full node.

## Example 2 - Create and execute transaction

Use this example to conduct a transaction in Haneul using the Haneul Devnet Full node:

```rust
use std::str::FromStr;
use haneul_sdk::{
    crypto::{FileBasedKeystore, Keystore},
    types::{
        base_types::{ObjectID, HaneulAddress},
        crypto::Signature,
        messages::Transaction,
    },
    HaneulClient,
    HaneulClientBuilder,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClientBuilder::default().build(
      "https://fullnode.devnet.haneul.io:443",
    ).await.unwrap();
    // Load keystore from ~/.haneul/haneul_config/haneul.keystore
    let keystore_path = match dirs::home_dir() {
        Some(v) => v.join(".haneul").join("haneul_config").join("haneul.keystore"),
        None => panic!("Cannot obtain home directory path"),
    };

    let my_address = HaneulAddress::from_str("0xbcab7526033aa0e014f634bf51316715dda0907a7fab5a8d7e3bd44e634a4d44")?;
    let gas_object_id = ObjectID::from_str("0xe638c76768804cebc0ab43e103999886641b0269a46783f2b454e2f8880b5255")?;
    let recipient = HaneulAddress::from_str("0x727b37454ab13d5c1dbb22e8741bff72b145d1e660f71b275c01f24e7860e5e5")?;

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
        .execute_transaction(Transaction::from_data(transfer_tx, Intent::default(), signature))

    println!("{:?}", transaction_response);

    Ok(())
}
```

## Example 3 - Event subscription

Use the WebSocket client to [subscribe to events](event_api.md#subscribe-to-haneul-events).

```rust
use futures::StreamExt;
use haneul_sdk::rpc_types::HaneulEventFilter;
use haneul_sdk::{HaneulClient, HaneulClientBuilder};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClientBuilder::default().build(
      "https://fullnode.devnet.haneul.io:443",
    ).await.unwrap();
    let mut subscribe_all = haneul.event_api().subscribe_event(HaneulEventFilter::All(vec![])).await?;
    loop {
        println!("{:?}", subscribe_all.next().await);
    }
}
```

**Note:** The Event subscription service requires a running Haneul Full node. To learn more, see [Full node setup](fullnode.md#fullnode-setup).

