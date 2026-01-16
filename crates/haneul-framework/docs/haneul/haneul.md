---
title: Module `haneul::haneul`
---

Coin<HANEUL> is the token used to pay for gas in Haneul.
It has 9 decimals, and the smallest unit (10^-9) is called "geunhwa".


-  [Struct `HANEUL`](#haneul_haneul_HANEUL)
-  [Constants](#@Constants_0)
-  [Function `new`](#haneul_haneul_new)
-  [Function `transfer`](#haneul_haneul_transfer)


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



<a name="haneul_haneul_HANEUL"></a>

## Struct `HANEUL`

Name of the coin


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/haneul.md#haneul_haneul_HANEUL">HANEUL</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_haneul_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="../haneul/haneul.md#haneul_haneul_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="haneul_haneul_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../haneul/haneul.md#haneul_haneul_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="haneul_haneul_GEUNHWA_PER_HANEUL"></a>

The amount of Geunhwa per Haneul token based on the fact that geunhwa is
10^-9 of a Haneul token


<pre><code><b>const</b> <a href="../haneul/haneul.md#haneul_haneul_GEUNHWA_PER_HANEUL">GEUNHWA_PER_HANEUL</a>: u64 = 1000000000;
</code></pre>



<a name="haneul_haneul_TOTAL_SUPPLY_HANEUL"></a>

The total supply of Haneul denominated in whole Haneul tokens (10 Billion)


<pre><code><b>const</b> <a href="../haneul/haneul.md#haneul_haneul_TOTAL_SUPPLY_HANEUL">TOTAL_SUPPLY_HANEUL</a>: u64 = 10000000000;
</code></pre>



<a name="haneul_haneul_TOTAL_SUPPLY_GEUNHWA"></a>

The total supply of Haneul denominated in Geunhwa (10 Billion * 10^9)


<pre><code><b>const</b> <a href="../haneul/haneul.md#haneul_haneul_TOTAL_SUPPLY_GEUNHWA">TOTAL_SUPPLY_GEUNHWA</a>: u64 = 10000000000000000000;
</code></pre>



<a name="haneul_haneul_new"></a>

## Function `new`

Register the <code><a href="../haneul/haneul.md#haneul_haneul_HANEUL">HANEUL</a></code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="../haneul/haneul.md#haneul_haneul_new">new</a>(ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/haneul.md#haneul_haneul_new">new</a>(ctx: &<b>mut</b> TxContext): Balance&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">HANEUL</a>&gt; {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../haneul/haneul.md#haneul_haneul_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(ctx.epoch() == 0, <a href="../haneul/haneul.md#haneul_haneul_EAlreadyMinted">EAlreadyMinted</a>);
    <b>let</b> (treasury, metadata) = <a href="../haneul/coin.md#haneul_coin_create_currency">coin::create_currency</a>(
        <a href="../haneul/haneul.md#haneul_haneul_HANEUL">HANEUL</a> {},
        9,
        b"<a href="../haneul/haneul.md#haneul_haneul_HANEUL">HANEUL</a>",
        b"Haneul",
        // TODO: add appropriate description and logo <a href="../haneul/url.md#haneul_url">url</a>
        b"",
        option::none(),
        ctx,
    );
    <a href="../haneul/transfer.md#haneul_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> <b>mut</b> supply = treasury.treasury_into_supply();
    <b>let</b> total_haneul = supply.increase_supply(<a href="../haneul/haneul.md#haneul_haneul_TOTAL_SUPPLY_GEUNHWA">TOTAL_SUPPLY_GEUNHWA</a>);
    supply.destroy_supply();
    total_haneul
}
</code></pre>



</details>

<a name="haneul_haneul_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/transfer.md#haneul_transfer">transfer</a>(c: <a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">haneul::haneul::HANEUL</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../haneul/transfer.md#haneul_transfer">transfer</a>(c: <a href="../haneul/coin.md#haneul_coin_Coin">coin::Coin</a>&lt;<a href="../haneul/haneul.md#haneul_haneul_HANEUL">HANEUL</a>&gt;, recipient: <b>address</b>) {
    <a href="../haneul/transfer.md#haneul_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>
