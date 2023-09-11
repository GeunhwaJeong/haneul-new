// tests modules can use transfer functions outside of the defining module, if the type
// has store
module a::m {
    use haneul::transfer;
    use a::other;

    public fun t(s: other::S) {
        transfer::public_transfer(s, @0x100)
    }

    public fun f(s: other::S) {
        transfer::public_freeze_object(s)
    }

    public fun s(s: other::S) {
        transfer::public_share_object(s)
    }
}

module a::other {
    struct S has key, store {
        id: haneul::object::UID,
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}

module haneul::transfer {
    public fun transfer<T: key>(_: T, _: address) {
        abort 0
    }

    public fun public_transfer<T: key + store>(_: T, _: address) {
        abort 0
    }

    public fun freeze_object<T: key>(_: T) {
        abort 0
    }

    public fun public_freeze_object<T: key + store>(_: T) {
        abort 0
    }

    public fun share_object<T: key>(_: T) {
        abort 0
    }

    public fun public_share_object<T: key + store>(_: T) {
        abort 0
    }
}
