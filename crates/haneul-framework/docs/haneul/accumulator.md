---
title: Module `haneul::accumulator`
---



-  [Struct `Accumulator`](#haneul_accumulator_Accumulator)
-  [Constants](#@Constants_0)
-  [Function `create`](#haneul_accumulator_create)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
</code></pre>



<a name="haneul_accumulator_Accumulator"></a>

## Struct `Accumulator`



<pre><code><b>public</b> <b>struct</b> <a href="../haneul/accumulator.md#haneul_accumulator_Accumulator">Accumulator</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_accumulator_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="../haneul/accumulator.md#haneul_accumulator_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="haneul_accumulator_create"></a>

## Function `create`



<pre><code><b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_create">create</a>(ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_create">create</a>(ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../haneul/accumulator.md#haneul_accumulator_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../haneul/transfer.md#haneul_transfer_share_object">transfer::share_object</a>(<a href="../haneul/accumulator.md#haneul_accumulator_Accumulator">Accumulator</a> {
        id: <a href="../haneul/object.md#haneul_object_haneul_accumulator_root_object_id">object::haneul_accumulator_root_object_id</a>(),
    })
}
</code></pre>



</details>
