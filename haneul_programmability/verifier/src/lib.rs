// Copyright (c) Haneul Labs
// SPDX-License-Identifier: Apache-2.0

pub mod verifier;

pub mod global_storage_access_verifier;
pub mod id_immutable_verifier;
pub mod id_leak_verifier;
pub mod param_typecheck_verifier;
pub mod struct_with_key_verifier;

use haneul_types::error::HaneulError;

fn verification_failure(error: String) -> HaneulError {
    HaneulError::ModuleVerificationFailure { error }
}
