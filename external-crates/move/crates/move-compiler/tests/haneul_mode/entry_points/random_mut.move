// invalid Random by mutable reference

module a::m {
    public entry fun no_random_mut(_: &mut haneul::random::Random) {
        abort 0
    }
}

module haneul::random {
    struct Random has key {
        id: haneul::object::UID,
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}
