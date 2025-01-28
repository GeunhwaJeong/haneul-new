// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<HANEUL> is the token used to pay for gas in Haneul.
/// It has 9 decimals, and the smallest unit (10^-9) is called "geunhwa".
module haneul::haneul {
    use std::option;
    use haneul::tx_context::{Self, TxContext};
    use haneul::balance::{Self, Balance};
    use haneul::transfer;
    use haneul::coin;

    const EAlreadyMinted: u64 = 0;
    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 1;

    #[allow(unused_const)]
    /// The amount of Geunhwa per Haneul token based on the the fact that geunhwa is
    /// 10^-9 of a Haneul token
    const GEUNHWA_PER_HANEUL: u64 = 1_000_000_000;

    #[allow(unused_const)]
    /// The total supply of Haneul denominated in whole Haneul tokens (10 Billion)
    const TOTAL_SUPPLY_HANEUL: u64 = 10_000_000_000;

    /// The total supply of Haneul denominated in Geunhwa (10 Billion * 10^9)
    const TOTAL_SUPPLY_GEUNHWA: u64 = 10_000_000_000_000_000_000;

    /// Name of the coin
    struct HANEUL has drop {}

    #[allow(unused_function)]
    /// Register the `HANEUL` Coin to acquire its `Supply`.
    /// This should be called only once during genesis creation.
    fun new(ctx: &mut TxContext): Balance<HANEUL> {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        assert!(tx_context::epoch(ctx) == 0, EAlreadyMinted);

        let (treasury, metadata) = coin::create_currency(
            HANEUL {},
            9,
            b"HANEUL",
            b"Haneul",
            // TODO: add appropriate description and logo url
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        let supply = coin::treasury_into_supply(treasury);
        let total_haneul = balance::increase_supply(&mut supply, TOTAL_SUPPLY_GEUNHWA);
        balance::destroy_supply(supply);
        total_haneul
    }

    public entry fun transfer(c: coin::Coin<HANEUL>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }
}
