// init is unused but does not error because we are in Haneul mode
module a::m {
    fun init(_: &mut haneul::tx_context::TxContext) {}
}

module haneul::tx_context {
    struct TxContext has drop {}
}
