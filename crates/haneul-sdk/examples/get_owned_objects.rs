// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use haneul_json_rpc_types::{HaneulObjectDataOptions, HaneulObjectResponseQuery};
use haneul_sdk::types::base_types::HaneulAddress;
use haneul_sdk::HaneulClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClientBuilder::default()
        .build("https://fullnode.devnet.haneul.io:443")
        .await?;
    let address = HaneulAddress::from_str("0xec11cad080d0496a53bafcea629fcbcfff2a9866")?;
    let objects = haneul
        .read_api()
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new(),
            )),
            None,
            None,
            None,
        )
        .await?;
    println!("{:?}", objects);
    Ok(())
}
