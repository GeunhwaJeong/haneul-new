// valid, Clock by immutable reference

module a::m {
    public entry fun yes_clock_ref(_: &haneul::clock::Clock) {
        abort 0
    }
}

module haneul::clock {
    struct Clock has key {
        id: haneul::object::UID,
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}
