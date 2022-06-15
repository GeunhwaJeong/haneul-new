// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module Test::M1 {
    use haneul::id::VersionedID;
    use haneul::tx_context::{Self, TxContext};
    use haneul::transfer;

    struct Object has key, store {
        id: VersionedID,
        value: u64,
    }

    // initializer that should be executed upon publishing this module
    fun init(ctx: &mut TxContext) {
        let singleton = Object { id: tx_context::new_id(ctx), value: 12 };
        transfer::transfer(singleton, tx_context::sender(ctx))
    }
}
