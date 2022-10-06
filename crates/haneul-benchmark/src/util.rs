// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use haneul_sdk::crypto::{AccountKeystore, FileBasedKeystore};
use haneul_types::{
    base_types::HaneulAddress,
    crypto::{AccountKeyPair, KeypairTraits, HaneulKeyPair},
};

use std::path::PathBuf;

pub fn get_ed25519_keypair_from_keystore(
    keystore_path: PathBuf,
    requested_address: &HaneulAddress,
) -> Result<AccountKeyPair> {
    let keystore = FileBasedKeystore::new(&keystore_path)?;
    match keystore.get_key(requested_address) {
        Ok(HaneulKeyPair::Ed25519HaneulKeyPair(kp)) => Ok(kp.copy()),
        _ => Err(anyhow::anyhow!("Unsupported key type")),
    }
}
