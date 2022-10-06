// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use futures::StreamExt;
use haneul_sdk::rpc_types::HaneulEventFilter;
use haneul_sdk::HaneulClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul =
        HaneulClient::new_rpc_client("http://127.0.0.1:5001", Some("ws://127.0.0.1:9001")).await?;
    let mut subscribe_all = haneul
        .event_api()
        .subscribe_event(HaneulEventFilter::All(vec![]))
        .await?;
    loop {
        println!("{:?}", subscribe_all.next().await);
    }
}
