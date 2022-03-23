// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module Test::M1 {
    use Haneul::ID::VersionedID;
    use Haneul::TxContext::{Self, TxContext};
    use Haneul::Transfer;

    struct Object has key, store {
        id: VersionedID,
        value: u64,
    }

    // public initializer - should not be executed
    public fun init(ctx: &mut TxContext) {
        let value = 42;
        let singleton = Object { id: TxContext::new_id(ctx), value };
        Transfer::transfer(singleton, TxContext::sender(ctx))
    }
}
