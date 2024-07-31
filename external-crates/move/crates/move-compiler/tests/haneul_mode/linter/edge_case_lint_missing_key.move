module a::edge_cases {
    use haneul::another::UID as AnotherUID;

    // Test case with a different UID type
    struct DifferentUID {
        id: AnotherUID,
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
