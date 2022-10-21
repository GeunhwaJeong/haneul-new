// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests valid transfers of an object that has children

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
        let child = S { id: haneul::object::new(ctx) };
        ofield::add(&mut id, 0, child);
        haneul::transfer::transfer(S { id }, tx_context::sender(ctx))
    }

    public entry fun share(s: S) {
        haneul::transfer::share_object(s)
    }

    public entry fun transfer(s: S, recipient: address) {
        haneul::transfer::transfer(s, recipient)
    }

}

//
// Test share object allows non-zero child count
//

//# run test::m::mint --sender A

//# run test::m::share --sender A --args object(108)

//# view-object 108

//
// Test transfer allows non-zero child count
//

//# run test::m::mint --sender A

//# run test::m::transfer --sender A --args object(113) @B

//# view-object 113

//
// Test TransferObject allows non-zero child count
//

//# run test::m::mint --sender A

//# transfer-object 119 --sender A --recipient B

//# view-object 119
