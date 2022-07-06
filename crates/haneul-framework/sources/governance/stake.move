// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module haneul::stake {
    use std::option::{Self, Option};
    use haneul::balance::Balance;
    use haneul::id::{Self, VersionedID};
    use haneul::locked_coin;
    use haneul::haneul::HANEUL;
    use haneul::transfer;
    use haneul::tx_context::{Self, TxContext};
    use haneul::epoch_time_lock::EpochTimeLock;
    use haneul::epoch_time_lock;
    use haneul::balance;
    use haneul::math;

    friend haneul::haneul_system;
    friend haneul::validator;

    /// A custodial stake object holding the staked HANEUL coin.
    struct Stake has key {
        id: VersionedID,
        /// The staked HANEUL tokens.
        balance: Balance<HANEUL>,
        /// The epoch until which the staked coin is locked. If the stake
        /// comes from a Coin<HANEUL>, this field is None. If it comes from a LockedCoin<HANEUL>, this
        /// field will record the original lock expiration epoch, to be used when unstaking.
        locked_until_epoch: Option<EpochTimeLock>,
    }

    /// The number of epochs the withdrawn stake is locked for.
    /// TODO: this is a placehodler number and may be changed.
    const BONDING_PERIOD: u64 = 1;

    /// Error number for when a Stake with nonzero balance is burnt. 
    const ENONZERO_BALANCE: u64 = 0;

    /// Create a stake object from a HANEUL balance. If the balance comes from a
    /// `LockedCoin`, an EpochTimeLock is passed in to keep track of locking period.
    public(friend) fun create(
        balance: Balance<HANEUL>,
        recipient: address,
        locked_until_epoch: Option<EpochTimeLock>,
        ctx: &mut TxContext,
    ) {
        let stake = Stake {
            id: tx_context::new_id(ctx),
            balance,
            locked_until_epoch,
        };
        transfer::transfer(stake, recipient)
    }

    /// Withdraw `amount` from the balance of `stake`.
    public(friend) fun withdraw_stake(
        self: &mut Stake,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        let sender = tx_context::sender(ctx);
        let unlock_epoch = tx_context::epoch(ctx) + BONDING_PERIOD;
        let balance = balance::split(&mut self.balance, amount);

        if (option::is_none(&self.locked_until_epoch)) {
            // If the stake didn't come from a locked coin, we give back the stake and
            // lock the coin for `BONDING_PERIOD`.
            locked_coin::new_from_balance(balance, epoch_time_lock::new(unlock_epoch, ctx), sender, ctx);
        } else {
            // If the stake did come from a locked coin, we lock the coin for
            // max(BONDING_PERIOD, remaining_lock_time).
            let original_unlock_epoch = epoch_time_lock::epoch(option::borrow(&self.locked_until_epoch));
            let unlock_epoch = math::max(original_unlock_epoch, unlock_epoch);
            locked_coin::new_from_balance(balance, epoch_time_lock::new(unlock_epoch, ctx), sender, ctx);
        };
    }

    /// Burn the stake object. This can be done only when the stake has a zero balance.
    public entry fun burn(self: Stake, ctx: &mut TxContext) {
        let Stake { id, balance, locked_until_epoch } = self;
        id::delete(id);
        balance::destroy_zero(balance);
        if (option::is_some(&locked_until_epoch)) {
            epoch_time_lock::destroy(option::extract(&mut locked_until_epoch), ctx);
        };
        option::destroy_none(locked_until_epoch);
    }

    public fun value(self: &Stake): u64 {
        balance::value(&self.balance)
    }
}
