// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_binary_format::access::ModuleAccess;
use haneul_framework::{MoveStdlib, HaneulFramework, HaneulSystem, SystemPackage};
use haneul_json_rpc::api::ReadApiClient;
use haneul_json_rpc_types::HaneulObjectResponse;
use haneul_types::{
    base_types::ObjectID, digests::TransactionDigest, object::Object, HANEUL_FRAMEWORK_ADDRESS,
};
use test_utils::network::TestClusterBuilder;

use haneul_macros::sim_test;

#[sim_test]
async fn test_additional_objects() {
    // Test the ability to add additional objects into genesis for test clusters
    let id = ObjectID::random();
    let cluster = TestClusterBuilder::new()
        .with_objects([Object::immutable_with_id_for_testing(id)])
        .build()
        .await
        .unwrap();

    let client = cluster.rpc_client();
    let resp = client.get_object_with_options(id, None).await.unwrap();
    assert!(matches!(resp, HaneulObjectResponse { data: Some(_), .. }));
}

#[sim_test]
async fn test_package_override() {
    // `with_objects` can be used to override existing packages.
    let framework_ref = {
        let default_cluster = TestClusterBuilder::new().build().await.unwrap();
        let client = default_cluster.rpc_client();
        let obj = client
            .get_object_with_options(HaneulSystem::ID, None)
            .await
            .unwrap();

        if let Some(obj) = obj.data {
            obj.object_ref()
        } else {
            panic!("Original framework package should exist");
        }
    };

    let modified_ref = {
        let mut framework_modules = HaneulSystem::as_modules();

        // Create an empty module that is pretending to be part of the haneul framework.
        let mut test_module = move_binary_format::file_format::empty_module();
        let address_idx = test_module.self_handle().address.0 as usize;
        test_module.address_identifiers[address_idx] = HANEUL_FRAMEWORK_ADDRESS;

        // Add the dummy module to the rest of the haneul-frameworks.  We can't replace the framework
        // entirely because we will call into it for genesis.
        framework_modules.push(test_module);

        let package_override = Object::new_package_for_testing(
            framework_modules,
            TransactionDigest::genesis(),
            &[MoveStdlib::as_package(), HaneulFramework::as_package()],
        )
        .unwrap();

        let modified_cluster = TestClusterBuilder::new()
            .with_objects([package_override])
            .build()
            .await
            .unwrap();

        let client = modified_cluster.rpc_client();
        let obj = client
            .get_object_with_options(HaneulSystem::ID, None)
            .await
            .unwrap();

        if let Some(obj) = obj.data {
            obj.object_ref()
        } else {
            panic!("Original framework package should exist");
        }
    };

    assert_ne!(framework_ref, modified_ref);
}
