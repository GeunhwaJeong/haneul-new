// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::trace;

pub mod builder;
pub mod genesis;
pub mod genesis_config;
pub mod node;
mod swarm;
pub mod utils;

pub use node::{CommitteeConfig, ConsensusConfig, NodeConfig, ValidatorInfo};
pub use swarm::NetworkConfig;

const HANEUL_DIR: &str = ".haneul";
const HANEUL_CONFIG_DIR: &str = "haneul_config";
pub const HANEUL_NETWORK_CONFIG: &str = "network.yaml";
pub const HANEUL_FULLNODE_CONFIG: &str = "fullnode.yaml";
pub const HANEUL_WALLET_CONFIG: &str = "wallet.yaml";
pub const HANEUL_GATEWAY_CONFIG: &str = "gateway.yaml";
pub const HANEUL_DEV_NET_URL: &str = "https://gateway.devnet.haneul.io:443";

pub const AUTHORITIES_DB_NAME: &str = "authorities_db";
pub const CONSENSUS_DB_NAME: &str = "consensus_db";
pub const FULL_NODE_DB_PATH: &str = "full_node_db";

const DEFAULT_STAKE: usize = 1;

pub fn haneul_config_dir() -> Result<PathBuf, anyhow::Error> {
    match std::env::var_os("HANEUL_CONFIG_DIR") {
        Some(config_env) => Ok(config_env.into()),
        None => match dirs::home_dir() {
            Some(v) => Ok(v.join(HANEUL_DIR).join(HANEUL_CONFIG_DIR)),
            None => anyhow::bail!("Cannot obtain home directory path"),
        },
    }
    .and_then(|dir| {
        if !dir.exists() {
            std::fs::create_dir_all(dir.clone())?;
        }
        Ok(dir)
    })
}

pub trait Config
where
    Self: DeserializeOwned + Serialize,
{
    fn persisted(self, path: &Path) -> PersistedConfig<Self> {
        PersistedConfig {
            inner: self,
            path: path.to_path_buf(),
        }
    }
}

pub struct PersistedConfig<C> {
    inner: C,
    path: PathBuf,
}

impl<C> PersistedConfig<C>
where
    C: Config,
{
    pub fn read(path: &Path) -> Result<C, anyhow::Error> {
        trace!("Reading config from '{:?}'", path);
        let reader = fs::File::open(path)?;
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        trace!("Writing config to '{:?}'", &self.path);
        let config = serde_yaml::to_string(&self.inner)?;
        fs::write(&self.path, config)?;
        Ok(())
    }

    pub fn into_inner(self) -> C {
        self.inner
    }
}

impl<C> std::ops::Deref for PersistedConfig<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<C> std::ops::DerefMut for PersistedConfig<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
