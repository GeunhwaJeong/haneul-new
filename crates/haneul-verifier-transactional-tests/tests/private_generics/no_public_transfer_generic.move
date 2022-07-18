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
        owner_info: &haneul::object::Info,
        ctx: &mut haneul::tx_context::TxContext,
    ) {
        haneul::transfer::transfer_to_object_id(s, owner_info)
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
    struct R has key { info: haneul::object::Info }
    fun t<T: key>(child: T, owner: &mut R) {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module test::m {
    struct R has key { info: haneul::object::Info }
    fun t<T: key>(child: R, owner: &mut T) {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module test::m {
    fun t<T: key>(child: T, owner: &haneul::object::Info) {
        haneul::transfer::transfer_to_object_id(child, owner)
    }
}
