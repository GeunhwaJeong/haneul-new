// invalid, one-time witness type has no drop ability

//# publish
module a::m {
    use haneul::tx_context;

    struct M { dummy: bool }

    fun init(otw: M, _ctx: &mut tx_context::TxContext) {
        let M { dummy: _ } = otw;
    }
}

module haneul::tx_context {
    struct TxContext has drop {}
}
