// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use futures::stream::StreamExt;
use haneul_json_rpc_types::TransactionFilter;
use haneul_sdk::HaneulClientBuilder;

// This example showcases how to use the Read API to listen
// for transactions. It subscribes to the transactions that
// transfer HANEUL on the Haneul testnet and prints every incoming
// transaction to the console. The program will loop until it
// is force stopped.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ws = HaneulClientBuilder::default()
        .ws_url("wss://rpc.testnet.haneul.io:443")
        .build("https://fullnode.testnet.haneul.io:443")
        .await?;
    println!("WS version {:?}", ws.api_version());

    let mut subscribe = ws
        .read_api()
        .subscribe_transaction(TransactionFilter::MoveFunction {
            package: "0x2".parse()?,
            module: Some("haneul".to_owned()),
            function: Some("transfer".to_owned()),
        })
        .await?;

    loop {
        println!("{:?}", subscribe.next().await);
    }
}
