// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Coin<Gas> is the token used to pay for gas in Haneul
module Haneul::GAS {
    use Haneul::Coin;
    use Haneul::Transfer;
    use Haneul::TxContext::{Self, TxContext};

    /// Name of the coin
    struct GAS has drop {}

    /// Register the token to acquire its `TreasuryCap`. Because
    /// this is a module initializer, it ensures the currency only gets
    /// registered once.
    // TODO(https://github.com/GeunhwaJeong/haneul/issues/90): implement module initializers
    fun init(ctx: &mut TxContext) {
        // Get a treasury cap for the coin and give it to the transaction sender
        let treasury_cap = Coin::create_currency(GAS{}, ctx);
        Transfer::transfer(treasury_cap, TxContext::sender(ctx))
    }

    /// Transfer to a recipient
    public fun transfer(c: Coin::Coin<GAS>, recipient: address, _ctx: &mut TxContext) {
        Coin::transfer(c, recipient)
    }

}
