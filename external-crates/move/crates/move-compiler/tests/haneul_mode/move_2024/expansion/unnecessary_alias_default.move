module haneul::object {
    public struct ID()
    public struct UID()
}
module haneul::transfer {

}
module haneul::tx_context {
    public struct TxContext()
}

module a::m {
    use haneul::object::{Self, ID, UID};
    use haneul::transfer;
    use haneul::tx_context::{Self, TxContext};
}
