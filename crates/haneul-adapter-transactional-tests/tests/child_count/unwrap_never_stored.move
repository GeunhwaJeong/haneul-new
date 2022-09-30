// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests deleting a wrapped object that has never been in storage

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

    public entry fun create(ctx: &mut TxContext) {
        let parent = haneul::object::new(ctx);
        let child = S { id: haneul::object::new(ctx) };
        haneul::transfer::transfer(R { id: parent, s: child }, tx_context::sender(ctx))
    }

    public entry fun delete(r: R) {
        let R { id, s } = r;
        haneul::object::delete(id);
        let S { id } = s;
        haneul::object::delete(id);
    }
}

//
// Test sharing
//

//# run test::m::create --sender A

//# run test::m::delete --args object(107) --sender A
