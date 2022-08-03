// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// tests TransferObject should fail for a shared object

//# init --accounts A B --addresses test=0x0

//# publish

module test::m {
    use haneul::transfer;
    use haneul::tx_context::TxContext;
    use haneul::object::{Self, UID};

    struct S has key { id: UID }

    public entry fun mint_s(ctx: &mut TxContext) {
        let id = object::new(ctx);
        transfer::share_object(S { id })
    }
}

//# run test::m::mint_s

//# view-object 107

//# transfer-object 107 --sender A --recipient B

//# view-object 107
