// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use haneul_framework::{self, DEFAULT_FRAMEWORK_PATH};
use haneul_types::error::HaneulResult;
use haneul_types::{
    base_types::{HaneulAddress, TransactionDigest},
    object::Object,
};

static GENESIS: Lazy<Mutex<Genesis>> = Lazy::new(|| {
    Mutex::new(create_genesis_module_objects(&PathBuf::from(DEFAULT_FRAMEWORK_PATH)).unwrap())
});

struct Genesis {
    pub objects: Vec<Object>,
}

pub fn clone_genesis_modules() -> Vec<Object> {
    let genesis = GENESIS.lock().unwrap();
    genesis.objects.clone()
}

/// Create and return objects wrapping the genesis modules for fastX
fn create_genesis_module_objects(lib_dir: &Path) -> HaneulResult<Genesis> {
    let haneul_modules = haneul_framework::get_haneul_framework_modules(lib_dir)?;
    let std_modules =
        haneul_framework::get_move_stdlib_modules(&lib_dir.join("deps").join("move-stdlib"))?;
    let owner = HaneulAddress::default();
    let objects = vec![
        Object::new_package(haneul_modules, owner, TransactionDigest::genesis()),
        Object::new_package(std_modules, owner, TransactionDigest::genesis()),
    ];
    Ok(Genesis { objects })
}
