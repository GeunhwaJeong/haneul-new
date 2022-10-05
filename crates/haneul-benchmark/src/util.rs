// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use haneul_sdk::crypto::FileBasedKeystore;
use haneul_types::{
    base_types::HaneulAddress,
    crypto::{AccountKeyPair, EncodeDecodeBase64, HaneulKeyPair},
};

use std::path::PathBuf;

pub fn get_ed25519_keypair_from_keystore(
    keystore_path: PathBuf,
    requested_address: &HaneulAddress,
) -> Result<AccountKeyPair> {
    let keystore = FileBasedKeystore::load_or_create(&keystore_path)?;
    let keypair = keystore
        .key_pairs()
        .iter()
        .find(|x| {
            let address: HaneulAddress = Into::<HaneulAddress>::into(&x.public());
            address == *requested_address
        })
        .map(|x| x.encode_base64())
        .unwrap();
    // TODO(joyqvq): This is a hack to decode base64 keypair with added flag, ok for now since it is for benchmark use.
    // Rework to get the typed keypair directly from above.
    Ok(match HaneulKeyPair::decode_base64(&keypair).unwrap() {
        HaneulKeyPair::Ed25519HaneulKeyPair(x) => x,
        _ => panic!("Unexpected keypair type"),
    })
}
