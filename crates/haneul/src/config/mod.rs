// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, serde_as};
use std::fmt::{Display, Formatter, Write};
use haneul_json_rpc_api::keystore::KeystoreType;
use haneul_types::base_types::*;

pub use haneul_config::Config;
pub use haneul_config::PersistedConfig;

pub use haneul_config::utils;

pub use haneul_gateway::config::{GatewayConfig, GatewayType};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct HaneulClientConfig {
    #[serde_as(as = "Vec<Hex>")]
    pub accounts: Vec<HaneulAddress>,
    pub keystore: KeystoreType,
    pub gateway: GatewayType,
    pub active_address: Option<HaneulAddress>,
}

impl Config for HaneulClientConfig {}

impl Display for HaneulClientConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();

        writeln!(writer, "Managed addresses : {}", self.accounts.len())?;
        write!(writer, "Active address: ")?;
        match self.active_address {
            Some(r) => writeln!(writer, "{}", r)?,
            None => writeln!(writer, "None")?,
        };
        writeln!(writer, "{}", self.keystore)?;
        write!(writer, "{}", self.gateway)?;

        write!(f, "{}", writer)
    }
}
