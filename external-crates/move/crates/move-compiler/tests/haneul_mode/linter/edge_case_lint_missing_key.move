module a::edge_cases {
    struct UID {}
    // Test case with a different UID type
    struct DifferentUID {
        id: haneul::another::UID,
    }

    struct NotAnObject {
        id: UID,
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}

module haneul::another {
    struct UID has store {
        id: address,
    }
}
