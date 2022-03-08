// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::time::Duration;

use serde::Deserialize;
use serde::Serialize;

use haneul_core::authority_client::AuthorityClient;
use haneul_core::client::{ClientAddressManager, GatewayClient};
use haneul_network::network::NetworkClient;
use haneul_network::transport;
use haneul_types::base_types::AuthorityName;
use haneul_types::committee::Committee;

use crate::config::AuthorityInfo;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GatewayType {
    Embedded(EmbeddedGatewayConfig),
    Rest(String),
}

impl Display for GatewayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();

        match self {
            GatewayType::Embedded(config) => {
                writeln!(writer, "Gateway Type : Embedded")?;
                writeln!(
                    writer,
                    "Client state DB folder path : {:?}",
                    config.db_folder_path
                )?;
                let authorities = config
                    .authorities
                    .iter()
                    .map(|info| format!("{}:{}", info.host, info.base_port));
                writeln!(
                    writer,
                    "Authorities : {:?}",
                    authorities.collect::<Vec<_>>()
                )?;
            }
            GatewayType::Rest(url) => {
                writeln!(writer, "Gateway Type : RestAPI")?;
                writeln!(writer, "Gateway URL : {}", url)?;
            }
        }
        write!(f, "{}", writer)
    }
}

impl GatewayType {
    pub fn init(&self) -> GatewayClient {
        match self {
            GatewayType::Embedded(config) => {
                let path = config.db_folder_path.clone();
                let committee = config.make_committee();
                let authority_clients = config.make_authority_clients();
                Box::new(ClientAddressManager::new(
                    path,
                    committee,
                    authority_clients,
                ))
            }
            _ => {
                panic!("Unsupported gateway type")
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmbeddedGatewayConfig {
    pub authorities: Vec<AuthorityInfo>,
    pub send_timeout: Duration,
    pub recv_timeout: Duration,
    pub buffer_size: usize,
    pub db_folder_path: PathBuf,
}

impl EmbeddedGatewayConfig {
    pub fn make_committee(&self) -> Committee {
        let voting_rights = self
            .authorities
            .iter()
            .map(|authority| (authority.name, 1))
            .collect();
        Committee::new(voting_rights)
    }

    pub fn make_authority_clients(&self) -> BTreeMap<AuthorityName, AuthorityClient> {
        let mut authority_clients = BTreeMap::new();
        for authority in &self.authorities {
            let client = AuthorityClient::new(NetworkClient::new(
                authority.host.clone(),
                authority.base_port,
                self.buffer_size,
                self.send_timeout,
                self.recv_timeout,
            ));
            authority_clients.insert(authority.name, client);
        }
        authority_clients
    }
}

impl Default for EmbeddedGatewayConfig {
    fn default() -> Self {
        Self {
            authorities: vec![],
            send_timeout: Duration::from_micros(4000000),
            recv_timeout: Duration::from_micros(4000000),
            buffer_size: transport::DEFAULT_MAX_DATAGRAM_SIZE,
            db_folder_path: Default::default(),
        }
    }
}
