// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<HANEUL> is the token used to pay for gas in Haneul
module haneul::haneul {
    use haneul::balance::{Self, Supply};
    use haneul::coin;
    use haneul::transfer;

    friend haneul::genesis;

    /// Name of the coin
    struct HANEUL has drop {}

    /// Register the token to acquire its `TreasuryCap`.
    /// This should be called only once during genesis creation.
    public(friend) fun new(): Supply<HANEUL> {
        balance::create_supply(HANEUL {})
    }

    public entry fun transfer(c: coin::Coin<HANEUL>, recipient: address) {
        transfer::transfer(c, recipient)
    }
}
