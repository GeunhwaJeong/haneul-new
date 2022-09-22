---
title: Interact with Haneul over Rust SDK
---

## Overview
The [Haneul SDK](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-sdk) is a collection of Rust language JSON-RPC wrapper and crypto utilities you can use to interact with the [Haneul Devnet Gateway](../build/devnet.md) and [Haneul Full Node](fullnode.md).

The [`HaneulClient`](cli-client.md) can be used to create an HTTP or a WebSocket client (`HaneulClient::new_rpc_client`).  
See our [JSON-RPC](json-rpc.md#haneul-json-rpc-methods) doc for the list of available methods.

> Note: As of [Haneul version 0.6.0](https://github.com/GeunhwaJeong/haneul/releases/tag/devnet-0.6.0), the WebSocket client is for [subscription only](pubsub.md); use the HTTP client for other API methods.

## References

Find the `rustdoc` output for key Haneul projects at:

* Haneul blockchain - https://haneullabs.github.io/haneul/
* Narwhal and Bullshark consensus engine - https://haneullabs.github.io/narwhal/
* Haneul Labs infrastructure - https://haneullabs.github.io/haneullabs-infra/

## Configuration
Add the `haneul-sdk` crate in your [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) file like so:
```toml
[dependencies]
haneul-sdk = { git = "https://github.com/GeunhwaJeong/haneul" }
```
If you are connecting to the devnet, use the `devnet` branch instead:
```toml
[dependencies]
haneul-sdk = { git = "https://github.com/GeunhwaJeong/haneul", branch = "devnet" }
```

## Examples

### Example 1 - Get all objects owned by an address

This will print a list of object summaries owned by the address `"0xec11cad080d0496a53bafcea629fcbcfff2a9866"`:

```rust
use std::str::FromStr;
use haneul_sdk::types::base_types::HaneulAddress;
use haneul_sdk::HaneulClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClient::new_rpc_client("https://gateway.devnet.haneul.io:443", None).await?;
    let address = HaneulAddress::from_str("0xec11cad080d0496a53bafcea629fcbcfff2a9866")?;
    let objects = haneul.read_api().get_objects_owned_by_address(address).await?;
    println!("{:?}", objects);
    Ok(())
}
```

You can verify the result with the [Haneul Explorer](https://explorer.devnet.haneul.io/) if you are using the Haneul Devnet Gateway.

### Example 2 - Create and execute transaction

Use this example to conduct a transaction in Haneul using the Haneul Devnet Gateway:

```rust
use std::str::FromStr;
use haneul_sdk::{
    crypto::HaneulKeystore,
    types::{
        base_types::{ObjectID, HaneulAddress},
        crypto::Signature,
        messages::Transaction,
    },
    HaneulClient,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClient::new_rpc_client("https://gateway.devnet.haneul.io:443", None).await?;
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

    // Get signer from keystore
    let keystore = KeystoreType::File(keystore_path).init()?;
    let signer = keystore.signer(my_address);

    // Sign the transaction
    let signature = Signature::new(&transfer_tx, &signer);

    // Execute the transaction
    let transaction_response = haneul
        .quorum_driver()
        .execute_transaction(Transaction::new(transfer_tx, signature))
        .await?;

    println!("{:?}", transaction_response);

    Ok(())
}
```

### Example 3 - Event subscription

Use the WebSocket client to [subscribe to events](pubsub.md).

```rust
use futures::StreamExt;
use haneul_sdk::rpc_types::HaneulEventFilter;
use haneul_sdk::HaneulClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClient::new_rpc_client("https://gateway.devnet.haneul.io:443", Some("ws://127.0.0.1:9001")).await?;
    let mut subscribe_all = haneul.event_api().subscribe_event(HaneulEventFilter::All(vec![])).await?;
    loop {
        println!("{:?}", subscribe_all.next().await);
    }
}
```
> Note: You will need to connect to a fullnode for the Event subscription service, see [Fullnode setup](fullnode.md#fullnode-setup) if you want to run a Haneul Fullnode.


## Larger examples

See the Haneul Rust SDK README for the [Tic Tac Toe](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-sdk) example.
