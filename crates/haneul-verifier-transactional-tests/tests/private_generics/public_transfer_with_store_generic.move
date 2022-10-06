// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// tests modules can use transfer functions outside of the defining module, if the type
// has store. This object conditionally has key+store

//# init --addresses a=0x0 t1=0x0 t2=0x0 t3=0x0 t4=0x0 t5=0x0 t6=0x0 t7=0x0 t8=0x0 t9=0x0

//# publish
module a::m {
    struct S<T> has key, store { id: haneul::object::UID, v: T }
}

//# publish
module t1::m {
    fun t(s: a::m::S<u64>) {
        haneul::transfer::transfer(s, @100)
    }
    fun t_gen<T: key + store>(s: T) {
        haneul::transfer::transfer(s, @100)
    }
}

//# publish
module t2::m {
    fun t(
        s: a::m::S<u64>,
        owner_id: &mut haneul::object::UID,
    ) {
        haneul::transfer::transfer_to_object_id(s, owner_id)
    }
    fun t_gen<T: key + store>(
        s: T,
        owner_id: &mut haneul::object::UID,
    ) {
        haneul::transfer::transfer_to_object_id(s, owner_id)
    }
}

//# publish
module t3::m {
    fun t(s: a::m::S<u64>) {
        haneul::transfer::freeze_object(s)
    }
    fun t_gen<T: key + store>(s: T) {
        haneul::transfer::freeze_object(s)
    }
}

//# publish
module t4::m {
    fun t(s: a::m::S<u64>) {
        haneul::transfer::share_object(s)
    }
    fun t_gen<T: key + store>(s: T) {
        haneul::transfer::share_object(s)
    }
}

//# publish
module t5::m {
    struct R has key { id: haneul::object::UID }
    fun t(child: a::m::S<u64>, owner: &mut R) {
        haneul::transfer::transfer_to_object(child, owner)
    }
    fun t_gen<T: key + store>(child: T, owner: &mut R) {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module t6::m {
    struct R has key { id: haneul::object::UID }
    fun t(child: R, owner: &mut a::m::S<u64>) {
        haneul::transfer::transfer_to_object(child, owner)
    }
    fun t_gen<T: key + store>(child: R, owner: &mut T) {
        haneul::transfer::transfer_to_object(child, owner)
    }
}

//# publish
module t7::m {
    fun t(child: a::m::S<u64>, owner: &mut haneul::object::UID) {
        haneul::transfer::transfer_to_object_id(child, owner)
    }
    fun t_gen<T: key + store>(child: T, owner: &mut haneul::object::UID) {
        haneul::transfer::transfer_to_object_id(child, owner)
    }
}
