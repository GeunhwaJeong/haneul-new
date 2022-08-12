// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// tests that transfering a child to the same parent does not change the count

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use haneul::tx_context::{Self, TxContext};

    struct S has key, store {
        id: haneul::object::UID,
    }

    public entry fun mint(ctx: &mut TxContext) {
        let parent = haneul::object::new(ctx);
        let child = S { id: haneul::object::new(ctx) };
        haneul::transfer::transfer_to_object_id(child, &mut parent);
        haneul::transfer::transfer(S { id: parent }, tx_context::sender(ctx))
    }

    public entry fun transfer_to_object(child: S, parent: &mut S) {
        haneul::transfer::transfer_to_object(child, parent)
    }
}

//# run test::m::mint --sender A

//# view-object 107

//# view-object 108

//# run test::m::transfer_to_object --sender A --args object(108) object(107)

//# view-object 107

//# view-object 108
