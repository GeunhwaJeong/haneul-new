// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// tests modules can use child ref functions, even with a type that does not have store

//# init --addresses a=0x0 t1=0x0 t2=0x0

//# publish
module a::m {
    struct S has key { id: haneul::id::VersionedID }
}

//# publish
module t1::m {
    use a::m::S;
    use haneul::transfer::ChildRef;
    fun t(c: &ChildRef<S>, child: &S): bool {
        haneul::transfer::is_child(c, child)
    }
    fun t_gen<T: key>(c: &ChildRef<T>, child: &T): bool {
        haneul::transfer::is_child(c, child)
    }
}

//# publish
module t2::m {
    use a::m::S;
    use haneul::transfer::ChildRef;
    fun t(id: haneul::id::VersionedID, c: ChildRef<S>) {
        haneul::transfer::delete_child_object(id, c)
    }
    fun t_gen<T: key>(id: haneul::id::VersionedID, c: ChildRef<T>) {
        haneul::transfer::delete_child_object(id, c)
    }
}
