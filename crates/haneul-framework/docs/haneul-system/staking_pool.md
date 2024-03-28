---
title: Module `0x3::staking_pool`
---



-  [Resource `StakingPool`](#0x3_staking_pool_StakingPool)
-  [Struct `PoolTokenExchangeRate`](#0x3_staking_pool_PoolTokenExchangeRate)
-  [Resource `StakedHaneul`](#0x3_staking_pool_StakedHaneul)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x3_staking_pool_new)
-  [Function `request_add_stake`](#0x3_staking_pool_request_add_stake)
-  [Function `request_withdraw_stake`](#0x3_staking_pool_request_withdraw_stake)
-  [Function `withdraw_from_principal`](#0x3_staking_pool_withdraw_from_principal)
-  [Function `unwrap_staked_haneul`](#0x3_staking_pool_unwrap_staked_haneul)
-  [Function `deposit_rewards`](#0x3_staking_pool_deposit_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#0x3_staking_pool_process_pending_stakes_and_withdraws)
-  [Function `process_pending_stake_withdraw`](#0x3_staking_pool_process_pending_stake_withdraw)
-  [Function `process_pending_stake`](#0x3_staking_pool_process_pending_stake)
-  [Function `withdraw_rewards`](#0x3_staking_pool_withdraw_rewards)
-  [Function `activate_staking_pool`](#0x3_staking_pool_activate_staking_pool)
-  [Function `deactivate_staking_pool`](#0x3_staking_pool_deactivate_staking_pool)
-  [Function `haneul_balance`](#0x3_staking_pool_haneul_balance)
-  [Function `pool_id`](#0x3_staking_pool_pool_id)
-  [Function `staked_haneul_amount`](#0x3_staking_pool_staked_haneul_amount)
-  [Function `stake_activation_epoch`](#0x3_staking_pool_stake_activation_epoch)
-  [Function `is_preactive`](#0x3_staking_pool_is_preactive)
-  [Function `is_inactive`](#0x3_staking_pool_is_inactive)
-  [Function `split`](#0x3_staking_pool_split)
-  [Function `split_staked_haneul`](#0x3_staking_pool_split_staked_haneul)
-  [Function `join_staked_haneul`](#0x3_staking_pool_join_staked_haneul)
-  [Function `is_equal_staking_metadata`](#0x3_staking_pool_is_equal_staking_metadata)
-  [Function `pool_token_exchange_rate_at_epoch`](#0x3_staking_pool_pool_token_exchange_rate_at_epoch)
-  [Function `pending_stake_amount`](#0x3_staking_pool_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#0x3_staking_pool_pending_stake_withdraw_amount)
-  [Function `exchange_rates`](#0x3_staking_pool_exchange_rates)
-  [Function `haneul_amount`](#0x3_staking_pool_haneul_amount)
-  [Function `pool_token_amount`](#0x3_staking_pool_pool_token_amount)
-  [Function `is_preactive_at_epoch`](#0x3_staking_pool_is_preactive_at_epoch)
-  [Function `get_haneul_amount`](#0x3_staking_pool_get_haneul_amount)
-  [Function `get_token_amount`](#0x3_staking_pool_get_token_amount)
-  [Function `initial_exchange_rate`](#0x3_staking_pool_initial_exchange_rate)
-  [Function `check_balance_invariants`](#0x3_staking_pool_check_balance_invariants)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../haneul-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../haneul-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../haneul-framework/math.md#0x2_math">0x2::math</a>;
<b>use</b> <a href="../haneul-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../haneul-framework/haneul.md#0x2_haneul">0x2::haneul</a>;
<b>use</b> <a href="../haneul-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="../haneul-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../haneul-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x3_staking_pool_StakingPool"></a>

## Resource `StakingPool`

A staking pool embedded in each validator struct in the system state object.


<pre><code><b>struct</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../haneul-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>activation_epoch: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this pool became active.
 The value is <code>None</code> if the pool is pre-active and <code>Some(&lt;epoch_number&gt;)</code> if active or inactive.
</dd>
<dt>
<code>deactivation_epoch: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this staking pool ceased to be active. <code>None</code> = {pre-active, active},
 <code>Some(&lt;epoch_number&gt;)</code> if in-active, and it was de-activated at epoch <code>&lt;epoch_number&gt;</code>.
</dd>
<dt>
<code>haneul_balance: u64</code>
</dt>
<dd>
 The total number of HANEUL tokens in this pool, including the HANEUL in the rewards_pool, as well as in all the principal
 in the <code><a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a></code> object, updated at epoch boundaries.
</dd>
<dt>
<code>rewards_pool: <a href="../haneul-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../haneul-framework/haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;</code>
</dt>
<dd>
 The epoch stake rewards will be added here at the end of each epoch.
</dd>
<dt>
<code>pool_token_balance: u64</code>
</dt>
<dd>
 Total number of pool tokens issued by the pool.
</dd>
<dt>
<code>exchange_rates: <a href="../haneul-framework/table.md#0x2_table_Table">table::Table</a>&lt;u64, <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;</code>
</dt>
<dd>
 Exchange rate history of previous epochs. Key is the epoch number.
 The entries start from the <code>activation_epoch</code> of this pool and contains exchange rates at the beginning of each epoch,
 i.e., right after the rewards for the previous epoch have been deposited into the pool.
</dd>
<dt>
<code>pending_stake: u64</code>
</dt>
<dd>
 Pending stake amount for this epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>pending_total_haneul_withdraw: u64</code>
</dt>
<dd>
 Pending stake withdrawn during the current epoch, emptied at epoch boundaries.
 This includes both the principal and rewards HANEUL withdrawn.
</dd>
<dt>
<code>pending_pool_token_withdraw: u64</code>
</dt>
<dd>
 Pending pool token withdrawn during the current epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>extra_fields: <a href="../haneul-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x3_staking_pool_PoolTokenExchangeRate"></a>

## Struct `PoolTokenExchangeRate`

Struct representing the exchange rate of the stake pool token to HANEUL.


<pre><code><b>struct</b> <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>haneul_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>pool_token_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_staking_pool_StakedHaneul"></a>

## Resource `StakedHaneul`

A self-custodial object holding the staked HANEUL tokens.


<pre><code><b>struct</b> <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../haneul-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>pool_id: <a href="../haneul-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 ID of the staking pool we are staking with.
</dd>
<dt>
<code>stake_activation_epoch: u64</code>
</dt>
<dd>
 The epoch at which the stake becomes active.
</dd>
<dt>
<code>principal: <a href="../haneul-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../haneul-framework/haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;</code>
</dt>
<dd>
 The staked HANEUL tokens.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_staking_pool_EActivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>: u64 = 16;
</code></pre>



<a name="0x3_staking_pool_EDeactivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>: u64 = 11;
</code></pre>



<a name="0x3_staking_pool_EDelegationOfZeroHaneul"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EDelegationOfZeroHaneul">EDelegationOfZeroHaneul</a>: u64 = 17;
</code></pre>



<a name="0x3_staking_pool_EDelegationToInactivePool"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>: u64 = 10;
</code></pre>



<a name="0x3_staking_pool_EDestroyNonzeroBalance"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EDestroyNonzeroBalance">EDestroyNonzeroBalance</a>: u64 = 5;
</code></pre>



<a name="0x3_staking_pool_EIncompatibleStakedHaneul"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EIncompatibleStakedHaneul">EIncompatibleStakedHaneul</a>: u64 = 12;
</code></pre>



<a name="0x3_staking_pool_EInsufficientPoolTokenBalance"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>: u64 = 0;
</code></pre>



<a name="0x3_staking_pool_EInsufficientRewardsPoolBalance"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EInsufficientRewardsPoolBalance">EInsufficientRewardsPoolBalance</a>: u64 = 4;
</code></pre>



<a name="0x3_staking_pool_EInsufficientHaneulTokenBalance"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EInsufficientHaneulTokenBalance">EInsufficientHaneulTokenBalance</a>: u64 = 3;
</code></pre>



<a name="0x3_staking_pool_EPendingDelegationDoesNotExist"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EPendingDelegationDoesNotExist">EPendingDelegationDoesNotExist</a>: u64 = 8;
</code></pre>



<a name="0x3_staking_pool_EPoolAlreadyActive"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>: u64 = 14;
</code></pre>



<a name="0x3_staking_pool_EPoolNotPreactive"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EPoolNotPreactive">EPoolNotPreactive</a>: u64 = 15;
</code></pre>



<a name="0x3_staking_pool_EStakedHaneulBelowThreshold"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EStakedHaneulBelowThreshold">EStakedHaneulBelowThreshold</a>: u64 = 18;
</code></pre>



<a name="0x3_staking_pool_ETokenBalancesDoNotMatchExchangeRate"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>: u64 = 9;
</code></pre>



<a name="0x3_staking_pool_ETokenTimeLockIsSome"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_ETokenTimeLockIsSome">ETokenTimeLockIsSome</a>: u64 = 6;
</code></pre>



<a name="0x3_staking_pool_EWithdrawAmountCannotBeZero"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EWithdrawAmountCannotBeZero">EWithdrawAmountCannotBeZero</a>: u64 = 2;
</code></pre>



<a name="0x3_staking_pool_EWithdrawalInSameEpoch"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EWithdrawalInSameEpoch">EWithdrawalInSameEpoch</a>: u64 = 13;
</code></pre>



<a name="0x3_staking_pool_EWrongDelegation"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EWrongDelegation">EWrongDelegation</a>: u64 = 7;
</code></pre>



<a name="0x3_staking_pool_EWrongPool"></a>



<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_EWrongPool">EWrongPool</a>: u64 = 1;
</code></pre>



<a name="0x3_staking_pool_MIN_STAKING_THRESHOLD"></a>

StakedHaneul objects cannot be split to below this amount.


<pre><code><b>const</b> <a href="staking_pool.md#0x3_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="0x3_staking_pool_new"></a>

## Function `new`

Create a new, empty staking pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_new">new</a>(ctx: &<b>mut</b> <a href="../haneul-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_new">new</a>(ctx: &<b>mut</b> TxContext) : <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a> {
    <b>let</b> exchange_rates = <a href="../haneul-framework/table.md#0x2_table_new">table::new</a>(ctx);
    <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a> {
        id: <a href="../haneul-framework/object.md#0x2_object_new">object::new</a>(ctx),
        activation_epoch: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        deactivation_epoch: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        haneul_balance: 0,
        rewards_pool: <a href="../haneul-framework/balance.md#0x2_balance_zero">balance::zero</a>(),
        pool_token_balance: 0,
        exchange_rates,
        pending_stake: 0,
        pending_total_haneul_withdraw: 0,
        pending_pool_token_withdraw: 0,
        extra_fields: <a href="../haneul-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
    }
}
</code></pre>



</details>

<a name="0x3_staking_pool_request_add_stake"></a>

## Function `request_add_stake`

Request to stake to a staking pool. The stake starts counting at the beginning of the next epoch,


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_request_add_stake">request_add_stake</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, stake: <a href="../haneul-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../haneul-framework/haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;, stake_activation_epoch: u64, ctx: &<b>mut</b> <a href="../haneul-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_request_add_stake">request_add_stake</a>(
    pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>,
    stake: Balance&lt;HANEUL&gt;,
    stake_activation_epoch: u64,
    ctx: &<b>mut</b> TxContext
) : <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a> {
    <b>let</b> haneul_amount = stake.value();
    <b>assert</b>!(!<a href="staking_pool.md#0x3_staking_pool_is_inactive">is_inactive</a>(pool), <a href="staking_pool.md#0x3_staking_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>);
    <b>assert</b>!(haneul_amount &gt; 0, <a href="staking_pool.md#0x3_staking_pool_EDelegationOfZeroHaneul">EDelegationOfZeroHaneul</a>);
    <b>let</b> staked_haneul = <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a> {
        id: <a href="../haneul-framework/object.md#0x2_object_new">object::new</a>(ctx),
        pool_id: <a href="../haneul-framework/object.md#0x2_object_id">object::id</a>(pool),
        stake_activation_epoch,
        principal: stake,
    };
    pool.pending_stake = pool.pending_stake + haneul_amount;
    staked_haneul
}
</code></pre>



</details>

<a name="0x3_staking_pool_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw the given stake plus rewards from a staking pool.
Both the principal and corresponding rewards in HANEUL are withdrawn.
A proportional amount of pool token withdraw is recorded and processed at epoch change time.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, staked_haneul: <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>, ctx: &<a href="../haneul-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../haneul-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../haneul-framework/haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_request_withdraw_stake">request_withdraw_stake</a>(
    pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>,
    staked_haneul: <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>,
    ctx: &TxContext
) : Balance&lt;HANEUL&gt; {
    <b>let</b> (pool_token_withdraw_amount, <b>mut</b> principal_withdraw) =
        <a href="staking_pool.md#0x3_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(pool, staked_haneul);
    <b>let</b> principal_withdraw_amount = principal_withdraw.value();

    <b>let</b> rewards_withdraw = <a href="staking_pool.md#0x3_staking_pool_withdraw_rewards">withdraw_rewards</a>(
        pool, principal_withdraw_amount, pool_token_withdraw_amount, ctx.epoch()
    );
    <b>let</b> total_haneul_withdraw_amount = principal_withdraw_amount + rewards_withdraw.value();

    pool.pending_total_haneul_withdraw = pool.pending_total_haneul_withdraw + total_haneul_withdraw_amount;
    pool.pending_pool_token_withdraw = pool.pending_pool_token_withdraw + pool_token_withdraw_amount;

    // If the pool is inactive, we immediately process the withdrawal.
    <b>if</b> (<a href="staking_pool.md#0x3_staking_pool_is_inactive">is_inactive</a>(pool)) <a href="staking_pool.md#0x3_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool);

    // TODO: implement withdraw bonding period here.
    principal_withdraw.join(rewards_withdraw);
    principal_withdraw
}
</code></pre>



</details>

<a name="0x3_staking_pool_withdraw_from_principal"></a>

## Function `withdraw_from_principal`

Withdraw the principal HANEUL stored in the StakedHaneul object, and calculate the corresponding amount of pool
tokens using exchange rate at staking epoch.
Returns values are amount of pool tokens withdrawn and withdrawn principal portion of HANEUL.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, staked_haneul: <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>): (u64, <a href="../haneul-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../haneul-framework/haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_withdraw_from_principal">withdraw_from_principal</a>(
    pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>,
    staked_haneul: <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>,
) : (u64, Balance&lt;HANEUL&gt;) {

    // Check that the stake information matches the pool.
    <b>assert</b>!(staked_haneul.pool_id == <a href="../haneul-framework/object.md#0x2_object_id">object::id</a>(pool), <a href="staking_pool.md#0x3_staking_pool_EWrongPool">EWrongPool</a>);

    <b>let</b> exchange_rate_at_staking_epoch = <a href="staking_pool.md#0x3_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, staked_haneul.stake_activation_epoch);
    <b>let</b> principal_withdraw = <a href="staking_pool.md#0x3_staking_pool_unwrap_staked_haneul">unwrap_staked_haneul</a>(staked_haneul);
    <b>let</b> pool_token_withdraw_amount = <a href="staking_pool.md#0x3_staking_pool_get_token_amount">get_token_amount</a>(
		&exchange_rate_at_staking_epoch,
		principal_withdraw.value()
	);

    (
        pool_token_withdraw_amount,
        principal_withdraw,
    )
}
</code></pre>



</details>

<a name="0x3_staking_pool_unwrap_staked_haneul"></a>

## Function `unwrap_staked_haneul`



<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_unwrap_staked_haneul">unwrap_staked_haneul</a>(staked_haneul: <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>): <a href="../haneul-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../haneul-framework/haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_unwrap_staked_haneul">unwrap_staked_haneul</a>(staked_haneul: <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>): Balance&lt;HANEUL&gt; {
    <b>let</b> <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a> {
        id,
        pool_id: _,
        stake_activation_epoch: _,
        principal,
    } = staked_haneul;
    <a href="../haneul-framework/object.md#0x2_object_delete">object::delete</a>(id);
    principal
}
</code></pre>



</details>

<a name="0x3_staking_pool_deposit_rewards"></a>

## Function `deposit_rewards`

Called at epoch advancement times to add rewards (in HANEUL) to the staking pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, rewards: <a href="../haneul-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../haneul-framework/haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_deposit_rewards">deposit_rewards</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>, rewards: Balance&lt;HANEUL&gt;) {
    pool.haneul_balance = pool.haneul_balance + rewards.value();
    pool.rewards_pool.join(rewards);
}
</code></pre>



</details>

<a name="0x3_staking_pool_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, ctx: &<a href="../haneul-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>, ctx: &TxContext) {
    <b>let</b> new_epoch = ctx.epoch() + 1;
    <a href="staking_pool.md#0x3_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool);
    <a href="staking_pool.md#0x3_staking_pool_process_pending_stake">process_pending_stake</a>(pool);
    pool.exchange_rates.add(
        new_epoch,
        <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { haneul_amount: pool.haneul_balance, pool_token_amount: pool.pool_token_balance },
    );
    <a href="staking_pool.md#0x3_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool, new_epoch);
}
</code></pre>



</details>

<a name="0x3_staking_pool_process_pending_stake_withdraw"></a>

## Function `process_pending_stake_withdraw`

Called at epoch boundaries to process pending stake withdraws requested during the epoch.
Also called immediately upon withdrawal if the pool is inactive.


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>) {
    pool.haneul_balance = pool.haneul_balance - pool.pending_total_haneul_withdraw;
    pool.pool_token_balance = pool.pool_token_balance - pool.pending_pool_token_withdraw;
    pool.pending_total_haneul_withdraw = 0;
    pool.pending_pool_token_withdraw = 0;
}
</code></pre>



</details>

<a name="0x3_staking_pool_process_pending_stake"></a>

## Function `process_pending_stake`

Called at epoch boundaries to process the pending stake.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_process_pending_stake">process_pending_stake</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>) {
    // Use the most up <b>to</b> date exchange rate <b>with</b> the rewards deposited and withdraws effectuated.
    <b>let</b> latest_exchange_rate =
        <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { haneul_amount: pool.haneul_balance, pool_token_amount: pool.pool_token_balance };
    pool.haneul_balance = pool.haneul_balance + pool.pending_stake;
    pool.pool_token_balance = <a href="staking_pool.md#0x3_staking_pool_get_token_amount">get_token_amount</a>(&latest_exchange_rate, pool.haneul_balance);
    pool.pending_stake = 0;
}
</code></pre>



</details>

<a name="0x3_staking_pool_withdraw_rewards"></a>

## Function `withdraw_rewards`

This function does the following:
1. Calculates the total amount of HANEUL (including principal and rewards) that the provided pool tokens represent
at the current exchange rate.
2. Using the above number and the given <code>principal_withdraw_amount</code>, calculates the rewards portion of the
stake we should withdraw.
3. Withdraws the rewards portion from the rewards pool at the current exchange rate. We only withdraw the rewards
portion because the principal portion was already taken out of the staker's self custodied StakedHaneul.


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_withdraw_rewards">withdraw_rewards</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, principal_withdraw_amount: u64, pool_token_withdraw_amount: u64, epoch: u64): <a href="../haneul-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../haneul-framework/haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_withdraw_rewards">withdraw_rewards</a>(
    pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>,
    principal_withdraw_amount: u64,
    pool_token_withdraw_amount: u64,
    epoch: u64,
) : Balance&lt;HANEUL&gt; {
    <b>let</b> exchange_rate = <a href="staking_pool.md#0x3_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, epoch);
    <b>let</b> total_haneul_withdraw_amount = <a href="staking_pool.md#0x3_staking_pool_get_haneul_amount">get_haneul_amount</a>(&exchange_rate, pool_token_withdraw_amount);
    <b>let</b> <b>mut</b> reward_withdraw_amount =
        <b>if</b> (total_haneul_withdraw_amount &gt;= principal_withdraw_amount)
            total_haneul_withdraw_amount - principal_withdraw_amount
        <b>else</b> 0;
    // This may happen when we are withdrawing everything from the pool and
    // the rewards pool <a href="../haneul-framework/balance.md#0x2_balance">balance</a> may be less than reward_withdraw_amount.
    // TODO: FIGURE OUT EXACTLY WHY THIS CAN HAPPEN.
    reward_withdraw_amount = <a href="../haneul-framework/math.md#0x2_math_min">math::min</a>(reward_withdraw_amount, pool.rewards_pool.value());
    pool.rewards_pool.<a href="staking_pool.md#0x3_staking_pool_split">split</a>(reward_withdraw_amount)
}
</code></pre>



</details>

<a name="0x3_staking_pool_activate_staking_pool"></a>

## Function `activate_staking_pool`

Called by <code><a href="validator.md#0x3_validator">validator</a></code> module to activate a staking pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, activation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_activate_staking_pool">activate_staking_pool</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>, activation_epoch: u64) {
    // Add the initial exchange rate <b>to</b> the <a href="../haneul-framework/table.md#0x2_table">table</a>.
    pool.exchange_rates.add(
        activation_epoch,
        <a href="staking_pool.md#0x3_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
    );
    // Check that the pool is preactive and not inactive.
    <b>assert</b>!(<a href="staking_pool.md#0x3_staking_pool_is_preactive">is_preactive</a>(pool), <a href="staking_pool.md#0x3_staking_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>);
    <b>assert</b>!(!<a href="staking_pool.md#0x3_staking_pool_is_inactive">is_inactive</a>(pool), <a href="staking_pool.md#0x3_staking_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>);
    // Fill in the active epoch.
    pool.activation_epoch.fill(activation_epoch);
}
</code></pre>



</details>

<a name="0x3_staking_pool_deactivate_staking_pool"></a>

## Function `deactivate_staking_pool`

Deactivate a staking pool by setting the <code>deactivation_epoch</code>. After
this pool deactivation, the pool stops earning rewards. Only stake
withdraws can be made to the pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_deactivate_staking_pool">deactivate_staking_pool</a>(pool: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>, deactivation_epoch: u64) {
    // We can't deactivate an already deactivated pool.
    <b>assert</b>!(!<a href="staking_pool.md#0x3_staking_pool_is_inactive">is_inactive</a>(pool), <a href="staking_pool.md#0x3_staking_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>);
    pool.deactivation_epoch = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(deactivation_epoch);
}
</code></pre>



</details>

<a name="0x3_staking_pool_haneul_balance"></a>

## Function `haneul_balance`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_haneul_balance">haneul_balance</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_haneul_balance">haneul_balance</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>): u64 { pool.haneul_balance }
</code></pre>



</details>

<a name="0x3_staking_pool_pool_id"></a>

## Function `pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_pool_id">pool_id</a>(staked_haneul: &<a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>): <a href="../haneul-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_pool_id">pool_id</a>(staked_haneul: &<a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>): ID { staked_haneul.pool_id }
</code></pre>



</details>

<a name="0x3_staking_pool_staked_haneul_amount"></a>

## Function `staked_haneul_amount`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_staked_haneul_amount">staked_haneul_amount</a>(staked_haneul: &<a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_staked_haneul_amount">staked_haneul_amount</a>(staked_haneul: &<a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>): u64 { staked_haneul.principal.value() }
</code></pre>



</details>

<a name="0x3_staking_pool_stake_activation_epoch"></a>

## Function `stake_activation_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_stake_activation_epoch">stake_activation_epoch</a>(staked_haneul: &<a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_stake_activation_epoch">stake_activation_epoch</a>(staked_haneul: &<a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>): u64 {
    staked_haneul.stake_activation_epoch
}
</code></pre>



</details>

<a name="0x3_staking_pool_is_preactive"></a>

## Function `is_preactive`

Returns true if the input staking pool is preactive.


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_is_preactive">is_preactive</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_is_preactive">is_preactive</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>): bool{
    pool.activation_epoch.is_none()
}
</code></pre>



</details>

<a name="0x3_staking_pool_is_inactive"></a>

## Function `is_inactive`

Returns true if the input staking pool is inactive.


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_is_inactive">is_inactive</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>): bool {
    pool.deactivation_epoch.is_some()
}
</code></pre>



</details>

<a name="0x3_staking_pool_split"></a>

## Function `split`

Split StakedHaneul <code>self</code> to two parts, one with principal <code>split_amount</code>,
and the remaining principal is left in <code>self</code>.
All the other parameters of the StakedHaneul like <code>stake_activation_epoch</code> or <code>pool_id</code> remain the same.


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_split">split</a>(self: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../haneul-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_split">split</a>(self: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>, split_amount: u64, ctx: &<b>mut</b> TxContext): <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a> {
    <b>let</b> original_amount = self.principal.value();
    <b>assert</b>!(split_amount &lt;= original_amount, <a href="staking_pool.md#0x3_staking_pool_EInsufficientHaneulTokenBalance">EInsufficientHaneulTokenBalance</a>);
    <b>let</b> remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least <a href="staking_pool.md#0x3_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(remaining_amount &gt;= <a href="staking_pool.md#0x3_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="staking_pool.md#0x3_staking_pool_EStakedHaneulBelowThreshold">EStakedHaneulBelowThreshold</a>);
    <b>assert</b>!(split_amount &gt;= <a href="staking_pool.md#0x3_staking_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="staking_pool.md#0x3_staking_pool_EStakedHaneulBelowThreshold">EStakedHaneulBelowThreshold</a>);
    <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a> {
        id: <a href="../haneul-framework/object.md#0x2_object_new">object::new</a>(ctx),
        pool_id: self.pool_id,
        stake_activation_epoch: self.stake_activation_epoch,
        principal: self.principal.<a href="staking_pool.md#0x3_staking_pool_split">split</a>(split_amount),
    }
}
</code></pre>



</details>

<a name="0x3_staking_pool_split_staked_haneul"></a>

## Function `split_staked_haneul`

Split the given StakedHaneul to the two parts, one with principal <code>split_amount</code>,
transfer the newly split part to the sender address.


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_split_staked_haneul">split_staked_haneul</a>(stake: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../haneul-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_split_staked_haneul">split_staked_haneul</a>(stake: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    <a href="../haneul-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(<a href="staking_pool.md#0x3_staking_pool_split">split</a>(stake, split_amount, ctx), ctx.sender());
}
</code></pre>



</details>

<a name="0x3_staking_pool_join_staked_haneul"></a>

## Function `join_staked_haneul`

Consume the staked haneul <code>other</code> and add its value to <code>self</code>.
Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_join_staked_haneul">join_staked_haneul</a>(self: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>, other: <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_join_staked_haneul">join_staked_haneul</a>(self: &<b>mut</b> <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>, other: <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>) {
    <b>assert</b>!(<a href="staking_pool.md#0x3_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self, &other), <a href="staking_pool.md#0x3_staking_pool_EIncompatibleStakedHaneul">EIncompatibleStakedHaneul</a>);
    <b>let</b> <a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a> {
        id,
        pool_id: _,
        stake_activation_epoch: _,
        principal,
    } = other;

    id.delete();
    self.principal.join(principal);
}
</code></pre>



</details>

<a name="0x3_staking_pool_is_equal_staking_metadata"></a>

## Function `is_equal_staking_metadata`

Returns true if all the staking parameters of the staked haneul except the principal are identical


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>, other: &<a href="staking_pool.md#0x3_staking_pool_StakedHaneul">staking_pool::StakedHaneul</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>, other: &<a href="staking_pool.md#0x3_staking_pool_StakedHaneul">StakedHaneul</a>): bool {
    (self.pool_id == other.pool_id) &&
    (self.stake_activation_epoch == other.stake_activation_epoch)
}
</code></pre>



</details>

<a name="0x3_staking_pool_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64): <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>, epoch: u64): <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    // If the pool is preactive then the exchange rate is always 1:1.
    <b>if</b> (<a href="staking_pool.md#0x3_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool, epoch)) {
        <b>return</b> <a href="staking_pool.md#0x3_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
    };
    <b>let</b> clamped_epoch = pool.deactivation_epoch.get_with_default(epoch);
    <b>let</b> <b>mut</b> epoch = <a href="../haneul-framework/math.md#0x2_math_min">math::min</a>(clamped_epoch, epoch);
    <b>let</b> activation_epoch = *pool.activation_epoch.borrow();

    // Find the latest epoch that's earlier than the given epoch <b>with</b> an entry in the <a href="../haneul-framework/table.md#0x2_table">table</a>
    <b>while</b> (epoch &gt;= activation_epoch) {
        <b>if</b> (pool.exchange_rates.contains(epoch)) {
            <b>return</b> pool.exchange_rates[epoch]
        };
        epoch = epoch - 1;
    };
    // This line really should be unreachable. Do we want an <b>assert</b> <b>false</b> here?
    <a href="staking_pool.md#0x3_staking_pool_initial_exchange_rate">initial_exchange_rate</a>()
}
</code></pre>



</details>

<a name="0x3_staking_pool_pending_stake_amount"></a>

## Function `pending_stake_amount`

Returns the total value of the pending staking requests for this staking pool.


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_pending_stake_amount">pending_stake_amount</a>(<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>): u64 {
    <a href="staking_pool.md#0x3_staking_pool">staking_pool</a>.pending_stake
}
</code></pre>



</details>

<a name="0x3_staking_pool_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`

Returns the total withdrawal from the staking pool this epoch.


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>): u64 {
    <a href="staking_pool.md#0x3_staking_pool">staking_pool</a>.pending_total_haneul_withdraw
}
</code></pre>



</details>

<a name="0x3_staking_pool_exchange_rates"></a>

## Function `exchange_rates`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>): &<a href="../haneul-framework/table.md#0x2_table_Table">table::Table</a>&lt;u64, <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_exchange_rates">exchange_rates</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>): &Table&lt;u64, <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>&gt; {
    &pool.exchange_rates
}
</code></pre>



</details>

<a name="0x3_staking_pool_haneul_amount"></a>

## Function `haneul_amount`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_haneul_amount">haneul_amount</a>(exchange_rate: &<a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_haneul_amount">haneul_amount</a>(exchange_rate: &<a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.haneul_amount
}
</code></pre>



</details>

<a name="0x3_staking_pool_pool_token_amount"></a>

## Function `pool_token_amount`



<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="staking_pool.md#0x3_staking_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.pool_token_amount
}
</code></pre>



</details>

<a name="0x3_staking_pool_is_preactive_at_epoch"></a>

## Function `is_preactive_at_epoch`

Returns true if the provided staking pool is preactive at the provided epoch.


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>, epoch: u64): bool{
    // Either the pool is currently preactive or the pool's starting epoch is later than the provided epoch.
    <a href="staking_pool.md#0x3_staking_pool_is_preactive">is_preactive</a>(pool) || (*pool.activation_epoch.borrow() &gt; epoch)
}
</code></pre>



</details>

<a name="0x3_staking_pool_get_haneul_amount"></a>

## Function `get_haneul_amount`



<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_get_haneul_amount">get_haneul_amount</a>(exchange_rate: &<a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>, token_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_get_haneul_amount">get_haneul_amount</a>(exchange_rate: &<a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, token_amount: u64): u64 {
    // When either amount is 0, that means we have no stakes <b>with</b> this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.haneul_amount == 0 || exchange_rate.pool_token_amount == 0) {
        <b>return</b> token_amount
    };
    <b>let</b> res = exchange_rate.haneul_amount <b>as</b> u128
            * (token_amount <b>as</b> u128)
            / (exchange_rate.pool_token_amount <b>as</b> u128);
    res <b>as</b> u64
}
</code></pre>



</details>

<a name="0x3_staking_pool_get_token_amount"></a>

## Function `get_token_amount`



<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>, haneul_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, haneul_amount: u64): u64 {
    // When either amount is 0, that means we have no stakes <b>with</b> this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.haneul_amount == 0 || exchange_rate.pool_token_amount == 0) {
        <b>return</b> haneul_amount
    };
    <b>let</b> res = exchange_rate.pool_token_amount <b>as</b> u128
            * (haneul_amount <b>as</b> u128)
            / (exchange_rate.haneul_amount <b>as</b> u128);
    res <b>as</b> u64
}
</code></pre>



</details>

<a name="0x3_staking_pool_initial_exchange_rate"></a>

## Function `initial_exchange_rate`



<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { haneul_amount: 0, pool_token_amount: 0 }
}
</code></pre>



</details>

<a name="0x3_staking_pool_check_balance_invariants"></a>

## Function `check_balance_invariants`



<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>, epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="staking_pool.md#0x3_staking_pool_check_balance_invariants">check_balance_invariants</a>(pool: &<a href="staking_pool.md#0x3_staking_pool_StakingPool">StakingPool</a>, epoch: u64) {
    <b>let</b> exchange_rate = <a href="staking_pool.md#0x3_staking_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, epoch);
    // check that the pool token <a href="../haneul-framework/balance.md#0x2_balance">balance</a> and <a href="../haneul-framework/haneul.md#0x2_haneul">haneul</a> <a href="../haneul-framework/balance.md#0x2_balance">balance</a> ratio matches the exchange rate stored.
    <b>let</b> expected = <a href="staking_pool.md#0x3_staking_pool_get_token_amount">get_token_amount</a>(&exchange_rate, pool.haneul_balance);
    <b>let</b> actual = pool.pool_token_balance;
    <b>assert</b>!(expected == actual, <a href="staking_pool.md#0x3_staking_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>)
}
</code></pre>



</details>
