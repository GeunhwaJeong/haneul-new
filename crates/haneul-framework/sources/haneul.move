// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<HANEUL> is the token used to pay for gas in Haneul.
/// It has 9 decimals, and the smallest unit (10^-9) is called "geunhwa".
module haneul::haneul {
    use haneul::tx_context::TxContext;
    use haneul::balance::Supply;
    use haneul::transfer;
    use haneul::coin;

    friend haneul::genesis;

    /// Name of the coin
    struct HANEUL has drop {}

    /// Register the `HANEUL` Coin to acquire its `Supply`.
    /// This should be called only once during genesis creation.
    public(friend) fun new(ctx: &mut TxContext): Supply<HANEUL> {
        coin::treasury_into_supply(
            coin::create_currency(HANEUL {}, 9, ctx)
        )
    }

    public entry fun transfer(c: coin::Coin<HANEUL>, recipient: address) {
        transfer::transfer(c, recipient)
    }
}
