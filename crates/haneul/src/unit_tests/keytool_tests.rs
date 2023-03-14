// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crate::keytool::read_authority_keypair_from_file;
use crate::keytool::read_keypair_from_file;

use super::write_keypair_to_file;
use super::KeyToolCommand;
use fastcrypto::encoding::Base64;
use fastcrypto::encoding::Encoding;
use rand::rngs::StdRng;
use rand::SeedableRng;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentScope;
use haneul_keys::keystore::{AccountKeystore, FileBasedKeystore, InMemKeystore, Keystore};
use haneul_types::base_types::ObjectDigest;
use haneul_types::base_types::ObjectID;
use haneul_types::base_types::SequenceNumber;
use haneul_types::base_types::HaneulAddress;
use haneul_types::crypto::get_key_pair;
use haneul_types::crypto::get_key_pair_from_rng;
use haneul_types::crypto::AuthorityKeyPair;
use haneul_types::crypto::Ed25519HaneulSignature;
use haneul_types::crypto::EncodeDecodeBase64;
use haneul_types::crypto::Secp256k1HaneulSignature;
use haneul_types::crypto::Secp256r1HaneulSignature;
use haneul_types::crypto::Signature;
use haneul_types::crypto::SignatureScheme;
use haneul_types::crypto::HaneulKeyPair;
use haneul_types::crypto::HaneulSignatureInner;
use haneul_types::messages::TransactionData;
use tempfile::TempDir;

const TEST_MNEMONIC: &str = "result crisp session latin must fruit genuine question prevent start coconut brave speak student dismiss";

#[test]
fn test_addresses_command() -> Result<(), anyhow::Error> {
    // Add 3 Ed25519 KeyPairs as default
    let mut keystore = Keystore::from(InMemKeystore::new(3));

    // Add another 3 Secp256k1 KeyPairs
    for _ in 0..3 {
        keystore.add_key(HaneulKeyPair::Secp256k1(get_key_pair().1))?;
    }

    // List all addresses with flag
    KeyToolCommand::List.execute(&mut keystore).unwrap();
    Ok(())
}

#[test]
fn test_flag_in_signature_and_keypair() -> Result<(), anyhow::Error> {
    let mut keystore = Keystore::from(InMemKeystore::new(0));

    keystore.add_key(HaneulKeyPair::Secp256k1(get_key_pair().1))?;
    keystore.add_key(HaneulKeyPair::Ed25519(get_key_pair().1))?;

    for pk in keystore.keys() {
        let pk1 = pk.clone();
        let sig = keystore.sign_secure(&(&pk).into(), b"hello", Intent::default())?;
        match sig {
            Signature::Ed25519HaneulSignature(_) => {
                // signature contains corresponding flag
                assert_eq!(
                    *sig.as_ref().first().unwrap(),
                    Ed25519HaneulSignature::SCHEME.flag()
                );
                // keystore stores pubkey with corresponding flag
                assert!(pk1.flag() == Ed25519HaneulSignature::SCHEME.flag())
            }
            Signature::Secp256k1HaneulSignature(_) => {
                assert_eq!(
                    *sig.as_ref().first().unwrap(),
                    Secp256k1HaneulSignature::SCHEME.flag()
                );
                assert!(pk1.flag() == Secp256k1HaneulSignature::SCHEME.flag())
            }
            Signature::Secp256r1HaneulSignature(_) => {
                assert_eq!(
                    *sig.as_ref().first().unwrap(),
                    Secp256r1HaneulSignature::SCHEME.flag()
                );
                assert!(pk1.flag() == Secp256r1HaneulSignature::SCHEME.flag())
            }
        }
    }
    Ok(())
}

#[test]
fn test_read_write_keystore_with_flag() {
    let dir = tempfile::TempDir::new().unwrap();

    // create Secp256k1 keypair
    let kp_secp = HaneulKeyPair::Secp256k1(get_key_pair().1);
    let addr_secp: HaneulAddress = (&kp_secp.public()).into();
    let fp_secp = dir.path().join(format!("{}.key", addr_secp));
    let fp_secp_2 = fp_secp.clone();

    // write Secp256k1 keypair to file
    let res = write_keypair_to_file(&kp_secp, &fp_secp);
    assert!(res.is_ok());

    // read from file as enum KeyPair success
    let kp_secp_read = read_keypair_from_file(fp_secp);
    assert!(kp_secp_read.is_ok());

    // KeyPair wrote into file is the same as read
    assert_eq!(
        kp_secp_read.unwrap().public().as_ref(),
        kp_secp.public().as_ref()
    );

    // read as AuthorityKeyPair fails
    let kp_secp_read = read_authority_keypair_from_file(fp_secp_2);
    assert!(kp_secp_read.is_err());

    // create Ed25519 keypair
    let kp_ed = HaneulKeyPair::Ed25519(get_key_pair().1);
    let addr_ed: HaneulAddress = (&kp_ed.public()).into();
    let fp_ed = dir.path().join(format!("{}.key", addr_ed));
    let fp_ed_2 = fp_ed.clone();

    // write Ed25519 keypair to file
    let res = write_keypair_to_file(&kp_ed, &fp_ed);
    assert!(res.is_ok());

    // read from file as enum KeyPair success
    let kp_ed_read = read_keypair_from_file(fp_ed);
    assert!(kp_ed_read.is_ok());

    // KeyPair wrote into file is the same as read
    assert_eq!(
        kp_ed_read.unwrap().public().as_ref(),
        kp_ed.public().as_ref()
    );

    // read from file as AuthorityKeyPair success
    let kp_ed_read = read_authority_keypair_from_file(fp_ed_2);
    assert!(kp_ed_read.is_err());
}

#[test]
fn test_haneul_operations_config() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("haneul.keystore");
    let path1 = path.clone();
    // This is the hardcoded keystore in haneul-operation: https://github.com/GeunhwaJeong/haneul-operations/blob/af04c9d3b61610dbb36401aff6bef29d06ef89f8/docker/config/generate/static/haneul.keystore
    // If this test fails, address hardcoded in haneul-operations is likely needed be updated.
    let kp = HaneulKeyPair::decode_base64("ANRj4Rx5FZRehqwrctiLgZDPrY/3tI5+uJLCdaXPCj6C").unwrap();
    let contents = kp.encode_base64();
    let res = std::fs::write(path, contents);
    assert!(res.is_ok());
    let kp_read = read_keypair_from_file(path1);
    assert_eq!(
        HaneulAddress::from_str("7d20dcdb2bca4f508ea9613994683eb4e76e9c4ed371169677c1be02aaf0b58e")
            .unwrap(),
        HaneulAddress::from(&kp_read.unwrap().public())
    );

    // This is the hardcoded keystore in haneul-operation: https://github.com/GeunhwaJeong/haneul-operations/blob/af04c9d3b61610dbb36401aff6bef29d06ef89f8/docker/config/generate/static/haneul-benchmark.keystore
    // If this test fails, address hardcoded in haneul-operations is likely needed be updated.
    let path2 = temp_dir.path().join("haneul-benchmark.keystore");
    let path3 = path2.clone();
    let kp = HaneulKeyPair::decode_base64("APCWxPNCbgGxOYKeMfPqPmXmwdNVyau9y4IsyBcmC14A").unwrap();
    let contents = kp.encode_base64();
    let res = std::fs::write(path2, contents);
    assert!(res.is_ok());
    let kp_read = read_keypair_from_file(path3);
    assert_eq!(
        HaneulAddress::from_str("160ef6ce4f395208a12119c5011bf8d8ceb760e3159307c819bd0197d154d384")
            .unwrap(),
        HaneulAddress::from(&kp_read.unwrap().public())
    );
}

#[test]
fn test_load_keystore_err() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("haneul.keystore");
    let path2 = path.clone();

    // write encoded AuthorityKeyPair without flag byte to file
    let kp: AuthorityKeyPair = get_key_pair_from_rng(&mut StdRng::from_seed([0; 32])).1;
    let contents = kp.encode_base64();
    let res = std::fs::write(path, contents);
    assert!(res.is_ok());

    // cannot load keypair due to missing flag
    assert!(FileBasedKeystore::new(&path2).is_err());
}

#[test]
fn test_mnemonics_ed25519() -> Result<(), anyhow::Error> {
    // Test case matches with /haneullabs/haneul/sdk/typescript/test/unit/cryptography/ed25519-keypair.test.ts
    const TEST_CASES: [[&str; 3]; 3] = [["film crazy soon outside stand loop subway crumble thrive popular green nuclear struggle pistol arm wife phrase warfare march wheat nephew ask sunny firm", "AN0JMHpDum3BhrVwnkylH0/HGRHBQ/fO/8+MYOawO8j6", "a2d14fad60c56049ecf75246a481934691214ce413e6a8ae2fe6834c173a6133"],
    ["require decline left thought grid priority false tiny gasp angle royal system attack beef setup reward aunt skill wasp tray vital bounce inflict level", "AJrA997C1eVz6wYIp7bO8dpITSRBXpvg1m70/P3gusu2", "1ada6e6f3f3e4055096f606c746690f1108fcc2ca479055cc434a3e1d3f758aa"],
    ["organ crash swim stick traffic remember army arctic mesh slice swear summer police vast chaos cradle squirrel hood useless evidence pet hub soap lake", "AAEMSIQeqyz09StSwuOW4MElQcZ+4jHW4/QcWlJEf5Yk", "e69e896ca10f5a77732769803cc2b5707f0ab9d4407afb5e4b4464b89769af14"]];

    for t in TEST_CASES {
        let mut keystore = Keystore::from(InMemKeystore::new(0));
        KeyToolCommand::Import {
            mnemonic_phrase: t[0].to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: None,
        }
        .execute(&mut keystore)?;
        let kp = HaneulKeyPair::decode_base64(t[1]).unwrap();
        let addr = HaneulAddress::from_str(t[2]).unwrap();
        assert_eq!(HaneulAddress::from(&kp.public()), addr);
        assert!(keystore.addresses().contains(&addr));
    }
    Ok(())
}

#[test]
fn test_mnemonics_secp256k1() -> Result<(), anyhow::Error> {
    // Test case matches with /haneullabs/haneul/sdk/typescript/test/unit/cryptography/secp256k1-keypair.test.ts
    const TEST_CASES: [[&str; 3]; 3] = [["film crazy soon outside stand loop subway crumble thrive popular green nuclear struggle pistol arm wife phrase warfare march wheat nephew ask sunny firm", "AQA9EYZoLXirIahsXHQMDfdi5DPQ72wLA79zke4EY6CP", "9e8f732575cc5386f8df3c784cd3ed1b53ce538da79926b2ad54dcc1197d2532"],
    ["require decline left thought grid priority false tiny gasp angle royal system attack beef setup reward aunt skill wasp tray vital bounce inflict level", "Ae+TTptXI6WaJfzplSrphnrbTD5qgftfMX5kTyca7unQ", "9fd5a804ed6b46d36949ff7434247f0fd594673973ece24aede6b86a7b5dae01"],
    ["organ crash swim stick traffic remember army arctic mesh slice swear summer police vast chaos cradle squirrel hood useless evidence pet hub soap lake", "AY2iJpGSDMhvGILPjjpyeM1bV4Jky979nUenB5kvQeSj", "60287d7c38dee783c2ab1077216124011774be6b0764d62bd05f32c88979d5c5"]];

    for t in TEST_CASES {
        let mut keystore = Keystore::from(InMemKeystore::new(0));
        KeyToolCommand::Import {
            mnemonic_phrase: t[0].to_string(),
            key_scheme: SignatureScheme::Secp256k1,
            derivation_path: None,
        }
        .execute(&mut keystore)?;
        let kp = HaneulKeyPair::decode_base64(t[1]).unwrap();
        let addr = HaneulAddress::from_str(t[2]).unwrap();
        assert_eq!(HaneulAddress::from(&kp.public()), addr);
        assert!(keystore.addresses().contains(&addr));
    }
    Ok(())
}

#[test]
fn test_invalid_derivation_path() -> Result<(), anyhow::Error> {
    let mut keystore = Keystore::from(InMemKeystore::new(0));
    assert!(KeyToolCommand::Import {
        mnemonic_phrase: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/44'/1'/0'/0/0".parse().unwrap()),
    }
    .execute(&mut keystore)
    .is_err());

    assert!(KeyToolCommand::Import {
        mnemonic_phrase: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/0'/784'/0'/0/0".parse().unwrap()),
    }
    .execute(&mut keystore)
    .is_err());

    assert!(KeyToolCommand::Import {
        mnemonic_phrase: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/54'/8282'/0'/0/0".parse().unwrap()),
    }
    .execute(&mut keystore)
    .is_err());

    assert!(KeyToolCommand::Import {
        mnemonic_phrase: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::Secp256k1,
        derivation_path: Some("m/54'/8282'/0'/0'/0'".parse().unwrap()),
    }
    .execute(&mut keystore)
    .is_err());

    assert!(KeyToolCommand::Import {
        mnemonic_phrase: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::Secp256k1,
        derivation_path: Some("m/44'/8282'/0'/0/0".parse().unwrap()),
    }
    .execute(&mut keystore)
    .is_err());

    Ok(())
}

#[test]
fn test_valid_derivation_path() -> Result<(), anyhow::Error> {
    let mut keystore = Keystore::from(InMemKeystore::new(0));
    assert!(KeyToolCommand::Import {
        mnemonic_phrase: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/44'/8282'/0'/0'/0'".parse().unwrap()),
    }
    .execute(&mut keystore)
    .is_ok());

    assert!(KeyToolCommand::Import {
        mnemonic_phrase: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/44'/8282'/0'/0'/1'".parse().unwrap()),
    }
    .execute(&mut keystore)
    .is_ok());

    assert!(KeyToolCommand::Import {
        mnemonic_phrase: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::ED25519,
        derivation_path: Some("m/44'/8282'/1'/0'/1'".parse().unwrap()),
    }
    .execute(&mut keystore)
    .is_ok());

    assert!(KeyToolCommand::Import {
        mnemonic_phrase: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::Secp256k1,
        derivation_path: Some("m/54'/8282'/0'/0/1".parse().unwrap()),
    }
    .execute(&mut keystore)
    .is_ok());

    assert!(KeyToolCommand::Import {
        mnemonic_phrase: TEST_MNEMONIC.to_string(),
        key_scheme: SignatureScheme::Secp256k1,
        derivation_path: Some("m/54'/8282'/1'/0/1".parse().unwrap()),
    }
    .execute(&mut keystore)
    .is_ok());
    Ok(())
}

#[test]
fn test_keytool_bls12381() -> Result<(), anyhow::Error> {
    let mut keystore = Keystore::from(InMemKeystore::new(0));
    KeyToolCommand::Generate {
        key_scheme: SignatureScheme::BLS12381,
        derivation_path: None,
    }
    .execute(&mut keystore)?;
    Ok(())
}

#[test]
fn test_sign_command() -> Result<(), anyhow::Error> {
    // Add a keypair
    let mut keystore = Keystore::from(InMemKeystore::new(1));
    let binding = keystore.addresses();
    let sender = binding.first().unwrap();

    // Create a dummy TransactionData
    let gas = (
        ObjectID::random(),
        SequenceNumber::new(),
        ObjectDigest::random(),
    );
    let tx_data = TransactionData::new_pay_haneul_with_dummy_gas_price(
        *sender,
        vec![gas],
        vec![HaneulAddress::random_for_testing_only()],
        vec![10000],
        gas,
        1000,
    )
    .unwrap();

    // Sign an intent message for the transaction data and a passed-in intent with scope as PersonalMessage.
    KeyToolCommand::Sign {
        address: *sender,
        data: Base64::encode(bcs::to_bytes(&tx_data)?),
        intent: Some(Intent::default().with_scope(IntentScope::PersonalMessage)),
    }
    .execute(&mut keystore)?;

    // Sign an intent message for the transaction data without intent passed in, so default is used.
    KeyToolCommand::Sign {
        address: *sender,
        data: Base64::encode(bcs::to_bytes(&tx_data)?),
        intent: None,
    }
    .execute(&mut keystore)?;
    Ok(())
}
