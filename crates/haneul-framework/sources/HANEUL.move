// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<HANEUL> is the token used to pay for gas in Haneul
module Haneul::HANEUL {
    use Haneul::Coin;
    use Haneul::Coin::TreasuryCap;
    use Haneul::TxContext::TxContext;

    friend Haneul::Genesis;

    /// Name of the coin
    struct HANEUL has drop {}

    /// Register the token to acquire its `TreasuryCap`.
    /// This should be called only once during genesis creation.
    public(friend) fun new(ctx: &mut TxContext): TreasuryCap<HANEUL> {
        Coin::create_currency(HANEUL{}, ctx)
    }

    /// Transfer to a recipient
    public(script) fun transfer(c: Coin::Coin<HANEUL>, recipient: address, _ctx: &mut TxContext) {
        Coin::transfer(c, recipient)
    }

}
