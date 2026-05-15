// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_types::storage::{BackingPackageStore, ChildObjectResolver, StorageView};

pub trait HaneulResolver: BackingPackageStore {
    fn as_backing_package_store(&self) -> &dyn BackingPackageStore;
}

impl<T> HaneulResolver for T
where
    T: BackingPackageStore,
{
    fn as_backing_package_store(&self) -> &dyn BackingPackageStore {
        self
    }
}

/// Interface with the store necessary to execute a programmable transaction
pub trait ExecutionState: StorageView + HaneulResolver {
    fn as_child_resolver(&self) -> &dyn ChildObjectResolver;
}

impl<T> ExecutionState for T
where
    T: StorageView,
    T: HaneulResolver,
{
    fn as_child_resolver(&self) -> &dyn ChildObjectResolver {
        self
    }
}
