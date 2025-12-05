// valid init function
module a::m {
    use haneul::tx_context;

    fun init(_: &mut tx_context::TxContext) {}
}

module haneul::tx_context {
    struct TxContext has drop {}
}
