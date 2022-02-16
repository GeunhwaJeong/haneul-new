// Copyright (c) Haneul Labs
// SPDX-License-Identifier: Apache-2.0

use move_vm_runtime::native_functions::NativeFunctionTable;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use haneul_framework::{self};
use haneul_types::{
    base_types::{HaneulAddress, TransactionDigest},
    object::Object,
    MOVE_STDLIB_ADDRESS, HANEUL_FRAMEWORK_ADDRESS,
};

static GENESIS: Lazy<Mutex<Genesis>> = Lazy::new(|| Mutex::new(create_genesis_module_objects()));

struct Genesis {
    pub objects: Vec<Object>,
    pub native_functions: NativeFunctionTable,
}

pub fn clone_genesis_data() -> (Vec<Object>, NativeFunctionTable) {
    let genesis = GENESIS.lock().unwrap();
    (genesis.objects.clone(), genesis.native_functions.clone())
}

/// Create and return objects wrapping the genesis modules for fastX
fn create_genesis_module_objects() -> Genesis {
    let haneul_modules = haneul_framework::get_haneul_framework_modules();
    let std_modules = haneul_framework::get_move_stdlib_modules();
    let native_functions =
        haneul_framework::natives::all_natives(MOVE_STDLIB_ADDRESS, HANEUL_FRAMEWORK_ADDRESS);
    let owner = HaneulAddress::default();
    let objects = vec![
        Object::new_package(haneul_modules, owner, TransactionDigest::genesis()),
        Object::new_package(std_modules, owner, TransactionDigest::genesis()),
    ];
    Genesis {
        objects,
        native_functions,
    }
}
