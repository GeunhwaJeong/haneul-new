// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::keystore::KeystoreType;
use anyhow::bail;
use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, serde_as};
use std::{
    fmt::{Display, Formatter, Write},
    fs::create_dir_all,
    path::PathBuf,
};
use haneul_types::base_types::*;

pub use haneul_config::Config;
pub use haneul_config::PersistedConfig;

pub use haneul_config::utils;

pub use haneul_gateway::config::{GatewayConfig, GatewayType};

const HANEUL_DIR: &str = ".haneul";
const HANEUL_CONFIG_DIR: &str = "haneul_config";
pub const HANEUL_NETWORK_CONFIG: &str = "network.conf";
pub const HANEUL_WALLET_CONFIG: &str = "wallet.conf";
pub const HANEUL_GATEWAY_CONFIG: &str = "gateway.conf";
pub const FULL_NODE_DB_PATH: &str = "full_node_db";

pub const HANEUL_DEV_NET_URL: &str = "https://gateway.devnet.haneul.io:9000";

pub fn haneul_config_dir() -> Result<PathBuf, anyhow::Error> {
    match std::env::var_os("HANEUL_CONFIG_DIR") {
        Some(config_env) => Ok(config_env.into()),
        None => match dirs::home_dir() {
            Some(v) => Ok(v.join(HANEUL_DIR).join(HANEUL_CONFIG_DIR)),
            None => bail!("Cannot obtain home directory path"),
        },
    }
    .and_then(|dir| {
        if !dir.exists() {
            create_dir_all(dir.clone())?;
        }
        Ok(dir)
    })
}

pub const AUTHORITIES_DB_NAME: &str = "authorities_db";
pub const DEFAULT_STARTING_PORT: u16 = 10000;
pub const CONSENSUS_DB_NAME: &str = "consensus_db";

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct WalletConfig {
    #[serde_as(as = "Vec<Hex>")]
    pub accounts: Vec<HaneulAddress>,
    pub keystore: KeystoreType,
    pub gateway: GatewayType,
    pub active_address: Option<HaneulAddress>,
}

impl Config for WalletConfig {}

impl Display for WalletConfig {
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
