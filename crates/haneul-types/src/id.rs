// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    base_types::{ObjectID, SequenceNumber},
    HANEUL_FRAMEWORK_ADDRESS,
};
use move_core_types::{
    ident_str,
    identifier::IdentStr,
    language_storage::StructTag,
    value::{MoveFieldLayout, MoveStructLayout, MoveTypeLayout},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const OBJECT_MODULE_NAME: &IdentStr = ident_str!("object");
pub const INFO_STRUCT_NAME: &IdentStr = ident_str!("Info");
pub const ID_STRUCT_NAME: &IdentStr = ident_str!("ID");

/// Rust version of the Move haneul::object::Info type
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Eq, PartialEq)]
pub struct Info {
    pub id: ID,
    pub version: u64,
    // pub child_count: Option<u64>,
}

/// Rust version of the Move haneul::object::ID type
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(transparent)]
pub struct ID {
    pub bytes: ObjectID,
}

impl Info {
    pub fn new(bytes: ObjectID, version: SequenceNumber) -> Self {
        Self {
            id: { ID { bytes } },
            version: version.value(),
            // child_count: None,
        }
    }

    pub fn type_() -> StructTag {
        StructTag {
            address: HANEUL_FRAMEWORK_ADDRESS,
            module: OBJECT_MODULE_NAME.to_owned(),
            name: INFO_STRUCT_NAME.to_owned(),
            type_params: Vec::new(),
        }
    }

    pub fn object_id(&self) -> &ObjectID {
        &self.id.bytes
    }

    pub fn version(&self) -> SequenceNumber {
        SequenceNumber::from(self.version)
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }

    pub fn layout() -> MoveStructLayout {
        MoveStructLayout::WithTypes {
            type_: Self::type_(),
            fields: vec![
                MoveFieldLayout::new(
                    ident_str!("id").to_owned(),
                    MoveTypeLayout::Struct(ID::layout()),
                ),
                MoveFieldLayout::new(ident_str!("version").to_owned(), MoveTypeLayout::U64),
                // MoveFieldLayout::new(
                //     ident_str!("child_count").to_owned(),
                //     MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U64)),
                // ),
            ],
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
