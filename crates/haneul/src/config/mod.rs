// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::fmt::{Display, Formatter, Write};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub use haneul_config::utils;
pub use haneul_config::Config;
pub use haneul_config::PersistedConfig;
use haneul_config::HANEUL_DEV_NET_URL;
use haneul_keys::keystore::AccountKeystore;
use haneul_keys::keystore::Keystore;
use haneul_sdk::HaneulClient;
use haneul_types::base_types::*;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct HaneulClientConfig {
    pub keystore: Keystore,
    pub envs: Vec<HaneulEnv>,
    pub active_env: Option<String>,
    pub active_address: Option<HaneulAddress>,
}

impl HaneulClientConfig {
    pub fn new(keystore: Keystore) -> Self {
        HaneulClientConfig {
            keystore,
            envs: vec![],
            active_env: None,
            active_address: None,
        }
    }

    pub fn get_env(&self, alias: &Option<String>) -> Option<&HaneulEnv> {
        if let Some(alias) = alias {
            self.envs.iter().find(|env| &env.alias == alias)
        } else {
            self.envs.first()
        }
    }

    pub fn get_active_env(&self) -> Result<&HaneulEnv, anyhow::Error> {
        self.get_env(&self.active_env).ok_or_else(|| {
            anyhow!(
                "Environment configuration not found for env [{}]",
                self.active_env.as_deref().unwrap_or("None")
            )
        })
    }

    pub fn add_env(&mut self, env: HaneulEnv) {
        if !self
            .envs
            .iter()
            .any(|other_env| other_env.alias == env.alias)
        {
            self.envs.push(env)
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HaneulEnv {
    pub alias: String,
    pub rpc: String,
    pub ws: Option<String>,
}

impl HaneulEnv {
    pub async fn create_rpc_client(&self) -> Result<HaneulClient, anyhow::Error> {
        HaneulClient::new(&self.rpc, self.ws.as_deref()).await
    }

    pub fn devnet() -> Self {
        Self {
            alias: "devnet".to_string(),
            rpc: HANEUL_DEV_NET_URL.into(),
            ws: None,
        }
    }
}

impl Display for HaneulEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        writeln!(writer, "Active environment : {}", self.alias)?;
        write!(writer, "RPC URL: {}", self.rpc)?;
        if let Some(ws) = &self.ws {
            writeln!(writer)?;
            write!(writer, "Websocket URL: {ws}")?;
        }
        write!(f, "{}", writer)
    }
}

impl Config for HaneulClientConfig {}

impl Display for HaneulClientConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();

        writeln!(
            writer,
            "Managed addresses : {}",
            self.keystore.addresses().len()
        )?;
        write!(writer, "Active address: ")?;
        match self.active_address {
            Some(r) => writeln!(writer, "{}", r)?,
            None => writeln!(writer, "None")?,
        };
        writeln!(writer, "{}", self.keystore)?;
        if let Ok(env) = self.get_active_env() {
            write!(writer, "{}", env)?;
        }
        write!(f, "{}", writer)
    }
}
