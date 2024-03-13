// valid Random by immutable reference

module a::m {
    public entry fun yes_random_ref(_: &haneul::random::Random) {
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
