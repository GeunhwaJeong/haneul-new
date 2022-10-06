// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests the invalid creation and deletion of a parent object

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use haneul::tx_context::TxContext;

    struct S has key, store {
        id: haneul::object::UID,
    }

    public entry fun t(ctx: &mut TxContext) {
        let parent = haneul::object::new(ctx);
        let child = S { id: haneul::object::new(ctx) };
        haneul::transfer::transfer_to_object_id(child, &mut parent);
        haneul::object::delete(parent);
    }
}

//# run test::m::t --sender A
