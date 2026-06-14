---
title: Module `haneul_system::stake_subsidy`
---



-  [Struct `StakeSubsidy`](#haneul_system_stake_subsidy_StakeSubsidy)
-  [Constants](#@Constants_0)
-  [Function `create`](#haneul_system_stake_subsidy_create)
-  [Function `advance_epoch`](#haneul_system_stake_subsidy_advance_epoch)
-  [Function `current_epoch_subsidy_amount`](#haneul_system_stake_subsidy_current_epoch_subsidy_amount)
-  [Function `get_distribution_counter`](#haneul_system_stake_subsidy_get_distribution_counter)


<pre><code><b>use</b> <a href="../haneul/accumulator.md#haneul_accumulator">haneul::accumulator</a>;
<b>use</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement">haneul::accumulator_settlement</a>;
<b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/bag.md#haneul_bag">haneul::bag</a>;
<b>use</b> <a href="../haneul/balance.md#haneul_balance">haneul::balance</a>;
<b>use</b> <a href="../haneul/bcs.md#haneul_bcs">haneul::bcs</a>;
<b>use</b> <a href="../haneul/coin.md#haneul_coin">haneul::coin</a>;
<b>use</b> <a href="../haneul/config.md#haneul_config">haneul::config</a>;
<b>use</b> <a href="../haneul/deny_list.md#haneul_deny_list">haneul::deny_list</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/dynamic_object_field.md#haneul_dynamic_object_field">haneul::dynamic_object_field</a>;
<b>use</b> <a href="../haneul/event.md#haneul_event">haneul::event</a>;
<b>use</b> <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator">haneul::funds_accumulator</a>;
<b>use</b> <a href="../haneul/haneul.md#haneul_haneul">haneul::haneul</a>;
<b>use</b> <a href="../haneul/hash.md#haneul_hash">haneul::hash</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/protocol_config.md#haneul_protocol_config">haneul::protocol_config</a>;
<b>use</b> <a href="../haneul/table.md#haneul_table">haneul::table</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/types.md#haneul_types">haneul::types</a>;
<b>use</b> <a href="../haneul/url.md#haneul_url">haneul::url</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../haneul/vec_set.md#haneul_vec_set">haneul::vec_set</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="haneul_system_stake_subsidy_StakeSubsidy"></a>

## Struct `StakeSubsidy`



<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;</code>
</dt>
<dd>
 Balance of HANEUL set aside for stake subsidies that will be drawn down over time.
</dd>
<dt>
<code>distribution_counter: u64</code>
</dt>
<dd>
 Count of the number of times stake subsidies have been distributed.
</dd>
<dt>
<code>current_distribution_amount: u64</code>
</dt>
<dd>
 The amount of stake subsidy to be drawn down per distribution.
 This amount decays and decreases over time.
</dd>
<dt>
<code>stake_subsidy_period_length: u64</code>
</dt>
<dd>
 Number of distributions to occur before the distribution amount decays.
</dd>
<dt>
<code>stake_subsidy_decrease_rate: u16</code>
</dt>
<dd>
 The rate at which the distribution amount decays at the end of each
 period. Expressed in basis points.
</dd>
<dt>
<code>extra_fields: <a href="../haneul/bag.md#haneul_bag_Bag">haneul::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_system_stake_subsidy_ESubsidyDecreaseRateTooLarge"></a>



<pre><code><b>const</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>: u64 = 0;
</code></pre>



<a name="haneul_system_stake_subsidy_BASIS_POINT_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>: u128 = 10000;
</code></pre>



<a name="haneul_system_stake_subsidy_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_create">create</a>(balance: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;, initial_distribution_amount: u64, stake_subsidy_period_length: u64, stake_subsidy_decrease_rate: u16, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_StakeSubsidy">haneul_system::stake_subsidy::StakeSubsidy</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_create">create</a>(
    balance: Balance&lt;HANEUL&gt;,
    initial_distribution_amount: u64,
    stake_subsidy_period_length: u64,
    stake_subsidy_decrease_rate: u16,
    ctx: &<b>mut</b> TxContext,
): <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a> {
    // Rate can't be higher than 100%.
    <b>assert</b>!(
        stake_subsidy_decrease_rate &lt;= <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a> <b>as</b> u16,
        <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>,
    );
    <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a> {
        balance,
        distribution_counter: 0,
        current_distribution_amount: initial_distribution_amount,
        stake_subsidy_period_length,
        stake_subsidy_decrease_rate,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="haneul_system_stake_subsidy_advance_epoch"></a>

## Function `advance_epoch`

Advance the epoch counter and draw down the subsidy for the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_StakeSubsidy">haneul_system::stake_subsidy::StakeSubsidy</a>): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>): Balance&lt;HANEUL&gt; {
    // Take the minimum of the reward amount and the remaining balance in
    // order to ensure we don't overdraft the remaining stake subsidy
    // balance
    <b>let</b> to_withdraw = self.current_distribution_amount.min(self.balance.value());
    // Drawn down the subsidy <b>for</b> this epoch.
    <b>let</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy">stake_subsidy</a> = self.balance.split(to_withdraw);
    self.distribution_counter = self.distribution_counter + 1;
    // Decrease the subsidy amount only when the current period ends.
    <b>if</b> (self.distribution_counter % self.stake_subsidy_period_length == 0) {
        <b>let</b> decrease_amount =
            self.current_distribution_amount <b>as</b> u128
            * (self.stake_subsidy_decrease_rate <b>as</b> u128) / <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
        self.current_distribution_amount =
            self.current_distribution_amount - (decrease_amount <b>as</b> u64)
    };
    <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy">stake_subsidy</a>
}
</code></pre>



</details>

<a name="haneul_system_stake_subsidy_current_epoch_subsidy_amount"></a>

## Function `current_epoch_subsidy_amount`

Returns the amount of stake subsidy to be added at the end of the current epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_current_epoch_subsidy_amount">current_epoch_subsidy_amount</a>(self: &<a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_StakeSubsidy">haneul_system::stake_subsidy::StakeSubsidy</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_current_epoch_subsidy_amount">current_epoch_subsidy_amount</a>(self: &<a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>): u64 {
    self.current_distribution_amount.min(self.balance.value())
}
</code></pre>



</details>

<a name="haneul_system_stake_subsidy_get_distribution_counter"></a>

## Function `get_distribution_counter`

Returns the number of distributions that have occurred.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_get_distribution_counter">get_distribution_counter</a>(self: &<a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_StakeSubsidy">haneul_system::stake_subsidy::StakeSubsidy</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_get_distribution_counter">get_distribution_counter</a>(self: &<a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>): u64 {
    self.distribution_counter
}
</code></pre>



</details>
