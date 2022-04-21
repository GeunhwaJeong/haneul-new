// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_binary_format::CompiledModule;
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use haneul_framework::{self, DEFAULT_FRAMEWORK_PATH};
use haneul_types::base_types::{ObjectRef, HaneulAddress, TxContext};
use haneul_types::error::HaneulResult;
use haneul_types::HANEUL_FRAMEWORK_ADDRESS;
use haneul_types::{base_types::TransactionDigest, object::Object};

static GENESIS: Lazy<Mutex<Genesis>> = Lazy::new(|| {
    Mutex::new(create_genesis_module_objects(&PathBuf::from(DEFAULT_FRAMEWORK_PATH)).unwrap())
});

struct Genesis {
    pub objects: Vec<Object>,
    pub modules: Vec<Vec<CompiledModule>>,
}

pub fn clone_genesis_compiled_modules() -> Vec<Vec<CompiledModule>> {
    let genesis = GENESIS.lock().unwrap();
    genesis.modules.clone()
}

pub fn clone_genesis_packages() -> Vec<Object> {
    let genesis = GENESIS.lock().unwrap();
    genesis.objects.clone()
}

pub fn get_framework_object_ref() -> ObjectRef {
    let genesis = GENESIS.lock().unwrap();
    genesis
        .objects
        .iter()
        .find(|o| o.id() == HANEUL_FRAMEWORK_ADDRESS.into())
        .unwrap()
        .compute_object_reference()
}

pub fn get_genesis_context() -> TxContext {
    get_genesis_context_with_custom_address(&HaneulAddress::default())
}

pub fn get_genesis_context_with_custom_address(address: &HaneulAddress) -> TxContext {
    TxContext::new(address, &TransactionDigest::genesis())
}

/// Create and return objects wrapping the genesis modules for haneul
fn create_genesis_module_objects(lib_dir: &Path) -> HaneulResult<Genesis> {
    let haneul_modules = haneul_framework::get_haneul_framework_modules(lib_dir)?;
    let std_modules =
        haneul_framework::get_move_stdlib_modules(&lib_dir.join("deps").join("move-stdlib"))?;
    let objects = vec![
        Object::new_package(std_modules.clone(), TransactionDigest::genesis()),
        Object::new_package(haneul_modules.clone(), TransactionDigest::genesis()),
    ];
    let modules = vec![std_modules, haneul_modules];
    Ok(Genesis { objects, modules })
}
