// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests invalid wrapping of a parent object with children

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use haneul::tx_context::{Self, TxContext};
    use haneul::dynamic_object_field as ofield;

    struct S has key, store {
        id: haneul::object::UID,
    }

    struct R has key, store {
        id: haneul::object::UID,
        s: S,
    }

    public entry fun mint(ctx: &mut TxContext) {
        let id = haneul::object::new(ctx);
        haneul::transfer::transfer(S { id }, tx_context::sender(ctx))
    }

    public entry fun add(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child = S { id: haneul::object::new(ctx) };
        ofield::add(&mut parent.id, idx, child);
    }

    public entry fun wrap(s: S, ctx: &mut TxContext) {
        let r = R { id: haneul::object::new(ctx), s };
        haneul::transfer::transfer(r, tx_context::sender(ctx))
    }
}

//# run test::m::mint --sender A

//# run test::m::add --sender A --args object(107) 0

//# view-object 107

//# run test::m::wrap --sender A --args object(107)
