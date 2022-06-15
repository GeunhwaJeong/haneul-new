// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<HANEUL> is the token used to pay for gas in Haneul
module haneul::haneul {
    use haneul::coin;
    use haneul::coin::TreasuryCap;
    use haneul::tx_context::TxContext;

    friend haneul::genesis;

    /// Name of the coin
    struct HANEUL has drop {}

    /// Register the token to acquire its `TreasuryCap`.
    /// This should be called only once during genesis creation.
    public(friend) fun new(ctx: &mut TxContext): TreasuryCap<HANEUL> {
        coin::create_currency(HANEUL{}, ctx)
    }

    /// Transfer to a recipient
    public entry fun transfer(c: coin::Coin<HANEUL>, recipient: address) {
        coin::transfer(c, recipient)
    }

}
