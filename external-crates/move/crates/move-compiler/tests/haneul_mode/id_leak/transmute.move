// not allowed, it re-uses an ID in a new object
module a::m {
    use haneul::object::UID;
    use haneul::transfer::transfer;
    use haneul::tx_context::{Self, TxContext};

    struct Cat has key {
        id: UID,
    }

    struct Dog has key {
        id: UID,
    }

    public fun transmute(cat: Cat, ctx: &mut TxContext) {
        let Cat { id } = cat;
        let dog = Dog { id };
        transfer(dog, tx_context::sender(ctx));
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}

module haneul::tx_context {
    struct TxContext has drop {}

    public fun sender(_: &TxContext): address {
        @0
    }
}

module haneul::transfer {
    public fun transfer<T: key>(_: T, _: address) {
        abort 0
    }
}
