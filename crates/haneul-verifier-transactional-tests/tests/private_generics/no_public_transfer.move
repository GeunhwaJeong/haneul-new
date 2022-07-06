// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// tests modules cannot use transfer functions outside of the defining module

//# init --addresses a=0x0 test=0x0

//# publish
module a::m {
    struct S has key { id: haneul::id::VersionedID }
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
        owner_id: haneul::id::VersionedID,
        ctx: &mut haneul::tx_context::TxContext,
    ): (haneul::id::VersionedID, haneul::transfer::ChildRef<a::m::S>)  {
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
    struct R has key { id: haneul::id::VersionedID }
    fun t(child: a::m::S, owner: &mut R): haneul::transfer::ChildRef<a::m::S> {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module test::m {
    struct R has key { id: haneul::id::VersionedID }
    fun t(child: R, owner: &mut a::m::S): haneul::transfer::ChildRef<R> {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module test::m {
    use haneul::transfer::ChildRef;
    use a::m::S;
    struct R has key { id: haneul::id::VersionedID }
    fun transfer_child_to_object(child: S, c: ChildRef<S>, owner: &mut R): ChildRef<S> {
        haneul::transfer::transfer_child_to_object(child, c, owner)
    }
}

//# publish
module test::m {
    use haneul::transfer::ChildRef;
    use a::m::S;
    struct R has key { id: haneul::id::VersionedID }
    fun transfer_child_to_object(child: R, c: ChildRef<R>, owner: &mut S): ChildRef<R> {
        haneul::transfer::transfer_child_to_object(child, c, owner)
    }
}

//# publish
module test::m {
    use haneul::transfer::ChildRef;
    use a::m::S;
    struct R has key { id: haneul::id::VersionedID }
    fun transfer_child_to_object(s: S, c: ChildRef<S>) {
        haneul::transfer::transfer_child_to_address(s, c, @0x100)
    }
}
