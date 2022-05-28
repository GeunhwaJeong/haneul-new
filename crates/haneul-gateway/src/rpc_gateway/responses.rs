// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::language_storage::TypeTag;
use move_core_types::parser::parse_type_tag;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use haneul_core::gateway_types::{HaneulData, HaneulObjectRef};

use haneul_types::base_types::TransactionDigest;
use haneul_types::object::Owner;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ObjectExistsResponse {
    object_ref: HaneulObjectRef,
    owner: Owner,
    previous_transaction: TransactionDigest,
    data: HaneulData,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ObjectNotExistsResponse {
    object_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename = "TypeTag")]
pub struct HaneulTypeTag(String);

impl TryInto<TypeTag> for HaneulTypeTag {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<TypeTag, Self::Error> {
        parse_type_tag(&self.0)
    }
}

impl From<TypeTag> for HaneulTypeTag {
    fn from(tag: TypeTag) -> Self {
        Self(format!("{}", tag))
    }
}
