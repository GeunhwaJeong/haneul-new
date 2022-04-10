// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module Tutorial::ColorObject {
    use Haneul::ID::VersionedID;
    use Haneul::Transfer;
    use Haneul::TxContext::{Self, TxContext};

    struct ColorObject has key {
        id: VersionedID,
        red: u8,
        green: u8,
        blue: u8,
    }

    fun new(red: u8, green: u8, blue: u8, ctx: &mut TxContext): ColorObject {
        ColorObject {
            id: TxContext::new_id(ctx),
            red,
            green,
            blue,
        }
    }

    public fun create(red: u8, green: u8, blue: u8, ctx: &mut TxContext) {
        let color_object = new(red, green, blue, ctx);
        Transfer::transfer(color_object, TxContext::sender(ctx))
    }
}
