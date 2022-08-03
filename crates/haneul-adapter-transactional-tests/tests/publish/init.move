// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//# init --addresses Test=0x0

//# publish
module Test::M1 {
    use haneul::object::{Self, UID};
    use haneul::tx_context::{Self, TxContext};
    use haneul::transfer;

    struct Object has key, store {
        id: UID,
        value: u64,
    }

    // initializer that should be executed upon publishing this module
    fun init(ctx: &mut TxContext) {
        let value = 42;
        let singleton = Object { id: object::new(ctx), value };
        transfer::transfer(singleton, tx_context::sender(ctx))
    }
}

//# view-object 104

//# view-object 103
