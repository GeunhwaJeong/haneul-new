// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_types::error::ExecutionError;
use move_core_types::language_storage::TypeTag;
use move_vm_types::loaded_data::runtime_types::Type;

pub trait TypeTagResolver {
    fn get_type_tag(&self, type_: &Type) -> Result<TypeTag, ExecutionError>;
}
