// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    base_types::SequenceNumber, error::HaneulResult, object::Owner, storage::ObjectStore,
    HANEUL_COIN_REGISTRY_OBJECT_ID,
};

pub fn get_coin_registry_obj_initial_shared_version(
    object_store: &dyn ObjectStore,
) -> HaneulResult<Option<SequenceNumber>> {
    Ok(object_store
        .get_object(&HANEUL_COIN_REGISTRY_OBJECT_ID)
        .map(|obj| match obj.owner {
            Owner::Shared {
                initial_shared_version,
            } => initial_shared_version,
            _ => unreachable!("CoinRegistry object must be shared"),
        }))
}
