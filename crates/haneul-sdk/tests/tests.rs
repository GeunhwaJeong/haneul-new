// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use std::str::FromStr;

use sha3::{Digest, Sha3_256};
use tempfile::TempDir;

use haneul_sdk::crypto::KeystoreType;
use haneul_types::crypto::{SignatureScheme, HaneulSignatureInner};
use haneul_types::{
    base_types::{HaneulAddress, HANEUL_ADDRESS_LENGTH},
    crypto::Ed25519HaneulSignature,
};
#[test]
fn mnemonic_test() {
    let temp_dir = TempDir::new().unwrap();
    let keystore_path = temp_dir.path().join("haneul.keystore");
    let mut keystore = KeystoreType::File(keystore_path).init().unwrap();

    let (address, phrase, scheme) = keystore.generate_new_key(SignatureScheme::ED25519).unwrap();

    let keystore_path_2 = temp_dir.path().join("haneul2.keystore");
    let mut keystore2 = KeystoreType::File(keystore_path_2).init().unwrap();
    let imported_address = keystore2
        .import_from_mnemonic(&phrase, SignatureScheme::ED25519)
        .unwrap();
    assert_eq!(scheme.flag(), Ed25519HaneulSignature::SCHEME.flag());
    assert_eq!(address, imported_address);
}

/// This test confirms rust's implementation of mnemonic is the same with the Haneul Wallet
#[test]
fn haneul_wallet_address_mnemonic_test() -> Result<(), anyhow::Error> {
    // Recovery phase and HaneulAddress obtained from Haneul wallet v0.0.4 (prior key flag changes)
    let phrase = "oil puzzle immense upon pony govern jelly neck portion laptop laptop wall";
    let expected_address = HaneulAddress::from_str("0x6a06dd564dfb2f0c71f3e167a48f569c705ed34c")?;

    let temp_dir = TempDir::new().unwrap();
    let keystore_path = temp_dir.path().join("haneul.keystore");
    let mut keystore = KeystoreType::File(keystore_path).init().unwrap();

    keystore
        .import_from_mnemonic(phrase, SignatureScheme::ED25519)
        .unwrap();

    let pubkey = keystore.keys()[0].clone();
    assert_eq!(pubkey.flag(), Ed25519HaneulSignature::SCHEME.flag());

    let mut hasher = Sha3_256::default();
    hasher.update(pubkey);
    let g_arr = hasher.finalize();
    let mut res = [0u8; HANEUL_ADDRESS_LENGTH];
    res.copy_from_slice(&AsRef::<[u8]>::as_ref(&g_arr)[..HANEUL_ADDRESS_LENGTH]);
    let address = HaneulAddress::try_from(res.as_slice())?;

    assert_eq!(expected_address, address);

    Ok(())
}
