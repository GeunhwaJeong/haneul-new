module a::trigger_lint_cases {
    use haneul::object::UID;

    // 4. Suppress warning
    #[allow(lint(missing_key))]
    struct SuppressWarning {
        id: UID,
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}
