// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//# init --addresses Test=0x0 --accounts A

//# publish
module Test::M {
    use Haneul::TxContext::{Self, TxContext};
    struct Obj has key {
        id: Haneul::ID::VersionedID,
        value: u64
    }

    public(script) fun mint(ctx: &mut TxContext) {
        Haneul::Transfer::transfer(
            Obj { id: TxContext::new_id(ctx), value: 0 },
            TxContext::sender(ctx),
        )
    }

    public(script) fun incr(obj: &mut Obj) {
        obj.value = obj.value + 1
    }
}

//# run Test::M::mint --sender A

//# run Test::M::incr --sender A --args object(106)
