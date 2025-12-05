module a::trigger_lint_cases {
    use haneul::object::UID;

    // This should trigger the linter warning (true positive)
    struct MissingKeyAbility {
        id: UID,
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}
