// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

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
