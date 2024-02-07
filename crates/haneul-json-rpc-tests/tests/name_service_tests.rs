// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use haneul_json_rpc::name_service;
use haneul_types::{
    base_types::{ObjectID, HaneulAddress},
    collection_types::VecMap,
};

#[test]
fn test_parent_extraction() {
    let mut name = name_service::Domain::from_str("leaf.node.test.haneul").unwrap();

    assert_eq!(name.parent().to_string(), "node.test.haneul");

    name = name_service::Domain::from_str("node.test.haneul").unwrap();

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
