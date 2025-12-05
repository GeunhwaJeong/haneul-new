module a::m {
    use haneul::tx_context;

    public entry fun foo<T>(_: T, _: &mut tx_context::TxContext) {
        abort 0
    }
}

module haneul::tx_context {
    struct TxContext has drop {}
}
