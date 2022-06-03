// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module contains the public APIs supported by the bytecode verifier.

use move_binary_format::file_format::CompiledModule;
use haneul_types::error::HaneulResult;

use crate::{
    entry_points_verifier, global_storage_access_verifier, id_immutable_verifier, id_leak_verifier,
    private_transfer, struct_with_key_verifier,
};

/// Helper for a "canonical" verification of a module.
pub fn verify_module(module: &CompiledModule) -> HaneulResult {
    struct_with_key_verifier::verify_module(module)?;
    global_storage_access_verifier::verify_module(module)?;
    id_immutable_verifier::verify_module(module)?;
    id_leak_verifier::verify_module(module)?;
    private_transfer::verify_module(module)?;
    entry_points_verifier::verify_module(module)
}
