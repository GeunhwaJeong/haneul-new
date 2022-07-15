// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

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
