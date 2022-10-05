// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt::{Display, Formatter, Write};
use haneul_sdk::crypto::AccountKeystore;
use haneul_sdk::crypto::Keystore;
use haneul_types::base_types::*;

pub use haneul_config::Config;
pub use haneul_config::PersistedConfig;

pub use haneul_config::utils;
use haneul_sdk::ClientType;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct HaneulClientConfig {
    pub keystore: Keystore,
    pub client_type: ClientType,
    pub active_address: Option<HaneulAddress>,
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
        write!(writer, "{}", self.client_type)?;
        write!(f, "{}", writer)
    }
}
