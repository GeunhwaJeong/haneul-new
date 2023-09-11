// tests all global storage operations are banned
// tests acquires is banned
module a::m {
    use haneul::object;
    struct R has key { id: object::UID }
    struct G<phantom T> has key { id: object::UID }

    public fun no<T>(s: &signer, addr: address, r: R, g: G<T>) acquires R, G {
        _ = exists<R>(addr);
        _ = exists<G<T>>(addr);
        _ = borrow_global<R>(addr);
        _ = borrow_global<G<T>>(addr);
        _ = borrow_global_mut<R>(addr);
        _ = borrow_global_mut<G<T>>(addr);
        consume<R>(move_from<R>(addr));
        consume<G<T>>(move_from<G<T>>(addr));
        move_to<R>(s, r);
        move_to<G<T>>(s, g);
    }

    fun consume<T>(_: T) {
        abort 0
    }

}

module haneul::object {
    struct UID has store {
        id: address,
    }
}

module haneul::transfer {
}
