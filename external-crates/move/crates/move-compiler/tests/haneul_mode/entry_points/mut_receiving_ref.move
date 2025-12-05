// valid, Receiving type by mut ref with object type param

module a::m {
    use haneul::object;
    use haneul::transfer::Receiving;

    struct S has key { id: object::UID }

    public entry fun yes(_: &mut Receiving<S>) {}
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}

module haneul::transfer {
    struct Receiving<phantom T: key> has drop {
        id: address,
    }
}
