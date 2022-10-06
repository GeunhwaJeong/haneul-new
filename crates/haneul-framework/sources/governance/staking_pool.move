// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module haneul::staking_pool {
    use haneul::balance::{Self, Balance, Supply};
    use haneul::haneul::HANEUL;
    use std::option::{Self, Option};
    use haneul::tx_context::{Self, TxContext};
    use haneul::transfer;
    use haneul::epoch_time_lock::{EpochTimeLock};
    use haneul::object::{Self, UID};
    use haneul::locked_coin;
    use haneul::coin;
    use std::vector;

    friend haneul::validator;
    friend haneul::validator_set;
    
    const EINSUFFICIENT_POOL_TOKEN_BALANCE: u64 = 0;
    const EWRONG_POOL: u64 = 1;
    const EWITHDRAW_AMOUNT_CANNOT_BE_ZERO: u64 = 2;
    const EINSUFFICIENT_HANEUL_TOKEN_BALANCE: u64 = 3;
    const EINSUFFICIENT_REWARDS_POOL_BALANCE: u64 = 4;
    const EDESTROY_NON_ZERO_BALANCE: u64 = 5;
    const ETOKEN_TIME_LOCK_IS_SOME: u64 = 6;

    /// A staking pool embedded in each validator struct in the system state object.
    struct StakingPool has store {
        /// The haneul address of the validator associated with this pool.
        validator_address: address,
        /// The epoch at which this pool started operating. Should be the epoch at which the validator became active.
        starting_epoch: u64,
        /// The total number of HANEUL tokens in this pool at the beginning of the current epoch.
        epoch_starting_haneul_balance: u64,
        /// The total number of HANEUL tokens in this pool, including the HANEUL in the rewards_pool, as well as in all the principal
        /// in the `Delegation` object.
        haneul_balance: u64,
        /// The epoch delegation rewards will be added here at the end of each epoch. 
        rewards_pool: Balance<HANEUL>,
        /// The number of delegation pool tokens we have issued so far. This number should equal the sum of
        /// pool token balance in all the `Delegation` objects delegated to this pool.
        delegation_token_supply: Supply<DelegationToken>,
        /// Delegations requested during the current epoch. We will activate these delegation at the end of current epoch
        /// and distribute staking pool tokens at the end-of-epoch exchange rate after the rewards for the current epoch
        /// have been deposited.
        pending_delegations: vector<PendingDelegationEntry>,
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

    /// A self-custodial delegation object, serving as evidence that the delegator
    /// has delegated to a staking pool.
    struct Delegation has key {
        id: UID,
        /// The haneul address of the validator associated with the staking pool this object delgates to.
        validator_address: address,
        /// The epoch at which the staking pool started operating.
        pool_starting_epoch: u64,
        /// The pool tokens representing the amount of rewards the delegator can get back when they withdraw
        /// from the pool. If this field is `none`, that means the delegation hasn't been activated yet.
        pool_tokens: Balance<DelegationToken>,
        /// Number of HANEUL token staked originally.
        principal_haneul_amount: u64,
    }

    /// A self-custodial object holding the staked HANEUL tokens.
    struct StakedHaneul has key {
        id: UID,
        /// The staked HANEUL tokens.
        principal: Balance<HANEUL>,
        /// If the stake comes from a Coin<HANEUL>, this field is None. If it comes from a LockedCoin<HANEUL>, this
        /// field will record the original lock expiration epoch, to be used when unstaking.
        haneul_token_lock: Option<EpochTimeLock>,
    }

    /// Create a new, empty staking pool.
    public(friend) fun new(validator_address: address, starting_epoch: u64) : StakingPool {
        StakingPool {
            validator_address,
            starting_epoch,
            epoch_starting_haneul_balance: 0,
            haneul_balance: 0,
            rewards_pool: balance::zero(),
            delegation_token_supply: balance::create_supply(DelegationToken {}),
            pending_delegations: vector::empty(),
        }
    }

    /// Called at epoch advancement times to add rewards (in HANEUL) to the staking pool, and distribute new delegation tokens. 
    public(friend) fun advance_epoch(pool: &mut StakingPool, rewards: Balance<HANEUL>, ctx: &mut TxContext) {
        pool.haneul_balance = pool.haneul_balance + balance::value(&rewards);
        balance::join(&mut pool.rewards_pool, rewards);
        
        // distribute pool tokens at new exchange rate.
        while (!vector::is_empty(&pool.pending_delegations)) {
            let PendingDelegationEntry { delegator, haneul_amount } = vector::pop_back(&mut pool.pending_delegations);
            mint_delegation_tokens_to_delegator(pool, delegator, haneul_amount, ctx);
            pool.haneul_balance = pool.haneul_balance + haneul_amount
        };

        // Record the epoch starting balance.
        pool.epoch_starting_haneul_balance = pool.haneul_balance;
    }

    // TODO: implement rate limiting new delegations per epoch.
    /// Request to delegate to a staking pool. The delegation gets counted at the beginning of the next epoch,
    /// when the delegation object containing the pool tokens is distributed to the delegator.
    public(friend) fun request_add_delegation(
        pool: &mut StakingPool, 
        stake: Balance<HANEUL>, 
        haneul_token_lock: Option<EpochTimeLock>, 
        ctx: &mut TxContext
    ) {
        let haneul_amount = balance::value(&stake);
        assert!(haneul_amount > 0, 0);
        let delegator = tx_context::sender(ctx);
        // insert delegation info into the pendng_delegations vector.         
        vector::push_back(&mut pool.pending_delegations, PendingDelegationEntry { delegator, haneul_amount });
        let staked_haneul = StakedHaneul {
            id: object::new(ctx),
            principal: stake,
            haneul_token_lock,
        };
        transfer::transfer(staked_haneul, delegator);
    }

    /// Activate a delegation. New pool tokens are minted at the current exchange rate and put into the
    /// `pool_tokens` field of the delegation object.
    /// After activation, the delegation officially counts toward the staking power of the validator.
    /// Aborts if the pool mismatches, the delegation is already activated, or the delegation cannot be activated yet. 
    public(friend) fun mint_delegation_tokens_to_delegator(
        pool: &mut StakingPool, 
        delegator: address, 
        haneul_amount: u64, 
        ctx: &mut TxContext
    ) {
        let new_pool_token_amount = get_token_amount(pool, haneul_amount);   

        // Mint new pool tokens at the current exchange rate.
        let pool_tokens = balance::increase_supply(&mut pool.delegation_token_supply, new_pool_token_amount);

        let delegation = Delegation {
            id: object::new(ctx),
            validator_address: pool.validator_address,
            pool_starting_epoch: pool.starting_epoch,
            pool_tokens,
            principal_haneul_amount: haneul_amount,
        };

        transfer::transfer(delegation, delegator);
    }

    /// Withdraw `withdraw_pool_token_amount` worth of delegated stake from a staking pool. A proportional amount of principal and rewards
    /// in HANEUL will be withdrawn and transferred to the delegator. 
    /// Returns the amount of HANEUL withdrawn.
    public(friend) fun withdraw_stake(
        pool: &mut StakingPool,  
        delegation: &mut Delegation, 
        staked_haneul: &mut StakedHaneul,
        withdraw_pool_token_amount: u64, 
        ctx: &mut TxContext
    ) : u64 {
        let (principal_withdraw, reward_withdraw, time_lock) = 
            withdraw_to_haneul_tokens(pool, delegation, staked_haneul, withdraw_pool_token_amount);
        let haneul_withdraw_amount = balance::value(&principal_withdraw) + balance::value(&reward_withdraw);
        let delegator = tx_context::sender(ctx);

        // TODO: implement withdraw bonding period here.
        transfer::transfer(coin::from_balance(reward_withdraw, ctx), delegator);

        if (option::is_some(&time_lock)) {
            locked_coin::new_from_balance(principal_withdraw, option::destroy_some(time_lock), delegator, ctx);
        } else {
            transfer::transfer(coin::from_balance(principal_withdraw, ctx), delegator);
            option::destroy_none(time_lock);
        };
        haneul_withdraw_amount
    }

    /// Withdraw all the pool tokens in `delegation` object, with separate principal and rewards components, and
    /// then destroy the delegation object.
    public(friend) fun withdraw_all_to_haneul_tokens(
        pool: &mut StakingPool,  
        delegation: Delegation, 
        staked_haneul: &mut StakedHaneul,
    ) : (Balance<HANEUL>, Balance<HANEUL>, Option<EpochTimeLock>) {
        let withdraw_amount = balance::value(&delegation.pool_tokens);
        let (principal_withdraw, reward_withdraw, time_lock) = 
            withdraw_to_haneul_tokens(pool, &mut delegation, staked_haneul, withdraw_amount);
        destroy_empty_delegation(delegation);
        (principal_withdraw, reward_withdraw, time_lock)
    }

    fun withdraw_to_haneul_tokens(
        pool: &mut StakingPool,  
        delegation: &mut Delegation, 
        staked_haneul: &mut StakedHaneul,
        withdraw_pool_token_amount: u64, 
    ) : (Balance<HANEUL>, Balance<HANEUL>, Option<EpochTimeLock>) {
        assert!(
            delegation.validator_address == pool.validator_address &&
            delegation.pool_starting_epoch == pool.starting_epoch,
            EWRONG_POOL
        );

        assert!(withdraw_pool_token_amount > 0, EWITHDRAW_AMOUNT_CANNOT_BE_ZERO);

        let pool_token_balance = balance::value(&delegation.pool_tokens);
        assert!(pool_token_balance >= withdraw_pool_token_amount, EINSUFFICIENT_POOL_TOKEN_BALANCE);

        // Calculate the amount of HANEUL tokens that should be withdrawn from the pool using the current exchange rate.
        let haneul_withdraw_amount = get_haneul_amount(pool, withdraw_pool_token_amount);

        // decrement haneul balance in the pool
        pool.haneul_balance = pool.haneul_balance - haneul_withdraw_amount;

        // Calculate the amounts of HANEUL to be withdrawn from the principal component and the rewards component.
        // We already checked that pool_token_balance is greater than zero.
        let haneul_withdraw_from_principal = 
            (delegation.principal_haneul_amount as u128) * (withdraw_pool_token_amount as u128) / (pool_token_balance as u128);
        let haneul_withdraw_from_rewards = haneul_withdraw_amount - (haneul_withdraw_from_principal as u64); 

        // burn the pool tokens
        balance::decrease_supply(
            &mut pool.delegation_token_supply, 
            balance::split(&mut delegation.pool_tokens, withdraw_pool_token_amount)
        );

        let (principal_withdraw, time_lock) = withdraw_from_principal(delegation, staked_haneul, (haneul_withdraw_from_principal as u64));

        // withdraw the rewards component from rewards pool and transfer it to the delegator.
        assert!(balance::value(&pool.rewards_pool) >= haneul_withdraw_from_rewards, EINSUFFICIENT_REWARDS_POOL_BALANCE);
        let reward_withdraw = balance::split(&mut pool.rewards_pool, haneul_withdraw_from_rewards);

        (principal_withdraw, reward_withdraw, time_lock)
    }

    /// Deactivate a staking pool by wrapping it in an `InactiveStakingPool` and sharing this newly created object. 
    /// After this pool deactivation, the pool stops earning rewards. Only delegation withdraws can be made to the pool.
    public(friend) fun deactivate_staking_pool(pool: StakingPool, ctx: &mut TxContext) {
        let inactive_pool = InactiveStakingPool { id: object::new(ctx), pool};
        transfer::share_object(inactive_pool);
    }

    /// Withdraw delegation from an inactive pool.
    public entry fun withdraw_from_inactive_pool(
        inactive_pool: &mut InactiveStakingPool, 
        staked_haneul: &mut StakedHaneul, 
        delegation: &mut Delegation, 
        withdraw_amount: u64, 
        ctx: &mut TxContext
    ) {
        withdraw_stake(&mut inactive_pool.pool, delegation, staked_haneul, withdraw_amount, ctx);
    }

    /// Destroy an empty delegation that no longer contains any HANEUL or pool tokens.
    public entry fun destroy_empty_delegation(delegation: Delegation) {
        let Delegation {
            id,
            validator_address: _,
            pool_starting_epoch: _,
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
            principal,
            haneul_token_lock
        } = staked_haneul;
        object::delete(id);
        assert!(balance::value(&principal) == 0, EDESTROY_NON_ZERO_BALANCE);
        balance::destroy_zero(principal);
        assert!(option::is_none(&haneul_token_lock), ETOKEN_TIME_LOCK_IS_SOME);
        option::destroy_none(haneul_token_lock);
    }

    public fun haneul_balance(pool: &StakingPool) : u64 { pool.epoch_starting_haneul_balance }

    public fun validator_address(delegation: &Delegation) : address { delegation.validator_address }

    public fun staked_haneul_amount(staked_haneul: &StakedHaneul): u64 { balance::value(&staked_haneul.principal) }

    public fun delegation_token_amount(delegation: &Delegation): u64 { balance::value(&delegation.pool_tokens) }

    /// Withdraw `withdraw_amount` of HANEUL tokens from the delegation and give it back to the delegator
    /// in the original state of the tokens.
    fun withdraw_from_principal(
        delegation: &mut Delegation, 
        staked_haneul: &mut StakedHaneul, 
        withdraw_amount: u64,
    ) : (Balance<HANEUL>, Option<EpochTimeLock>) {
        assert!(balance::value(&staked_haneul.principal) >= withdraw_amount, EINSUFFICIENT_HANEUL_TOKEN_BALANCE);
        delegation.principal_haneul_amount = delegation.principal_haneul_amount - withdraw_amount;
        let principal_withdraw = balance::split(&mut staked_haneul.principal, withdraw_amount);
        if (option::is_some(&staked_haneul.haneul_token_lock)) {
            let time_lock = 
                if (balance::value(&staked_haneul.principal) == 0) {option::extract(&mut staked_haneul.haneul_token_lock)}
                else *option::borrow(&staked_haneul.haneul_token_lock);
            (principal_withdraw, option::some(time_lock))
        } else {
            (principal_withdraw, option::none())
        }
    }

    fun get_haneul_amount(pool: &StakingPool, token_amount: u64): u64 {
        let token_supply_amount = balance::supply_value(&pool.delegation_token_supply);
        if (token_supply_amount == 0) { 
            return token_amount 
        };
        let res = (pool.haneul_balance as u128) * (token_amount as u128) / (token_supply_amount as u128);
        (res as u64)
    }

    fun get_token_amount(pool: &StakingPool, haneul_amount: u64): u64 {
        let token_supply_amount = balance::supply_value(&pool.delegation_token_supply);
        if (pool.haneul_balance == 0) { 
            return haneul_amount
        };
        let res = (token_supply_amount as u128) * (haneul_amount as u128) / (pool.haneul_balance as u128);
        (res as u64)
    }    
}
