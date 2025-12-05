// not allowed, a new object is being made with the UID
module a::m {
    use haneul::object::UID;

    struct Foo has key {
        id: UID,
    }

    public fun foo(f: Foo): Foo {
        let Foo { id } = f;
        Foo { id }
    }
}

module haneul::object {
    struct UID has store {
        id: address,
    }
}
