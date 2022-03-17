module Test::M1 {
    use Haneul::ID::VersionedID;
    use Haneul::TxContext::{Self, TxContext};
    use Haneul::Transfer;

    struct Object has key, store {
        id: VersionedID,
        value: u64,
    }

    // initializer that should be executed upon publishing this module
    fun init(ctx: &mut TxContext) {
        let singleton = Object { id: TxContext::new_id(ctx), value: 12 };
        Transfer::transfer(singleton, TxContext::sender(ctx))
    }
}
