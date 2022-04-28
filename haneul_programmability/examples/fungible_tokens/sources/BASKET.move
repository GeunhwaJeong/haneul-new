// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// A synthetic fungible token backed by a basket of other tokens.
/// Here, we use a basket that is 1:1 HANEUL and MANAGED,
/// but this approach would work for a basket with arbitrary assets/ratios.
/// E.g., [SDR](https://www.imf.org/en/About/Factsheets/Sheets/2016/08/01/14/51/Special-Drawing-Right-SDR)
/// could be implemented this way.
module FungibleTokens::BASKET {
    use FungibleTokens::MANAGED::MANAGED;
    use Haneul::Coin::{Self, Coin, TreasuryCap};
    use Haneul::ID::VersionedID;
    use Haneul::HANEUL::HANEUL;
    use Haneul::Transfer;
    use Haneul::TxContext::{Self, TxContext};

    /// Name of the coin. By convention, this type has the same name as its parent module
    /// and has no fields. The full type of the coin defined by this module will be `COIN<BASKET>`.
    struct BASKET has drop { }

    /// Singleton shared object holding the reserve assets and the capability.
    struct Reserve has key {
        id: VersionedID,
        /// capability allowing the reserve to mint and burn BASKET
        treasury_cap: TreasuryCap<BASKET>,
        /// HANEUL coins held in the reserve
        haneul: Coin<HANEUL>,
        /// MANAGED coins held in the reserve
        managed: Coin<MANAGED>,
    }

    /// Needed to deposit a 1:1 ratio of HANEUL and MANAGED for minting, but deposited a different ratio
    const EBadDepositRatio: u64 = 0;

    fun init(ctx: &mut TxContext) {
        // Get a treasury cap for the coin put it in the reserve
        let treasury_cap = Coin::create_currency<BASKET>(BASKET{}, ctx);
        Transfer::share_object(Reserve {
            id: TxContext::new_id(ctx),
            treasury_cap,
            haneul: Coin::zero<HANEUL>(ctx),
            managed: Coin::zero<MANAGED>(ctx),
        })
    }

    /// === Writes ===

    /// Mint BASKET coins by accepting an equal number of HANEUL and MANAGED coins
    public fun mint(
        reserve: &mut Reserve, haneul: Coin<HANEUL>, managed: Coin<MANAGED>, ctx: &mut TxContext
    ): Coin<BASKET> {
        let num_haneul = Coin::value(&haneul);
        assert!(num_haneul == Coin::value(&managed), EBadDepositRatio);

        Coin::join(&mut reserve.haneul, haneul);
        Coin::join(&mut reserve.managed, managed);
        Coin::mint(num_haneul, &mut reserve.treasury_cap, ctx)
    }

    /// Burn BASKET coins and return the underlying reserve assets
    public fun burn(
        reserve: &mut Reserve, basket: Coin<BASKET>, ctx: &mut TxContext
    ): (Coin<HANEUL>, Coin<MANAGED>) {
        let num_basket = Coin::value(&basket);
        Coin::burn(basket, &mut reserve.treasury_cap);
        let haneul = Coin::withdraw(&mut reserve.haneul, num_basket, ctx);
        let managed = Coin::withdraw(&mut reserve.managed, num_basket, ctx);
        (haneul, managed)
    }

    // === Reads ===

    /// Return the number of `MANAGED` coins in circulation
    public fun total_supply(reserve: &Reserve): u64 {
        Coin::total_supply(&reserve.treasury_cap)
    }

    /// Return the number of HANEUL in the reserve
    public fun haneul_supply(reserve: &Reserve): u64 {
        Coin::value(&reserve.haneul)
    }

    /// Return the number of MANAGED in the reserve
    public fun managed_supply(reserve: &Reserve): u64 {
        Coin::value(&reserve.managed)
    }

    #[test_only]
    public fun init_for_testing(ctx: &mut TxContext) {
        init(ctx)
    }
}
