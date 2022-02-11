// Copyright (c) Haneul Labs
// SPDX-License-Identifier: Apache-2.0

//! This module contains the public APIs supported by the bytecode verifier.

use move_binary_format::file_format::CompiledModule;
use haneul_types::error::HaneulResult;

use crate::{
    global_storage_access_verifier, id_immutable_verifier, id_leak_verifier,
    struct_with_key_verifier,
};

/// Helper for a "canonical" verification of a module.
pub fn verify_module(module: &CompiledModule) -> HaneulResult {
    struct_with_key_verifier::verify_module(module)?;
    global_storage_access_verifier::verify_module(module)?;
    id_immutable_verifier::verify_module(module)?;
    id_leak_verifier::verify_module(module)
}
