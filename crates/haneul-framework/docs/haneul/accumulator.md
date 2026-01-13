---
title: Module `haneul::accumulator`
---



-  [Struct `AccumulatorRoot`](#haneul_accumulator_AccumulatorRoot)
-  [Struct `U128`](#haneul_accumulator_U128)
-  [Struct `Key`](#haneul_accumulator_Key)
-  [Constants](#@Constants_0)
-  [Function `create`](#haneul_accumulator_create)
-  [Function `root_id`](#haneul_accumulator_root_id)
-  [Function `root_id_mut`](#haneul_accumulator_root_id_mut)
-  [Function `accumulator_u128_exists`](#haneul_accumulator_accumulator_u128_exists)
-  [Function `accumulator_u128_read`](#haneul_accumulator_accumulator_u128_read)
-  [Function `create_u128`](#haneul_accumulator_create_u128)
-  [Function `destroy_u128`](#haneul_accumulator_destroy_u128)
-  [Function `update_u128`](#haneul_accumulator_update_u128)
-  [Function `is_zero_u128`](#haneul_accumulator_is_zero_u128)
-  [Function `accumulator_key`](#haneul_accumulator_accumulator_key)
-  [Function `accumulator_address`](#haneul_accumulator_accumulator_address)
-  [Function `root_has_accumulator`](#haneul_accumulator_root_has_accumulator)
-  [Function `root_add_accumulator`](#haneul_accumulator_root_add_accumulator)
-  [Function `root_borrow_accumulator_mut`](#haneul_accumulator_root_borrow_accumulator_mut)
-  [Function `root_borrow_accumulator`](#haneul_accumulator_root_borrow_accumulator)
-  [Function `root_remove_accumulator`](#haneul_accumulator_root_remove_accumulator)
-  [Function `emit_deposit_event`](#haneul_accumulator_emit_deposit_event)
-  [Function `emit_withdraw_event`](#haneul_accumulator_emit_withdraw_event)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
</code></pre>



<a name="haneul_accumulator_AccumulatorRoot"></a>

## Struct `AccumulatorRoot`



<pre><code><b>public</b> <b>struct</b> <a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a> <b>has</b> key
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

<a name="haneul_accumulator_U128"></a>

## Struct `U128`

Storage for 128-bit accumulator values.

Currently only used to represent the sum of 64 bit values (such as <code>Balance&lt;T&gt;</code>).
The additional bits are necessary to prevent overflow, as it would take 2^64 deposits of U64_MAX
to cause an overflow.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/accumulator.md#haneul_accumulator_U128">U128</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>value: u128</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_accumulator_Key"></a>

## Struct `Key`

<code><a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a></code> is used only for computing the field id of accumulator objects.
<code>T</code> is the type of the accumulated value, e.g. <code>Balance&lt;HANEUL&gt;</code>


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><b>address</b>: <b>address</b></code>
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
    <a href="../haneul/transfer.md#haneul_transfer_share_object">transfer::share_object</a>(<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a> {
        id: <a href="../haneul/object.md#haneul_object_haneul_accumulator_root_object_id">object::haneul_accumulator_root_object_id</a>(),
    })
}
</code></pre>



</details>

<a name="haneul_accumulator_root_id"></a>

## Function `root_id`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_id">root_id</a>(accumulator_root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>): &<a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_id">root_id</a>(accumulator_root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a>): &UID {
    &accumulator_root.id
}
</code></pre>



</details>

<a name="haneul_accumulator_root_id_mut"></a>

## Function `root_id_mut`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_id_mut">root_id_mut</a>(accumulator_root: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>): &<b>mut</b> <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_id_mut">root_id_mut</a>(accumulator_root: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a>): &<b>mut</b> UID {
    &<b>mut</b> accumulator_root.id
}
</code></pre>



</details>

<a name="haneul_accumulator_accumulator_u128_exists"></a>

## Function `accumulator_u128_exists`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_accumulator_u128_exists">accumulator_u128_exists</a>&lt;T&gt;(root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>, <b>address</b>: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_accumulator_u128_exists">accumulator_u128_exists</a>&lt;T&gt;(root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a>, <b>address</b>: <b>address</b>): bool {
    root.has_accumulator&lt;T, <a href="../haneul/accumulator.md#haneul_accumulator_U128">U128</a>&gt;(<a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;T&gt; { <b>address</b> })
}
</code></pre>



</details>

<a name="haneul_accumulator_accumulator_u128_read"></a>

## Function `accumulator_u128_read`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_accumulator_u128_read">accumulator_u128_read</a>&lt;T&gt;(root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>, <b>address</b>: <b>address</b>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_accumulator_u128_read">accumulator_u128_read</a>&lt;T&gt;(root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a>, <b>address</b>: <b>address</b>): u128 {
    <b>let</b> <a href="../haneul/accumulator.md#haneul_accumulator">accumulator</a> = root.borrow_accumulator&lt;T, <a href="../haneul/accumulator.md#haneul_accumulator_U128">U128</a>&gt;(<a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;T&gt; { <b>address</b> });
    <a href="../haneul/accumulator.md#haneul_accumulator">accumulator</a>.value
}
</code></pre>



</details>

<a name="haneul_accumulator_create_u128"></a>

## Function `create_u128`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_create_u128">create_u128</a>(value: u128): <a href="../haneul/accumulator.md#haneul_accumulator_U128">haneul::accumulator::U128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_create_u128">create_u128</a>(value: u128): <a href="../haneul/accumulator.md#haneul_accumulator_U128">U128</a> {
    <a href="../haneul/accumulator.md#haneul_accumulator_U128">U128</a> { value }
}
</code></pre>



</details>

<a name="haneul_accumulator_destroy_u128"></a>

## Function `destroy_u128`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_destroy_u128">destroy_u128</a>(u128: <a href="../haneul/accumulator.md#haneul_accumulator_U128">haneul::accumulator::U128</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_destroy_u128">destroy_u128</a>(u128: <a href="../haneul/accumulator.md#haneul_accumulator_U128">U128</a>) {
    <b>let</b> <a href="../haneul/accumulator.md#haneul_accumulator_U128">U128</a> { value: _ } = u128;
}
</code></pre>



</details>

<a name="haneul_accumulator_update_u128"></a>

## Function `update_u128`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_update_u128">update_u128</a>(u128: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_U128">haneul::accumulator::U128</a>, merge: u128, split: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_update_u128">update_u128</a>(u128: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_U128">U128</a>, merge: u128, split: u128) {
    u128.value = u128.value + merge - split;
}
</code></pre>



</details>

<a name="haneul_accumulator_is_zero_u128"></a>

## Function `is_zero_u128`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_is_zero_u128">is_zero_u128</a>(u128: &<a href="../haneul/accumulator.md#haneul_accumulator_U128">haneul::accumulator::U128</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_is_zero_u128">is_zero_u128</a>(u128: &<a href="../haneul/accumulator.md#haneul_accumulator_U128">U128</a>): bool {
    u128.value == 0
}
</code></pre>



</details>

<a name="haneul_accumulator_accumulator_key"></a>

## Function `accumulator_key`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_accumulator_key">accumulator_key</a>&lt;T&gt;(<b>address</b>: <b>address</b>): <a href="../haneul/accumulator.md#haneul_accumulator_Key">haneul::accumulator::Key</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_accumulator_key">accumulator_key</a>&lt;T&gt;(<b>address</b>: <b>address</b>): <a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;T&gt; {
    <a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a> { <b>address</b> }
}
</code></pre>



</details>

<a name="haneul_accumulator_accumulator_address"></a>

## Function `accumulator_address`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_accumulator_address">accumulator_address</a>&lt;T&gt;(<b>address</b>: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_accumulator_address">accumulator_address</a>&lt;T&gt;(<b>address</b>: <b>address</b>): <b>address</b> {
    <b>let</b> key = <a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;T&gt; { <b>address</b> };
    <a href="../haneul/dynamic_field.md#haneul_dynamic_field_hash_type_and_key">dynamic_field::hash_type_and_key</a>(haneul_accumulator_root_address(), key)
}
</code></pre>



</details>

<a name="haneul_accumulator_root_has_accumulator"></a>

## Function `root_has_accumulator`

Balance object methods


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_has_accumulator">root_has_accumulator</a>&lt;K, V: store&gt;(accumulator_root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>, name: <a href="../haneul/accumulator.md#haneul_accumulator_Key">haneul::accumulator::Key</a>&lt;K&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_has_accumulator">root_has_accumulator</a>&lt;K, V: store&gt;(
    accumulator_root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a>,
    name: <a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;K&gt;,
): bool {
    <a href="../haneul/dynamic_field.md#haneul_dynamic_field_exists_with_type">dynamic_field::exists_with_type</a>&lt;<a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;K&gt;, V&gt;(&accumulator_root.id, name)
}
</code></pre>



</details>

<a name="haneul_accumulator_root_add_accumulator"></a>

## Function `root_add_accumulator`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_add_accumulator">root_add_accumulator</a>&lt;K, V: store&gt;(accumulator_root: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>, name: <a href="../haneul/accumulator.md#haneul_accumulator_Key">haneul::accumulator::Key</a>&lt;K&gt;, value: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_add_accumulator">root_add_accumulator</a>&lt;K, V: store&gt;(
    accumulator_root: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a>,
    name: <a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;K&gt;,
    value: V,
) {
    <a href="../haneul/dynamic_field.md#haneul_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> accumulator_root.id, name, value);
}
</code></pre>



</details>

<a name="haneul_accumulator_root_borrow_accumulator_mut"></a>

## Function `root_borrow_accumulator_mut`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_borrow_accumulator_mut">root_borrow_accumulator_mut</a>&lt;K, V: store&gt;(accumulator_root: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>, name: <a href="../haneul/accumulator.md#haneul_accumulator_Key">haneul::accumulator::Key</a>&lt;K&gt;): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_borrow_accumulator_mut">root_borrow_accumulator_mut</a>&lt;K, V: store&gt;(
    accumulator_root: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a>,
    name: <a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;K&gt;,
): &<b>mut</b> V {
    <a href="../haneul/dynamic_field.md#haneul_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>&lt;<a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;K&gt;, V&gt;(&<b>mut</b> accumulator_root.id, name)
}
</code></pre>



</details>

<a name="haneul_accumulator_root_borrow_accumulator"></a>

## Function `root_borrow_accumulator`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_borrow_accumulator">root_borrow_accumulator</a>&lt;K, V: store&gt;(accumulator_root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>, name: <a href="../haneul/accumulator.md#haneul_accumulator_Key">haneul::accumulator::Key</a>&lt;K&gt;): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_borrow_accumulator">root_borrow_accumulator</a>&lt;K, V: store&gt;(
    accumulator_root: &<a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a>,
    name: <a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;K&gt;,
): &V {
    <a href="../haneul/dynamic_field.md#haneul_dynamic_field_borrow">dynamic_field::borrow</a>&lt;<a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;K&gt;, V&gt;(&accumulator_root.id, name)
}
</code></pre>



</details>

<a name="haneul_accumulator_root_remove_accumulator"></a>

## Function `root_remove_accumulator`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_remove_accumulator">root_remove_accumulator</a>&lt;K, V: store&gt;(accumulator_root: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>, name: <a href="../haneul/accumulator.md#haneul_accumulator_Key">haneul::accumulator::Key</a>&lt;K&gt;): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_root_remove_accumulator">root_remove_accumulator</a>&lt;K, V: store&gt;(
    accumulator_root: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">AccumulatorRoot</a>,
    name: <a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;K&gt;,
): V {
    <a href="../haneul/dynamic_field.md#haneul_dynamic_field_remove">dynamic_field::remove</a>&lt;<a href="../haneul/accumulator.md#haneul_accumulator_Key">Key</a>&lt;K&gt;, V&gt;(&<b>mut</b> accumulator_root.id, name)
}
</code></pre>



</details>

<a name="haneul_accumulator_emit_deposit_event"></a>

## Function `emit_deposit_event`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_emit_deposit_event">emit_deposit_event</a>&lt;T&gt;(<a href="../haneul/accumulator.md#haneul_accumulator">accumulator</a>: <b>address</b>, recipient: <b>address</b>, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>native</b> <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_emit_deposit_event">emit_deposit_event</a>&lt;T&gt;(
    <a href="../haneul/accumulator.md#haneul_accumulator">accumulator</a>: <b>address</b>,
    recipient: <b>address</b>,
    amount: u64,
);
</code></pre>



</details>

<a name="haneul_accumulator_emit_withdraw_event"></a>

## Function `emit_withdraw_event`



<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_emit_withdraw_event">emit_withdraw_event</a>&lt;T&gt;(<a href="../haneul/accumulator.md#haneul_accumulator">accumulator</a>: <b>address</b>, owner: <b>address</b>, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../haneul/package.md#haneul_package">package</a>) <b>native</b> <b>fun</b> <a href="../haneul/accumulator.md#haneul_accumulator_emit_withdraw_event">emit_withdraw_event</a>&lt;T&gt;(
    <a href="../haneul/accumulator.md#haneul_accumulator">accumulator</a>: <b>address</b>,
    owner: <b>address</b>,
    amount: u64,
);
</code></pre>



</details>
