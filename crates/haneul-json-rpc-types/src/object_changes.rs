// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::language_storage::StructTag;
use serde_with::DisplayFromStr;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use haneul_types::base_types::{ObjectDigest, ObjectID, SequenceNumber, HaneulAddress};
use haneul_types::object::Owner;

/// ObjectChange are derived from the object mutations in the TransactionEffect to provide richer object information.
#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ObjectChange {
    /// Module published
    Published {
        package_id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
        modules: Vec<String>,
    },
    /// Transfer objects to new address / wrap in another object
    Transferred {
        sender: HaneulAddress,
        recipient: Owner,
        #[schemars(with = "String")]
        #[serde_as(as = "DisplayFromStr")]
        object_type: StructTag,
        object_id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
    },
    /// Object mutated.
    Mutated {
        sender: HaneulAddress,
        owner: Owner,
        #[schemars(with = "String")]
        #[serde_as(as = "DisplayFromStr")]
        object_type: StructTag,
        object_id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
    },
    /// Delete object
    Deleted {
        sender: HaneulAddress,
        #[schemars(with = "String")]
        #[serde_as(as = "DisplayFromStr")]
        object_type: StructTag,
        object_id: ObjectID,
        version: SequenceNumber,
    },
    /// Wrapped object
    Wrapped {
        sender: HaneulAddress,
        #[schemars(with = "String")]
        #[serde_as(as = "DisplayFromStr")]
        object_type: StructTag,
        object_id: ObjectID,
        version: SequenceNumber,
    },
    /// New object creation
    Created {
        sender: HaneulAddress,
        owner: Owner,
        #[schemars(with = "String")]
        #[serde_as(as = "DisplayFromStr")]
        object_type: StructTag,
        object_id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
    },
}
