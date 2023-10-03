// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use balance_changes::*;
pub use object_changes::*;
pub use haneul_checkpoint::*;
pub use haneul_coin::*;
pub use haneul_event::*;
pub use haneul_extended::*;
pub use haneul_governance::*;
pub use haneul_move::*;
pub use haneul_object::*;
pub use haneul_protocol::*;
pub use haneul_transaction::*;
use haneul_types::base_types::ObjectID;
use haneul_types::dynamic_field::DynamicFieldInfo;

#[cfg(test)]
#[path = "unit_tests/rpc_types_tests.rs"]
mod rpc_types_tests;

mod balance_changes;
mod object_changes;
mod haneul_checkpoint;
mod haneul_coin;
mod haneul_event;
mod haneul_extended;
mod haneul_governance;
mod haneul_move;
mod haneul_object;
mod haneul_protocol;
mod haneul_transaction;

pub type DynamicFieldPage = Page<DynamicFieldInfo, ObjectID>;
/// `next_cursor` points to the last item in the page;
/// Reading with `next_cursor` will start from the next item after `next_cursor` if
/// `next_cursor` is `Some`, otherwise it will start from the first item.
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Page<T, C> {
    pub data: Vec<T>,
    pub next_cursor: Option<C>,
    pub has_next_page: bool,
}

impl<T, C> Page<T, C> {
    pub fn empty() -> Self {
        Self {
            data: vec![],
            next_cursor: None,
            has_next_page: false,
        }
    }
}
