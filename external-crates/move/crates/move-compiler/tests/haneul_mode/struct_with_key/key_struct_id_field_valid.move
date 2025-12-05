// valid
module a::m {
    use haneul::object;

    struct S has key {
        id: object::UID,
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}
