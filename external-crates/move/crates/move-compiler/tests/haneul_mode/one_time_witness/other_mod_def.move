// invalid, one-time witness type candidate used in a different module

module a::n {
    use haneul::haneul;
    use haneul::tx_context;

    fun init(_otw: haneul::HANEUL, _ctx: &mut tx_context::TxContext) {}
}

module haneul::tx_context {
    struct TxContext has drop {}
}

module haneul::haneul {
    struct HANEUL has drop {}
}
