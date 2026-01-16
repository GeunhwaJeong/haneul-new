---
title: Module `haneul::pay`
---

This module provides handy functionality for wallets and <code>haneul::Coin</code> management.


-  [Constants](#@Constants_0)
-  [Function `keep`](#haneul_pay_keep)
-  [Function `split`](#haneul_pay_split)
-  [Function `split_vec`](#haneul_pay_split_vec)
-  [Function `split_and_transfer`](#haneul_pay_split_and_transfer)
-  [Function `divide_and_keep`](#haneul_pay_divide_and_keep)
-  [Function `join`](#haneul_pay_join)
-  [Function `join_vec`](#haneul_pay_join_vec)
-  [Function `join_vec_and_transfer`](#haneul_pay_join_vec_and_transfer)


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
<b>use</b> <a href="../haneul/table.md#haneul_table">haneul::table</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/types.md#haneul_types">haneul::types</a>;
<b>use</b> <a href="../haneul/url.md#haneul_url">haneul::url</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../haneul/vec_set.md#haneul_vec_set">haneul::vec_set</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="haneul_pay_ENoCoins"></a>

For when empty vector is supplied into join function.


<pre><code><b>const</b> <a href="../haneul/pay.md#haneul_pay_ENoCoins">ENoCoins</a>: u64 = 0;
</code></pre>



<a name="haneul_pay_keep"></a>

## Function `keep`

Transfer <code>c</code> to the sender of the current transaction


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_keep">keep</a>&lt;T&gt;(c: <a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_keep">keep</a>&lt;T&gt;(c: Coin&lt;T&gt;, ctx: &TxContext) {
    <a href="../haneul/transfer.md#haneul_transfer_public_transfer">transfer::public_transfer</a>(c, ctx.sender())
}
</code></pre>



</details>

<a name="haneul_pay_split"></a>

## Function `split`

Split <code><a href="../haneul/coin.md#haneul_coin">coin</a></code> to two coins, one with balance <code>split_amount</code>,
and the remaining balance is left in <code><a href="../haneul/coin.md#haneul_coin">coin</a></code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_split">split</a>&lt;T&gt;(<a href="../haneul/coin.md#haneul_coin">coin</a>: &<b>mut</b> <a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_split">split</a>&lt;T&gt;(<a href="../haneul/coin.md#haneul_coin">coin</a>: &<b>mut</b> Coin&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    <a href="../haneul/pay.md#haneul_pay_keep">keep</a>(<a href="../haneul/coin.md#haneul_coin">coin</a>.<a href="../haneul/pay.md#haneul_pay_split">split</a>(split_amount, ctx), ctx)
}
</code></pre>



</details>

<a name="haneul_pay_split_vec"></a>

## Function `split_vec`

Split coin <code>self</code> into multiple coins, each with balance specified
in <code>split_amounts</code>. Remaining balance is left in <code>self</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_split_vec">split_vec</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;, split_amounts: vector&lt;u64&gt;, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_split_vec">split_vec</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, split_amounts: vector&lt;u64&gt;, ctx: &<b>mut</b> TxContext) {
    split_amounts.do!(|amount| <a href="../haneul/pay.md#haneul_pay_split">split</a>(self, amount, ctx));
}
</code></pre>



</details>

<a name="haneul_pay_split_and_transfer"></a>

## Function `split_and_transfer`

Send <code>amount</code> units of <code>c</code> to <code>recipient</code>
Aborts with <code><a href="../haneul/balance.md#haneul_balance_ENotEnough">haneul::balance::ENotEnough</a></code> if <code>amount</code> is greater than the balance in <code>c</code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_split_and_transfer">split_and_transfer</a>&lt;T&gt;(c: &<b>mut</b> <a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;, amount: u64, recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_split_and_transfer">split_and_transfer</a>&lt;T&gt;(
    c: &<b>mut</b> Coin&lt;T&gt;,
    amount: u64,
    recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../haneul/transfer.md#haneul_transfer_public_transfer">transfer::public_transfer</a>(c.<a href="../haneul/pay.md#haneul_pay_split">split</a>(amount, ctx), recipient)
}
</code></pre>



</details>

<a name="haneul_pay_divide_and_keep"></a>

## Function `divide_and_keep`

Divide coin <code>self</code> into <code>n - 1</code> coins with equal balances. If the balance is
not evenly divisible by <code>n</code>, the remainder is left in <code>self</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_divide_and_keep">divide_and_keep</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;, n: u64, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_divide_and_keep">divide_and_keep</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, n: u64, ctx: &<b>mut</b> TxContext) {
    self.divide_into_n(n, ctx).destroy!(|<a href="../haneul/coin.md#haneul_coin">coin</a>| <a href="../haneul/transfer.md#haneul_transfer_public_transfer">transfer::public_transfer</a>(<a href="../haneul/coin.md#haneul_coin">coin</a>, ctx.sender()));
}
</code></pre>



</details>

<a name="haneul_pay_join"></a>

## Function `join`

Join <code><a href="../haneul/coin.md#haneul_coin">coin</a></code> into <code>self</code>. Re-exports <code><a href="../haneul/coin.md#haneul_coin_join">coin::join</a></code> function.
Deprecated: you should call <code><a href="../haneul/coin.md#haneul_coin">coin</a>.<a href="../haneul/pay.md#haneul_pay_join">join</a>(other)</code> directly.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;, <a href="../haneul/coin.md#haneul_coin">coin</a>: <a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_join">join</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, <a href="../haneul/coin.md#haneul_coin">coin</a>: Coin&lt;T&gt;) {
    self.<a href="../haneul/pay.md#haneul_pay_join">join</a>(<a href="../haneul/coin.md#haneul_coin">coin</a>)
}
</code></pre>



</details>

<a name="haneul_pay_join_vec"></a>

## Function `join_vec`

Join everything in <code>coins</code> with <code>self</code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_join_vec">join_vec</a>&lt;T&gt;(self: &<b>mut</b> <a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;, coins: vector&lt;<a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_join_vec">join_vec</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, coins: vector&lt;Coin&lt;T&gt;&gt;) {
    coins.destroy!(|<a href="../haneul/coin.md#haneul_coin">coin</a>| self.<a href="../haneul/pay.md#haneul_pay_join">join</a>(<a href="../haneul/coin.md#haneul_coin">coin</a>));
}
</code></pre>



</details>

<a name="haneul_pay_join_vec_and_transfer"></a>

## Function `join_vec_and_transfer`

Join a vector of <code>Coin</code> into a single object and transfer it to <code>receiver</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_join_vec_and_transfer">join_vec_and_transfer</a>&lt;T&gt;(coins: vector&lt;<a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;&gt;, receiver: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/pay.md#haneul_pay_join_vec_and_transfer">join_vec_and_transfer</a>&lt;T&gt;(<b>mut</b> coins: vector&lt;Coin&lt;T&gt;&gt;, receiver: <b>address</b>) {
    <b>assert</b>!(coins.length() &gt; 0, <a href="../haneul/pay.md#haneul_pay_ENoCoins">ENoCoins</a>);
    <b>let</b> <b>mut</b> self = coins.pop_back();
    <a href="../haneul/pay.md#haneul_pay_join_vec">join_vec</a>(&<b>mut</b> self, coins);
    <a href="../haneul/transfer.md#haneul_transfer_public_transfer">transfer::public_transfer</a>(self, receiver)
}
</code></pre>



</details>
