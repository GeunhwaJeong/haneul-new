// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module examples::restricted_transfer {
    use haneul::tx_context::{Self, TxContext};
    use haneul::balance::{Self, Balance};
    use haneul::coin::{Self, Coin};
    use haneul::id::VersionedID;
    use haneul::transfer;
    use haneul::haneul::HANEUL;

    /// For when paid amount is not equal to the transfer price.
    const EWrongAmount: u64 = 0;

    /// A Capability that allows bearer to create new `TitleDeed`s.
    struct GovernmentCapability has key { id: VersionedID }

    /// An object that marks a property ownership. Can only be issued
    /// by an authority.
    struct TitleDeed has key {
        id: VersionedID,
        // ... some additional fields
    }

    /// A centralized registry that approves property ownership
    /// transfers and collects fees.
    struct LandRegistry has key {
        id: VersionedID,
        balance: Balance<HANEUL>,
        fee: u64
    }

    /// Create a `LandRegistry` on module init.
    fun init(ctx: &mut TxContext) {
        transfer::transfer(GovernmentCapability {
            id: tx_context::new_id(ctx)
        }, tx_context::sender(ctx));

        transfer::share_object(LandRegistry {
            id: tx_context::new_id(ctx),
            balance: balance::zero<HANEUL>(),
            fee: 10000
        })
    }

    /// Create `TitleDeed` and transfer it to the property owner.
    /// Only owner of the `GovernmentCapability` can perform this action.
    public entry fun issue_title_deed(
        _: &GovernmentCapability,
        for: address,
        ctx: &mut TxContext
    ) {
        transfer::transfer(TitleDeed {
            id: tx_context::new_id(ctx)
        }, for)
    }

    /// A custom transfer function. Required due to `TitleDeed` not having
    /// a `store` ability. All transfers of `TitleDeed`s have to go through
    /// this function and pay a fee to the `LandRegistry`.
    public entry fun transfer_ownership(
        registry: &mut LandRegistry,
        paper: TitleDeed,
        fee: Coin<HANEUL>,
        to: address,
    ) {
        assert!(coin::value(&fee) == registry.fee, EWrongAmount);

        // add a payment to the LandRegistry balance
        balance::join(&mut registry.balance, coin::into_balance(fee));

        // finally call the transfer function
        transfer::transfer(paper, to)
    }
}
