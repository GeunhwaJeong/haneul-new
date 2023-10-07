// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::move_object::MoveObject;
use async_graphql::*;

pub(crate) struct StakedHaneul {
    pub move_obj: MoveObject,
}

#[Object]
impl StakedHaneul {
    // TODO: implement these fields
    // status: StakeStatus
    // requestEpoch: Epoch
    // activeEpoch: Epoch
    // principal: BigInt

    async fn as_move_object(&self) -> Option<MoveObject> {
        Some(self.move_obj.clone())
    }
}
