// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::errors::IndexerError;
use crate::schema::packages;

use diesel::prelude::*;

use haneul_json_rpc_types::HaneulMovePackage;
use haneul_types::base_types::{ObjectID, HaneulAddress};

#[derive(Queryable, Insertable, Debug, Identifiable)]
#[diesel(table_name = packages)]
pub struct Package {
    pub id: Option<i64>,
    pub package_id: String,
    pub author: String,
    pub module_names: Vec<String>,
    pub package_content: String,
}

impl Package {
    pub fn try_from(
        id: ObjectID,
        sender: HaneulAddress,
        package: &HaneulMovePackage,
    ) -> Result<Self, IndexerError> {
        Ok(Self {
            id: None,
            package_id: id.to_hex(),
            author: sender.to_string(),
            module_names: package.disassembled.keys().cloned().collect(),
            // TODO: store raw package bytes instead when object refactoring is done.
            package_content: serde_json::to_string(&package.disassembled).map_err(|err| {
                IndexerError::InsertableParsingError(format!(
                    "Failed converting package module map to JSON with error: {:?}",
                    err
                ))
            })?,
        })
    }
}
