// invalid, objects need UID not ID
module a::m {
    use haneul::object;

    struct S has key {
        id: object::ID,
    }
}

module haneul::object {
    struct ID has store {
        id: address,
    }
}
