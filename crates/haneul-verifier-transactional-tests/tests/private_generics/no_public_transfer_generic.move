// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// tests modules cannot use transfer functions outside of the defining module
// Note: it is not possible to make a generic type `T<...> has key, store`
// where a given instantiation`T<...>` has key but does _not_ have store

//# init --addresses test=0x0

//# publish
module test::m {
    fun t<T: key>(s: T) {
        haneul::transfer::transfer(s, @100)
    }
}

//# publish
module test::m {
    fun t<T: key>(
        s: T,
        owner_id: haneul::id::VersionedID,
        ctx: &mut haneul::tx_context::TxContext,
    ): (haneul::id::VersionedID, haneul::transfer::ChildRef<T>)  {
        haneul::transfer::transfer_to_object_id(s, owner_id)
    }
}

//# publish
module test::m {
    fun t<T: key>(s: T) {
        haneul::transfer::freeze_object(s)
    }
}

//# publish
module test::m {
    fun t<T: key>(s: T) {
        haneul::transfer::share_object(s)
    }
}

//# publish
module test::m {
    struct R has key { id: haneul::id::VersionedID }
    fun t<T: key>(child: T, owner: &mut R): haneul::transfer::ChildRef<T> {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module test::m {
    struct R has key { id: haneul::id::VersionedID }
    fun t<T: key>(child: R, owner: &mut T): haneul::transfer::ChildRef<R> {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module test::m {
    use haneul::transfer::ChildRef;
    struct R has key { id: haneul::id::VersionedID }
    fun transfer_child_to_object<T: key>(child: T, c: ChildRef<T>, owner: &mut R): ChildRef<T> {
        haneul::transfer::transfer_child_to_object(child, c, owner)
    }
}

//# publish
module test::m {
    use haneul::transfer::ChildRef;
    struct R has key { id: haneul::id::VersionedID }
    fun transfer_child_to_object<T: key>(child: R, c: ChildRef<R>, owner: &mut T): ChildRef<R> {
        haneul::transfer::transfer_child_to_object(child, c, owner)
    }
}

//# publish
module test::m {
    use haneul::transfer::ChildRef;
    struct R has key { id: haneul::id::VersionedID }
    fun transfer_child_to_object<T: key>(s: T, c: ChildRef<T>) {
        haneul::transfer::transfer_child_to_address(s, c, @0x100)
    }
}
