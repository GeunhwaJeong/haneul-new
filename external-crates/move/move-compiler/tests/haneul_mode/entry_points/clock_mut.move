// invalid, Clock by mutable reference

module a::m {
    public entry fun no_clock_mut(_: &mut haneul::clock::Clock) {
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
