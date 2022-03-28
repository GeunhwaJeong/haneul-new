// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use ed25519_dalek::ed25519::signature;
use ed25519_dalek::{ed25519, Signer};
use serde::{Deserialize, Serialize};

use haneul_types::base_types::HaneulAddress;
use haneul_types::crypto::{get_key_pair, KeyPair, Signature};

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
// This will work on user signatures, but not suitable for authority signatures.
pub enum KeystoreType {
    File(PathBuf),
}

pub trait Keystore: Send + Sync {
    fn sign(&self, address: &HaneulAddress, msg: &[u8]) -> Result<Signature, signature::Error>;
    fn add_random_key(&mut self) -> Result<HaneulAddress, anyhow::Error>;
}

impl KeystoreType {
    pub fn init(&self) -> Result<Box<dyn Keystore>, anyhow::Error> {
        Ok(match self {
            KeystoreType::File(path) => Box::new(HaneulKeystore::load_or_create(path)?),
        })
    }
}

impl Display for KeystoreType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match self {
            KeystoreType::File(path) => {
                writeln!(writer, "Keystore Type : File")?;
                write!(writer, "Keystore Path : {:?}", path)?;
                write!(f, "{}", writer)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct HaneulKeystore {
    keys: BTreeMap<HaneulAddress, KeyPair>,
}

impl Keystore for HaneulKeystore {
    fn sign(&self, address: &HaneulAddress, msg: &[u8]) -> Result<Signature, signature::Error> {
        Ok(self
            .keys
            .get(address)
            .ok_or_else(|| {
                signature::Error::from_source(format!("Cannot find key for address: [{address}]"))
            })?
            .sign(msg))
    }

    fn add_random_key(&mut self) -> Result<HaneulAddress, anyhow::Error> {
        let (address, keypair) = get_key_pair();
        self.keys.insert(address, keypair);
        Ok(address)
    }
}

impl HaneulKeystore {
    pub fn load_or_create(path: &Path) -> Result<Self, anyhow::Error> {
        let keys: Vec<KeyPair> = if path.exists() {
            let reader = BufReader::new(File::open(path)?);
            serde_json::from_reader(reader)?
        } else {
            Vec::new()
        };

        let keys = keys
            .into_iter()
            .map(|key| (HaneulAddress::from(key.public_key_bytes()), key))
            .collect();

        Ok(Self { keys })
    }

    pub fn save(&self, path: &Path) -> Result<(), anyhow::Error> {
        let store = serde_json::to_string_pretty(&self.keys.values().collect::<Vec<_>>()).unwrap();
        Ok(fs::write(path, store)?)
    }

    pub fn add_key(&mut self, address: HaneulAddress, keypair: KeyPair) -> Result<(), anyhow::Error> {
        self.keys.insert(address, keypair);
        Ok(())
    }

    pub fn addresses(&self) -> Vec<HaneulAddress> {
        self.keys.keys().cloned().collect()
    }
}

pub struct HaneulKeystoreSigner {
    keystore: Arc<RwLock<Box<dyn Keystore>>>,
    address: HaneulAddress,
}

impl HaneulKeystoreSigner {
    pub fn new(keystore: Arc<RwLock<Box<dyn Keystore>>>, account: HaneulAddress) -> Self {
        Self {
            keystore,
            address: account,
        }
    }
}

impl signature::Signer<Signature> for HaneulKeystoreSigner {
    fn try_sign(&self, msg: &[u8]) -> Result<Signature, ed25519::Error> {
        self.keystore
            .read()
            .unwrap()
            .sign(&self.address, msg)
            .map_err(ed25519::Error::from_source)
    }
}
