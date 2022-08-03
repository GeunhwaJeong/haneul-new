// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// tests modules cannot use transfer functions outside of the defining module

//# init --addresses a=0x0 test=0x0

//# publish
module a::m {
    struct S has key { id: haneul::object::UID }
}

//# publish
module test::m {
    fun t(s: a::m::S) {
        haneul::transfer::transfer(s, @100)
    }
}

//# publish
module test::m {
    fun t(
        s: a::m::S,
        owner_id: &haneul::object::UID,
        ctx: &mut haneul::tx_context::TxContext,
    ) {
        haneul::transfer::transfer_to_object_id(s, owner_id)
    }
}

//# publish
module test::m {
    fun t(s: a::m::S) {
        haneul::transfer::freeze_object(s)
    }
}

//# publish
module test::m {
    fun t(s: a::m::S) {
        haneul::transfer::share_object(s)
    }
}

//# publish
module test::m {
    struct R has key { id: haneul::object::UID }
    fun t(child: a::m::S, owner: &mut R) {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module test::m {
    struct R has key { id: haneul::object::UID }
    fun t(child: R, owner: &mut a::m::S) {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module test::m {
    struct R has key { id: haneul::object::UID }
    fun t(child: a::m::S, owner: &haneul::object::UID) {
        haneul::transfer::transfer_to_object_id(child, owner)
    }
}
