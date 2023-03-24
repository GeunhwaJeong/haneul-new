// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::base_types::HaneulAddress;
use crate::ObjectID;

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum TransactionFilter {
    /// Query by move function.
    MoveFunction {
        package: ObjectID,
        module: Option<String>,
        function: Option<String>,
    },
    /// Query by input object.
    InputObject(ObjectID),
    /// Query by changed object, including created, mutated and unwrapped objects.
    ChangedObject(ObjectID),
    /// Query by sender address.
    FromAddress(HaneulAddress),
    /// Query by recipient address.
    ToAddress(HaneulAddress),
    /// Query by sender and recipient address.
    FromAndToAddress { from: HaneulAddress, to: HaneulAddress },
    /// Query by transaction kind
    TransactionKind(String),
}
