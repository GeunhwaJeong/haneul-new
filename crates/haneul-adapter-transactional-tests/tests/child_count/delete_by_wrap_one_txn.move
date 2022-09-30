// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests invalid wrapping of a parent object with children, in a single transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use haneul::tx_context::{Self, TxContext};

    struct S has key, store {
        id: haneul::object::UID,
    }

    struct R has key {
        id: haneul::object::UID,
        s: S,
    }

    public entry fun test_wrap(ctx: &mut TxContext) {
        let id = haneul::object::new(ctx);
        let child = S { id: haneul::object::new(ctx) };
        haneul::transfer::transfer_to_object_id(child, &mut id);
        let parent = S { id };
        let r = R { id: haneul::object::new(ctx), s: parent };
        haneul::transfer::transfer(r, tx_context::sender(ctx))
    }
}

//# run test::m::test_wrap --sender A
