// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::bail;
use std::{fs::create_dir_all, path::PathBuf};

pub mod benchmark;
pub mod config;
pub mod keystore;
pub mod rpc_gateway;
pub mod rpc_gateway_client;
pub mod shell;
pub mod haneul_commands;
pub mod haneul_json;
pub mod wallet_commands;

const HANEUL_DIR: &str = ".haneul";
const HANEUL_CONFIG_DIR: &str = "haneul_config";
pub const HANEUL_NETWORK_CONFIG: &str = "network.conf";
pub const HANEUL_WALLET_CONFIG: &str = "wallet.conf";
pub const HANEUL_GATEWAY_CONFIG: &str = "gateway.conf";
pub const HANEUL_DEV_NET_URL: &str = "http://gateway.devnet.haneul.io:9000";

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
