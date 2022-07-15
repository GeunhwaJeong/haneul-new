// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// tests modules can use transfer functions outside of the defining module, if the type
// has store

//# init --addresses a=0x0 t1=0x0 t2=0x0 t3=0x0 t4=0x0 t5=0x0 t6=0x0 t7=0x0 t8=0x0 t9=0x0

//# publish
module a::m {
    struct S has key, store { id: haneul::id::VersionedID }
}

//# publish
module t1::m {
    fun t(s: a::m::S) {
        haneul::transfer::transfer(s, @100)
    }
}

//# publish
module t2::m {
    fun t(
        s: a::m::S,
        owner_id: &haneul::id::VersionedID,
    ) {
        haneul::transfer::transfer_to_object_id(s, owner_id)
    }
}

//# publish
module t3::m {
    fun t(s: a::m::S) {
        haneul::transfer::freeze_object(s)
    }
}

//# publish
module t4::m {
    fun t(s: a::m::S) {
        haneul::transfer::share_object(s)
    }
}

//# publish
module t5::m {
    struct R has key { id: haneul::id::VersionedID }
    fun t(child: a::m::S, owner: &mut R) {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module t6::m {
    struct R has key { id: haneul::id::VersionedID }
    fun t(child: R, owner: &mut a::m::S) {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module t7::m {
    fun t(child: a::m::S, owner: &haneul::id::VersionedID) {
        haneul::transfer::transfer_to_object_id(child, owner)
    }
}
