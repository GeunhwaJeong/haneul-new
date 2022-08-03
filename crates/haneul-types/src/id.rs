// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{base_types::ObjectID, HANEUL_FRAMEWORK_ADDRESS};
use move_core_types::{
    ident_str,
    identifier::IdentStr,
    language_storage::StructTag,
    value::{MoveFieldLayout, MoveStructLayout, MoveTypeLayout},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const OBJECT_MODULE_NAME: &IdentStr = ident_str!("object");
pub const UID_STRUCT_NAME: &IdentStr = ident_str!("UID");
pub const ID_STRUCT_NAME: &IdentStr = ident_str!("ID");

/// Rust version of the Move haneul::object::Info type
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Eq, PartialEq)]
pub struct UID {
    pub id: ID,
}

/// Rust version of the Move haneul::object::ID type
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(transparent)]
pub struct ID {
    pub bytes: ObjectID,
}

impl UID {
    pub fn new(bytes: ObjectID) -> Self {
        Self {
            id: { ID { bytes } },
        }
    }

    pub fn type_() -> StructTag {
        StructTag {
            address: HANEUL_FRAMEWORK_ADDRESS,
            module: OBJECT_MODULE_NAME.to_owned(),
            name: UID_STRUCT_NAME.to_owned(),
            type_params: Vec::new(),
        }
    }

    pub fn object_id(&self) -> &ObjectID {
        &self.id.bytes
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }

    pub fn layout() -> MoveStructLayout {
        MoveStructLayout::WithTypes {
            type_: Self::type_(),
            fields: vec![MoveFieldLayout::new(
                ident_str!("id").to_owned(),
                MoveTypeLayout::Struct(ID::layout()),
            )],
        }
    }
}

impl ID {
    pub fn type_() -> StructTag {
        StructTag {
            address: HANEUL_FRAMEWORK_ADDRESS,
            module: OBJECT_MODULE_NAME.to_owned(),
            name: ID_STRUCT_NAME.to_owned(),
            type_params: Vec::new(),
        }
    }

    pub fn layout() -> MoveStructLayout {
        MoveStructLayout::WithTypes {
            type_: Self::type_(),
            fields: vec![MoveFieldLayout::new(
                ident_str!("bytes").to_owned(),
                MoveTypeLayout::Address,
            )],
        }
    }
}
