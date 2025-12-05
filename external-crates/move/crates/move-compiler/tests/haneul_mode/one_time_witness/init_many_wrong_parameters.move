module a::m {
    fun init(_: who::X, _: who::Y, _: who::Z) {}
}

module a::beep {
    struct BEEP has drop {}

    fun init(_: Who, _: u64, _: &mut haneul::tx_context::TxContext) {}
}

module haneul::tx_context {
    struct TxContext {}
}
