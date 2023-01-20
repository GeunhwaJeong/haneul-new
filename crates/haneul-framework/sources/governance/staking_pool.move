// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module haneul::staking_pool {
    use haneul::balance::{Self, Balance, Supply};
    use haneul::haneul::HANEUL;
    use std::option::{Self, Option};
    use haneul::tx_context::{Self, TxContext};
    use haneul::transfer;
    use haneul::epoch_time_lock::{EpochTimeLock};
    use haneul::object::{Self, ID, UID};
    use haneul::locked_coin;
    use haneul::coin;
    use std::vector;
    use haneul::table_vec::{Self, TableVec};
    use haneul::linked_table::{Self, LinkedTable};

    friend haneul::validator;
    friend haneul::validator_set;
    
    const EINSUFFICIENT_POOL_TOKEN_BALANCE: u64 = 0;
    const EWRONG_POOL: u64 = 1;
    const EWITHDRAW_AMOUNT_CANNOT_BE_ZERO: u64 = 2;
    const EINSUFFICIENT_HANEUL_TOKEN_BALANCE: u64 = 3;
    const EINSUFFICIENT_REWARDS_POOL_BALANCE: u64 = 4;
    const EDESTROY_NON_ZERO_BALANCE: u64 = 5;
    const ETOKEN_TIME_LOCK_IS_SOME: u64 = 6;
    const EWRONG_DELEGATION: u64 = 7;
    const EPENDING_DELEGATION_DOES_NOT_EXIST: u64 = 8;

    /// A staking pool embedded in each validator struct in the system state object.
    struct StakingPool has store {
        /// The haneul address of the validator associated with this pool.
        validator_address: address,
        /// The epoch at which this pool started operating. Should be the epoch at which the validator became active.
        starting_epoch: u64,
        /// The total number of HANEUL tokens in this pool, including the HANEUL in the rewards_pool, as well as in all the principal
        /// in the `Delegation` object, updated at epoch boundaries.
        haneul_balance: u64,
        /// The epoch delegation rewards will be added here at the end of each epoch. 
        rewards_pool: Balance<HANEUL>,
        /// The number of delegation pool tokens we have issued so far. This number should equal the sum of
        /// pool token balance in all the `Delegation` objects delegated to this pool. Updated at epoch boundaries.
        delegation_token_supply: Supply<DelegationToken>,
        /// Delegations requested during the current epoch. We will activate these delegation at the end of current epoch
        /// and distribute staking pool tokens at the end-of-epoch exchange rate after the rewards for the current epoch
        /// have been deposited.
        pending_delegations: LinkedTable<ID, PendingDelegationEntry>,
        /// Delegation withdraws requested during the current epoch. Similar to new delegation, the withdraws are processed
        /// at epoch boundaries. Rewards are withdrawn and distributed after the rewards for the current epoch have come in. 
        pending_withdraws: TableVec<PendingWithdrawEntry>,
    }

    /// Struct representing the exchange rate of the delegation pool token to HANEUL.
    struct PoolTokenExchangeRate has copy, drop {
        haneul_amount: u64,
        pool_token_amount: u64,
    }

    /// An inactive staking pool associated with an inactive validator.
    /// Only withdraws can be made from this pool.
    struct InactiveStakingPool has key {
        id: UID, // TODO: inherit an ID from active staking pool?
        pool: StakingPool,
    }

    /// The staking pool token.
    struct DelegationToken has drop {}

    /// Struct representing a pending delegation.
    struct PendingDelegationEntry has store, drop {
        delegator: address, 
        haneul_amount: u64,
    }

    /// Struct representing a pending delegation withdraw.
    struct PendingWithdrawEntry has store {
        delegator: address, 
        principal_withdraw_amount: u64,
        withdrawn_pool_tokens: Balance<DelegationToken>,
    }

    /// A self-custodial delegation object, serving as evidence that the delegator
    /// has delegated to a staking pool.
    struct Delegation has key {
        id: UID,
        /// The ID of the corresponding `StakedHaneul` object.
        staked_haneul_id: ID,
        /// The pool tokens representing the amount of rewards the delegator can get back when they withdraw
        /// from the pool.
        pool_tokens: Balance<DelegationToken>,
        /// Number of HANEUL token staked originally.
        principal_haneul_amount: u64,
    }

    /// A self-custodial object holding the staked HANEUL tokens.
    struct StakedHaneul has key {
        id: UID,
        /// The validator we are staking with.
        validator_address: address,
        /// The epoch at which the staking pool started operating.
        pool_starting_epoch: u64,
        /// The epoch at which the delegation is requested.
        delegation_request_epoch: u64,
        /// The staked HANEUL tokens.
        principal: Balance<HANEUL>,
        /// If the stake comes from a Coin<HANEUL>, this field is None. If it comes from a LockedCoin<HANEUL>, this
        /// field will record the original lock expiration epoch, to be used when unstaking.
        haneul_token_lock: Option<EpochTimeLock>,
    }

    // ==== initializer ====

    /// Create a new, empty staking pool.
    public(friend) fun new(validator_address: address, starting_epoch: u64, ctx: &mut TxContext) : StakingPool {
        StakingPool {
            validator_address,
            starting_epoch,
            haneul_balance: 0,
            rewards_pool: balance::zero(),
            delegation_token_supply: balance::create_supply(DelegationToken {}),
            pending_delegations: linked_table::new(ctx),
            pending_withdraws: table_vec::empty(ctx),
        }
    }


    // ==== delegation requests ====

    // TODO: implement rate limiting new delegations per epoch.
    /// Request to delegate to a staking pool. The delegation gets counted at the beginning of the next epoch,
    /// when the delegation object containing the pool tokens is distributed to the delegator.
    public(friend) fun request_add_delegation(
        pool: &mut StakingPool, 
        stake: Balance<HANEUL>, 
        haneul_token_lock: Option<EpochTimeLock>,
        delegator: address,
        ctx: &mut TxContext
    ) {
        let haneul_amount = balance::value(&stake);
        assert!(haneul_amount > 0, 0);
        let staked_haneul = StakedHaneul {
            id: object::new(ctx),
            validator_address: pool.validator_address,
            pool_starting_epoch: pool.starting_epoch,
            delegation_request_epoch: tx_context::epoch(ctx),
            principal: stake,
            haneul_token_lock,
        };
        // insert delegation info into the pending_delegations table.
        linked_table::push_back(
            &mut pool.pending_delegations,
            object::id(&staked_haneul),
            PendingDelegationEntry { delegator, haneul_amount }
        );
        transfer::transfer(staked_haneul, delegator);
    }

    /// Request to withdraw `principal_withdraw_amount` of stake plus rewards from a staking pool.
    /// This amount of principal in HANEUL is withdrawn and transferred to the delegator. A proportional amount
    /// of pool tokens will be later burnt.
    /// The rewards portion will be withdrawn at the end of the epoch, after the rewards have come in so we
    /// can use the new exchange rate to calculate the rewards.
    public(friend) fun request_withdraw_delegation(
        pool: &mut StakingPool,  
        delegation: Delegation, 
        staked_haneul: StakedHaneul,
        ctx: &mut TxContext
    ) : u64 {
        let (withdrawn_pool_tokens, principal_withdraw, time_lock) = 
            withdraw_from_principal(pool, delegation, staked_haneul);
        
        let delegator = tx_context::sender(ctx);
        let principal_withdraw_amount = balance::value(&principal_withdraw);
        table_vec::push_back(&mut pool.pending_withdraws, PendingWithdrawEntry {
            delegator, principal_withdraw_amount, withdrawn_pool_tokens });

        // TODO: implement withdraw bonding period here.
        if (option::is_some(&time_lock)) {
            locked_coin::new_from_balance(principal_withdraw, option::destroy_some(time_lock), delegator, ctx);
        } else {
            transfer::transfer(coin::from_balance(principal_withdraw, ctx), delegator);
            option::destroy_none(time_lock);
        };
        principal_withdraw_amount
    }

    public(friend) fun cancel_delegation_request(pool: &mut StakingPool, staked_haneul: StakedHaneul, ctx: &mut TxContext) {
        let delegator = tx_context::sender(ctx);
        let staked_haneul_id = object::id(&staked_haneul);
        assert!(linked_table::contains(&mut pool.pending_delegations, staked_haneul_id), EPENDING_DELEGATION_DOES_NOT_EXIST);

        linked_table::remove(&mut pool.pending_delegations, staked_haneul_id);

        let StakedHaneul { 
            id,
            validator_address,
            pool_starting_epoch,
            delegation_request_epoch: _,
            principal,
            haneul_token_lock
        } = staked_haneul;

        // sanity check that the StakedHaneul is indeed from this pool. Should never fail.
        assert!(
            validator_address == pool.validator_address &&
            pool_starting_epoch == pool.starting_epoch,
            EWRONG_POOL
        );
        object::delete(id);
        if (option::is_some(&haneul_token_lock)) {
            locked_coin::new_from_balance(principal, option::destroy_some(haneul_token_lock), delegator, ctx);
        } else {
            transfer::transfer(coin::from_balance(principal, ctx), delegator);
            option::destroy_none(haneul_token_lock);
        };
    }

    /// Withdraw the requested amount of the principal HANEUL stored in the StakedHaneul object, as
    /// well as a proportional amount of pool tokens from the delegation object.
    /// For example, suppose the delegation object contains 15 pool tokens and the principal HANEUL 
    /// amount is 21. Then if `principal_withdraw_amount` is 7, 5 pool tokens and 7 HANEUL tokens will
    /// be withdrawn.
    /// Returns values are withdrawn pool tokens, withdrawn principal portion of HANEUL, and its 
    /// time lock if applicable.
    public(friend) fun withdraw_from_principal(
        pool: &mut StakingPool,  
        delegation: Delegation, 
        staked_haneul: StakedHaneul,
    ) : (Balance<DelegationToken>, Balance<HANEUL>, Option<EpochTimeLock>) {
        // Check that the delegation and staked haneul objects match.
        assert!(object::id(&staked_haneul) == delegation.staked_haneul_id, EWRONG_DELEGATION);

        // Check that the delegation information matches the pool. 
        assert!(
            staked_haneul.validator_address == pool.validator_address &&
            staked_haneul.pool_starting_epoch == pool.starting_epoch,
            EWRONG_POOL
        );

        assert!(delegation.principal_haneul_amount == balance::value(&staked_haneul.principal), EINSUFFICIENT_HANEUL_TOKEN_BALANCE);

        let pool_tokens = destroy_delegation_and_return_pool_tokens(delegation);
        let (principal_withdraw, time_lock) = unwrap_staked_haneul(staked_haneul);

        (
            pool_tokens,
            principal_withdraw,
            time_lock
        )
    }

    fun destroy_delegation_and_return_pool_tokens(delegation: Delegation): Balance<DelegationToken> {
        let Delegation { id, staked_haneul_id: _, pool_tokens, principal_haneul_amount: _ } = delegation;
        object::delete(id);
        pool_tokens
    }

    fun unwrap_staked_haneul(staked_haneul: StakedHaneul): (Balance<HANEUL>, Option<EpochTimeLock>) {
        let StakedHaneul { 
            id,
            validator_address: _,
            pool_starting_epoch: _,
            delegation_request_epoch: _,
            principal,
            haneul_token_lock
        } = staked_haneul;
        object::delete(id);
        (principal, haneul_token_lock)
    }

    // ==== functions called at epoch boundaries ===

    /// Called at epoch advancement times to add rewards (in HANEUL) to the staking pool. 
    public(friend) fun deposit_rewards(pool: &mut StakingPool, rewards: Balance<HANEUL>) {
        pool.haneul_balance = pool.haneul_balance + balance::value(&rewards);
        balance::join(&mut pool.rewards_pool, rewards);
    }

    /// Called at epoch boundaries to process pending delegation withdraws requested during the epoch.
    /// For each pending withdraw entry, we withdraw the rewards from the pool at the new exchange rate and burn the pool
    /// tokens.
    public(friend) fun process_pending_delegation_withdraws(pool: &mut StakingPool, ctx: &mut TxContext) : u64 {
        let total_reward_withdraw = 0;

        while (!table_vec::is_empty(&pool.pending_withdraws)) {
            let PendingWithdrawEntry {
                delegator, principal_withdraw_amount, withdrawn_pool_tokens
            } = table_vec::pop_back(&mut pool.pending_withdraws);
            let reward_withdraw = withdraw_rewards_and_burn_pool_tokens(pool, principal_withdraw_amount, withdrawn_pool_tokens);
            total_reward_withdraw = total_reward_withdraw + balance::value(&reward_withdraw);
            transfer::transfer(coin::from_balance(reward_withdraw, ctx), delegator);
        };
        total_reward_withdraw
    }

    /// Called at epoch boundaries to mint new pool tokens to new delegators at the new exchange rate.
    /// New delegators include both entirely new delegations and delegations switched to this staking pool
    /// during the previous epoch.
    public(friend) fun process_pending_delegations(pool: &mut StakingPool, ctx: &mut TxContext) {
        while (!linked_table::is_empty(&pool.pending_delegations)) {
            let (staked_haneul_id, PendingDelegationEntry { delegator, haneul_amount }) =
                linked_table::pop_back(&mut pool.pending_delegations);
            mint_delegation_tokens_to_delegator(pool, delegator, haneul_amount, staked_haneul_id, ctx);
            pool.haneul_balance = pool.haneul_balance + haneul_amount;
        };
    }

    /// Called by validator_set at epoch boundaries for delegation switches.
    /// This function goes through the provided vector of pending withdraw entries, 
    /// and for each entry, calls `withdraw_rewards_and_burn_pool_tokens` to withdraw
    /// the rewards portion of the delegation and burn the pool tokens. We then aggregate
    /// the delegator addresses and their rewards into vectors, as well as calculate 
    /// the total amount of rewards HANEUL withdrawn. These three return values are then
    /// used in `validator_set`'s delegation switching code to deposit the rewards part
    /// into the new validator's staking pool.
    public(friend) fun batch_withdraw_rewards_and_burn_pool_tokens(
        pool: &mut StakingPool,
        entries: TableVec<PendingWithdrawEntry>,
    ) : (vector<address>, vector<Balance<HANEUL>>, u64) {
        let (delegators, rewards, total_rewards_withdraw_amount) = (vector::empty(), vector::empty(), 0);
        while (!table_vec::is_empty(&mut entries)) {
            let PendingWithdrawEntry { delegator, principal_withdraw_amount, withdrawn_pool_tokens } 
                = table_vec::pop_back(&mut entries);
            let reward = withdraw_rewards_and_burn_pool_tokens(pool, principal_withdraw_amount, withdrawn_pool_tokens);
            total_rewards_withdraw_amount = total_rewards_withdraw_amount + balance::value(&reward);
            vector::push_back(&mut delegators, delegator);
            vector::push_back(&mut rewards, reward);
        };
        table_vec::destroy_empty(entries);
        (delegators, rewards, total_rewards_withdraw_amount)
    }

    /// This function does the following:
    ///     1. Calculates the total amount of HANEUL (including principal and rewards) that the provided pool tokens represent
    ///        at the current exchange rate.
    ///     2. Using the above number and the given `principal_withdraw_amount`, calculates the rewards portion of the 
    ///        delegation we should withdraw.
    ///     3. Withdraws the rewards portion from the rewards pool at the current exchange rate. We only withdraw the rewards
    ///        portion because the principal portion was already taken out of the delegator's self custodied StakedHaneul at request 
    ///        time in `request_withdraw_stake`.
    ///     4. Since HANEUL tokens are withdrawn, we need to burn the corresponding pool tokens to keep the exchange rate the same.
    ///     5. Updates the HANEUL balance amount of the pool.
    fun withdraw_rewards_and_burn_pool_tokens(
        pool: &mut StakingPool, 
        principal_withdraw_amount: u64, 
        withdrawn_pool_tokens: Balance<DelegationToken>,
    ) : Balance<HANEUL> {
        let pool_token_amount = balance::value(&withdrawn_pool_tokens);
        let total_haneul_withdraw_amount = get_haneul_amount(pool, pool_token_amount);
        let reward_withdraw_amount =
            if (total_haneul_withdraw_amount >= principal_withdraw_amount)
                total_haneul_withdraw_amount - principal_withdraw_amount
            else 0;
        balance::decrease_supply(
            &mut pool.delegation_token_supply, 
            withdrawn_pool_tokens
        );
        pool.haneul_balance = pool.haneul_balance - (principal_withdraw_amount + reward_withdraw_amount);
        balance::split(&mut pool.rewards_pool, reward_withdraw_amount)
    }

    /// Given the `haneul_amount`, mint the corresponding amount of pool tokens at the current exchange
    /// rate, puts the pool tokens in a delegation object, and gives the delegation object to the delegator.
    fun mint_delegation_tokens_to_delegator(
        pool: &mut StakingPool, 
        delegator: address, 
        haneul_amount: u64, 
        staked_haneul_id: ID,
        ctx: &mut TxContext
    ) {
        let new_pool_token_amount = get_token_amount(pool, haneul_amount);   

        // Mint new pool tokens at the current exchange rate.
        let pool_tokens = balance::increase_supply(&mut pool.delegation_token_supply, new_pool_token_amount);

        let delegation = Delegation {
            id: object::new(ctx),
            staked_haneul_id,
            pool_tokens,
            principal_haneul_amount: haneul_amount,
        };

        transfer::transfer(delegation, delegator);
    }


    // ==== inactive pool related ====

    /// Deactivate a staking pool by wrapping it in an `InactiveStakingPool` and sharing this newly created object. 
    /// After this pool deactivation, the pool stops earning rewards. Only delegation withdraws can be made to the pool.
    public(friend) fun deactivate_staking_pool(pool: StakingPool, ctx: &mut TxContext) {
        let inactive_pool = InactiveStakingPool { id: object::new(ctx), pool};
        transfer::share_object(inactive_pool);
    }

    /// Withdraw delegation from an inactive pool. Since no epoch rewards will be added to an inactive pool,
    /// the exchange rate between pool tokens and HANEUL tokens stay the same. Therefore, unlike withdrawing
    /// from an active pool, we can handle both principal and rewards withdraws directly here.
    public entry fun withdraw_from_inactive_pool(
        inactive_pool: &mut InactiveStakingPool, 
        staked_haneul: StakedHaneul, 
        delegation: Delegation, 
        ctx: &mut TxContext
    ) {
        let pool = &mut inactive_pool.pool;
        let (withdrawn_pool_tokens, principal_withdraw, time_lock) = 
            withdraw_from_principal(pool, delegation, staked_haneul);
        let principal_withdraw_amount = balance::value(&principal_withdraw);
        let rewards_withdraw = withdraw_rewards_and_burn_pool_tokens(pool, principal_withdraw_amount, withdrawn_pool_tokens);
        let total_withdraw_amount = principal_withdraw_amount + balance::value(&rewards_withdraw);
        pool.haneul_balance = pool.haneul_balance - total_withdraw_amount;

        let delegator = tx_context::sender(ctx);
        // TODO: implement withdraw bonding period here.
        if (option::is_some(&time_lock)) {
            locked_coin::new_from_balance(principal_withdraw, option::destroy_some(time_lock), delegator, ctx);
            transfer::transfer(coin::from_balance(rewards_withdraw, ctx), delegator);
        } else {
            balance::join(&mut principal_withdraw, rewards_withdraw);
            transfer::transfer(coin::from_balance(principal_withdraw, ctx), delegator);
            option::destroy_none(time_lock);
        };
    }


    // ==== destroyers ====

    /// Destroy an empty delegation that no longer contains any HANEUL or pool tokens.
    public entry fun destroy_empty_delegation(delegation: Delegation) {
        let Delegation {
            id,
            staked_haneul_id: _,
            pool_tokens,
            principal_haneul_amount,
        } = delegation;
        object::delete(id);
        assert!(balance::value(&pool_tokens) == 0, EDESTROY_NON_ZERO_BALANCE);
        assert!(principal_haneul_amount == 0, EDESTROY_NON_ZERO_BALANCE);
        balance::destroy_zero(pool_tokens);
    }

    /// Destroy an empty delegation that no longer contains any HANEUL or pool tokens.
    public entry fun destroy_empty_staked_haneul(staked_haneul: StakedHaneul) {
        let StakedHaneul {
            id,
            validator_address: _,
            pool_starting_epoch: _,
            delegation_request_epoch: _,
            principal,
            haneul_token_lock
        } = staked_haneul;
        object::delete(id);
        assert!(balance::value(&principal) == 0, EDESTROY_NON_ZERO_BALANCE);
        balance::destroy_zero(principal);
        assert!(option::is_none(&haneul_token_lock), ETOKEN_TIME_LOCK_IS_SOME);
        option::destroy_none(haneul_token_lock);
    }


    // ==== getters and misc utility functions ====

    public fun haneul_balance(pool: &StakingPool) : u64 { pool.haneul_balance }

    public fun validator_address(staked_haneul: &StakedHaneul) : address { staked_haneul.validator_address }

    public fun staked_haneul_amount(staked_haneul: &StakedHaneul): u64 { balance::value(&staked_haneul.principal) }

    public fun delegation_request_epoch(staked_haneul: &StakedHaneul): u64 {
        staked_haneul.delegation_request_epoch
    }

    public fun delegation_token_amount(delegation: &Delegation): u64 { balance::value(&delegation.pool_tokens) }

    public fun pool_token_exchange_rate(pool: &StakingPool): PoolTokenExchangeRate {
        PoolTokenExchangeRate {
            haneul_amount: pool.haneul_balance,
            pool_token_amount: balance::supply_value(&pool.delegation_token_supply),
        }
    }
    /// Create a new pending withdraw entry.
    public(friend) fun new_pending_withdraw_entry(
        delegator: address, 
        principal_withdraw_amount: u64,
        withdrawn_pool_tokens: Balance<DelegationToken>,
    ) : PendingWithdrawEntry {
        PendingWithdrawEntry { delegator, principal_withdraw_amount, withdrawn_pool_tokens }
    }

    fun get_haneul_amount(pool: &StakingPool, token_amount: u64): u64 {
        let token_supply = balance::supply_value(&pool.delegation_token_supply);
        if (token_supply == 0) { 
            return token_amount 
        };
        let res = (pool.haneul_balance as u128) 
                * (token_amount as u128) 
                / (token_supply as u128);
        (res as u64)
    }

    fun get_token_amount(pool: &StakingPool, haneul_amount: u64): u64 {
        if (pool.haneul_balance == 0) { 
            return haneul_amount
        };
        let token_supply = balance::supply_value(&pool.delegation_token_supply);
        let res = (token_supply as u128) 
                * (haneul_amount as u128)
                / (pool.haneul_balance as u128);
        (res as u64)
    }    
}
