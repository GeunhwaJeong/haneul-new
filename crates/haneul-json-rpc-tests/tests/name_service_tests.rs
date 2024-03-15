// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use haneul_json_rpc::name_service::{self, Domain};
use haneul_types::{
    base_types::{ObjectID, HaneulAddress},
    collection_types::VecMap,
};

#[test]
fn test_parent_extraction() {
    let mut name = Domain::from_str("leaf.node.test.haneul").unwrap();

    assert_eq!(name.parent().to_string(), "node.test.haneul");

    name = Domain::from_str("node.test.haneul").unwrap();

    assert_eq!(name.parent().to_string(), "test.haneul");
}

#[test]
fn test_expirations() {
    let system_time: u64 = 100;

    let mut name = name_service::NameRecord {
        nft_id: haneul_types::id::ID::new(ObjectID::random()),
        data: VecMap { contents: vec![] },
        target_address: Some(HaneulAddress::random_for_testing_only()),
        expiration_timestamp_ms: system_time + 10,
    };

    assert!(!name.is_node_expired(system_time));

    name.expiration_timestamp_ms = system_time - 10;

    assert!(name.is_node_expired(system_time));
}

#[test]
fn test_name_service_outputs() {
    assert_eq!("@test".parse::<Domain>().unwrap().to_string(), "test.haneul");
    assert_eq!(
        "test.haneul".parse::<Domain>().unwrap().to_string(),
        "test.haneul"
    );
    assert_eq!(
        "test@sld".parse::<Domain>().unwrap().to_string(),
        "test.sld.haneul"
    );
    assert_eq!(
        "test.test@example".parse::<Domain>().unwrap().to_string(),
        "test.test.example.haneul"
    );
    assert_eq!(
        "haneul@haneul".parse::<Domain>().unwrap().to_string(),
        "haneul.haneul.haneul"
    );

    assert_eq!("@haneul".parse::<Domain>().unwrap().to_string(), "haneul.haneul");

    assert_eq!(
        "test*test@test".parse::<Domain>().unwrap().to_string(),
        "test.test.test.haneul"
    );
    assert_eq!(
        "test.test.haneul".parse::<Domain>().unwrap().to_string(),
        "test.test.haneul"
    );
    assert_eq!(
        "test.test.test.haneul".parse::<Domain>().unwrap().to_string(),
        "test.test.test.haneul"
    );
}

#[test]
fn test_different_wildcard() {
    assert_eq!("test.haneul".parse::<Domain>(), "test*haneul".parse::<Domain>(),);

    assert_eq!("@test".parse::<Domain>(), "test*haneul".parse::<Domain>(),);
}

#[test]
fn test_invalid_inputs() {
    assert!("*".parse::<Domain>().is_err());
    assert!(".".parse::<Domain>().is_err());
    assert!("@".parse::<Domain>().is_err());
    assert!("@inner.haneul".parse::<Domain>().is_err());
    assert!("@inner*haneul".parse::<Domain>().is_err());
    assert!("test@".parse::<Domain>().is_err());
    assert!("haneul".parse::<Domain>().is_err());
    assert!("test.test@example.haneul".parse::<Domain>().is_err());
    assert!("test@test@example".parse::<Domain>().is_err());
}
