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

pub const ID_MODULE_NAME: &IdentStr = ident_str!("id");
pub const VERSIONED_ID_STRUCT_NAME: &IdentStr = ident_str!("VersionedID");
pub const UNIQUE_ID_STRUCT_NAME: &IdentStr = ident_str!("UniqueID");
pub const ID_STRUCT_NAME: &IdentStr = ident_str!("ID");

/// Rust version of the Move haneul::id::VersionedID type
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Eq, PartialEq)]
pub struct VersionedID {
    pub id: UniqueID,
    pub version: u64,
}

/// Rust version of the Move haneul::id::UniqueID type
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(transparent)]
pub struct UniqueID {
    pub id: ID,
}

/// Rust version of the Move haneul::id::ID type
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Eq, PartialEq)]
#[serde(transparent)]
pub struct ID {
    pub bytes: ObjectID,
}

impl VersionedID {
    pub fn new(bytes: ObjectID, version: SequenceNumber) -> Self {
        Self {
            id: UniqueID {
                id: { ID { bytes } },
            },
            version: version.value(),
        }
    }

    pub fn type_() -> StructTag {
        StructTag {
            address: HANEUL_FRAMEWORK_ADDRESS,
            module: ID_MODULE_NAME.to_owned(),
            name: VERSIONED_ID_STRUCT_NAME.to_owned(),
            type_params: Vec::new(),
        }
    }

    pub fn object_id(&self) -> &ObjectID {
        &self.id.id.bytes
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
                    MoveTypeLayout::Struct(UniqueID::layout()),
                ),
                MoveFieldLayout::new(ident_str!("version").to_owned(), MoveTypeLayout::U64),
            ],
        }
    }
}

impl UniqueID {
    pub fn type_() -> StructTag {
        StructTag {
            address: HANEUL_FRAMEWORK_ADDRESS,
            module: ID_MODULE_NAME.to_owned(),
            name: UNIQUE_ID_STRUCT_NAME.to_owned(),
            type_params: Vec::new(),
        }
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
            module: ID_MODULE_NAME.to_owned(),
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
