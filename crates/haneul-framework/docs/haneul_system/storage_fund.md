---
title: Module `haneul_system::storage_fund`
---



-  [Struct `StorageFund`](#haneul_system_storage_fund_StorageFund)
-  [Function `new`](#haneul_system_storage_fund_new)
-  [Function `advance_epoch`](#haneul_system_storage_fund_advance_epoch)
-  [Function `total_object_storage_rebates`](#haneul_system_storage_fund_total_object_storage_rebates)
-  [Function `total_balance`](#haneul_system_storage_fund_total_balance)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../haneul/accumulator.md#haneul_accumulator">haneul::accumulator</a>;
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
<b>use</b> <a href="../haneul/hash.md#haneul_hash">haneul::hash</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/protocol_config.md#haneul_protocol_config">haneul::protocol_config</a>;
<b>use</b> <a href="../haneul/haneul.md#haneul_haneul">haneul::haneul</a>;
<b>use</b> <a href="../haneul/table.md#haneul_table">haneul::table</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/types.md#haneul_types">haneul::types</a>;
<b>use</b> <a href="../haneul/url.md#haneul_url">haneul::url</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../haneul/vec_set.md#haneul_vec_set">haneul::vec_set</a>;
</code></pre>



<a name="haneul_system_storage_fund_StorageFund"></a>

## Struct `StorageFund`

Struct representing the storage fund, containing two <code>Balance</code>s:
- <code><a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a></code> has the invariant that it's the sum of <code>storage_rebate</code> of
all objects currently stored on-chain. To maintain this invariant, the only inflow of this
balance is storage charges collected from transactions, and the only outflow is storage rebates
of transactions, including both the portion refunded to the transaction senders as well as
the non-refundable portion taken out and put into <code>non_refundable_balance</code>.
- <code>non_refundable_balance</code> contains any remaining inflow of the storage fund that should not
be taken out of the fund.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_StorageFund">StorageFund</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>non_refundable_balance: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_system_storage_fund_new"></a>

## Function `new`

Called by <code><a href="../haneul_system/haneul_system.md#haneul_system_haneul_system">haneul_system</a></code> at genesis time.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_new">new</a>(initial_fund: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;): <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_StorageFund">haneul_system::storage_fund::StorageFund</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_new">new</a>(initial_fund: Balance&lt;HANEUL&gt;): <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_StorageFund">StorageFund</a> {
    <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_StorageFund">StorageFund</a> {
        // At the beginning there's no object in the storage yet
        <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>: balance::zero(),
        non_refundable_balance: initial_fund,
    }
}
</code></pre>



</details>

<a name="haneul_system_storage_fund_advance_epoch"></a>

## Function `advance_epoch`

Called by <code><a href="../haneul_system/haneul_system.md#haneul_system_haneul_system">haneul_system</a></code> at epoch change times to process the inflows and outflows of storage fund.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_StorageFund">haneul_system::storage_fund::StorageFund</a>, storage_charges: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;, storage_fund_reinvestment: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;, leftover_staking_rewards: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;, storage_rebate_amount: u64, non_refundable_storage_fee_amount: u64): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_StorageFund">StorageFund</a>,
    storage_charges: Balance&lt;HANEUL&gt;,
    storage_fund_reinvestment: Balance&lt;HANEUL&gt;,
    leftover_staking_rewards: Balance&lt;HANEUL&gt;,
    storage_rebate_amount: u64,
    non_refundable_storage_fee_amount: u64,
): Balance&lt;HANEUL&gt; {
    // Both the reinvestment and leftover rewards are not to be refunded so they go to the non-refundable balance.
    self.non_refundable_balance.join(storage_fund_reinvestment);
    self.non_refundable_balance.join(leftover_staking_rewards);
    // The storage charges <b>for</b> the epoch come from the storage rebate of the <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_new">new</a> objects created
    // and the <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_new">new</a> storage rebates of the objects modified during the epoch so we put the charges
    // into `<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>`.
    self.<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.join(storage_charges);
    // Split out the non-refundable portion of the storage rebate and put it into the non-refundable balance.
    <b>let</b> non_refundable_storage_fee = self
        .<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>
        .split(non_refundable_storage_fee_amount);
    self.non_refundable_balance.join(non_refundable_storage_fee);
    // `storage_rebates` include the already refunded rebates of deleted objects and old rebates of modified objects and
    // should be taken out of the `<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>`.
    <b>let</b> storage_rebate = self.<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.split(storage_rebate_amount);
    // The storage rebate <b>has</b> already been returned to individual transaction senders' gas coins
    // so we <b>return</b> the balance to be burnt at the very end of epoch change.
    storage_rebate
}
</code></pre>



</details>

<a name="haneul_system_storage_fund_total_object_storage_rebates"></a>

## Function `total_object_storage_rebates`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>(self: &<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_StorageFund">haneul_system::storage_fund::StorageFund</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>(self: &<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_StorageFund">StorageFund</a>): u64 {
    self.<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.value()
}
</code></pre>



</details>

<a name="haneul_system_storage_fund_total_balance"></a>

## Function `total_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_balance">total_balance</a>(self: &<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_StorageFund">haneul_system::storage_fund::StorageFund</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_balance">total_balance</a>(self: &<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_StorageFund">StorageFund</a>): u64 {
    self.<a href="../haneul_system/storage_fund.md#haneul_system_storage_fund_total_object_storage_rebates">total_object_storage_rebates</a>.value() + self.non_refundable_balance.value()
}
</code></pre>



</details>
