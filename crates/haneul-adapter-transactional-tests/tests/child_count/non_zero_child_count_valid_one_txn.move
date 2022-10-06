// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests valid transfers of an object that has children
// all transfers done in a single transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use haneul::tx_context::{Self, TxContext};

    struct S has key, store {
        id: haneul::object::UID,
    }

    public entry fun mint(ctx: &mut TxContext) {
        let s = S { id: haneul::object::new(ctx) };
        haneul::transfer::transfer(s, tx_context::sender(ctx))
    }

    public entry fun test_transfer_to_object(super_parent: &mut S, ctx: &mut TxContext) {
        let id = haneul::object::new(ctx);
        let child = S { id: haneul::object::new(ctx) };
        haneul::transfer::transfer_to_object_id(child, &mut id);
        let parent = S { id };
        haneul::transfer::transfer_to_object(parent, super_parent)
    }

    public entry fun test_transfer(recipient: address, ctx: &mut TxContext) {
        let id = haneul::object::new(ctx);
        let child = S { id: haneul::object::new(ctx) };
        haneul::transfer::transfer_to_object_id(child, &mut id);
        let parent = S { id };
        haneul::transfer::transfer(parent, recipient)
    }

    public entry fun test_share(ctx: &mut TxContext) {
        let id = haneul::object::new(ctx);
        let child = S { id: haneul::object::new(ctx) };
        haneul::transfer::transfer_to_object_id(child, &mut id);
        let parent = S { id };
        haneul::transfer::share_object(parent)
    }
}

//
// Test transfer_to_object allows non-zero child count
//

//# run test::m::mint --sender A

//# run test::m::test_transfer_to_object --sender A --args object(107)


//
// Test share object allows non-zero child count
//

//# run test::m::test_share --sender A

//
// Test transfer allows non-zero child count
//

//# run test::m::test_transfer --sender A --args @B
