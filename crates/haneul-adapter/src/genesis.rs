// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_binary_format::CompiledModule;
use once_cell::sync::Lazy;
use haneul_types::base_types::{ObjectRef, HaneulAddress, TxContext};
use haneul_types::clock::Clock;
use haneul_types::epoch_data::EpochData;
use haneul_types::id::UID;
use haneul_types::object::{MoveObject, Owner};
use haneul_types::{base_types::TransactionDigest, object::Object};
use haneul_types::{HANEUL_CLOCK_OBJECT_ID, HANEUL_CLOCK_OBJECT_SHARED_VERSION, HANEUL_FRAMEWORK_ADDRESS};

static GENESIS: Lazy<Genesis> = Lazy::new(create_genesis_module_objects);

struct Genesis {
    pub objects: Vec<Object>,
    pub packages: Vec<Object>,
    pub modules: Vec<Vec<CompiledModule>>,
}

pub fn clone_genesis_compiled_modules() -> Vec<Vec<CompiledModule>> {
    GENESIS.modules.clone()
}

pub fn clone_genesis_packages() -> Vec<Object> {
    GENESIS.packages.clone()
}

pub fn clone_genesis_objects() -> Vec<Object> {
    GENESIS.objects.clone()
}

pub fn get_framework_object_ref() -> ObjectRef {
    GENESIS
        .packages
        .iter()
        .find(|o| o.id() == HANEUL_FRAMEWORK_ADDRESS.into())
        .unwrap()
        .compute_object_reference()
}

pub fn get_genesis_context() -> TxContext {
    TxContext::new(
        &HaneulAddress::default(),
        &TransactionDigest::genesis(),
        &EpochData::genesis(),
    )
}

/// Create and return objects wrapping the genesis modules for haneul
fn create_genesis_module_objects() -> Genesis {
    let haneul_modules = haneul_framework::get_haneul_framework();
    let std_modules = haneul_framework::get_move_stdlib();
    let objects = vec![create_clock()];
    // SAFETY: unwraps safe because genesis packages should never exceed max size
    let packages = vec![
        Object::new_package(std_modules.clone(), TransactionDigest::genesis()).unwrap(),
        Object::new_package(haneul_modules.clone(), TransactionDigest::genesis()).unwrap(),
    ];
    let modules = vec![std_modules, haneul_modules];
    Genesis {
        objects,
        packages,
        modules,
    }
}

fn create_clock() -> Object {
    // SAFETY: unwrap safe because genesis objects should be serializable
    let contents = bcs::to_bytes(&Clock {
        id: UID::new(HANEUL_CLOCK_OBJECT_ID),
        timestamp_ms: 0,
    })
    .unwrap();

    // SAFETY: Whether `Clock` has public transfer or not is statically known, and unwrap safe
    // because genesis objects should never exceed max size
    let move_object = unsafe {
        let has_public_transfer = false;
        MoveObject::new_from_execution(
            Clock::type_(),
            has_public_transfer,
            HANEUL_CLOCK_OBJECT_SHARED_VERSION,
            contents,
        )
        .unwrap()
    };

    Object::new_move(
        move_object,
        Owner::Shared {
            initial_shared_version: HANEUL_CLOCK_OBJECT_SHARED_VERSION,
        },
        TransactionDigest::genesis(),
    )
}
