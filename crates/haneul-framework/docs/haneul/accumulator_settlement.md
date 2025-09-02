---
title: Module `haneul::accumulator_settlement`
---



-  [Constants](#@Constants_0)
-  [Function `settlement_prologue`](#haneul_accumulator_settlement_settlement_prologue)
-  [Function `settle_u128`](#haneul_accumulator_settlement_settle_u128)
-  [Function `record_settlement_haneul_conservation`](#haneul_accumulator_settlement_record_settlement_haneul_conservation)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../haneul/accumulator.md#haneul_accumulator">haneul::accumulator</a>;
<b>use</b> <a href="../haneul/accumulator_metadata.md#haneul_accumulator_metadata">haneul::accumulator_metadata</a>;
<b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/bag.md#haneul_bag">haneul::bag</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="haneul_accumulator_settlement_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="haneul_accumulator_settlement_EInvalidSplitAmount"></a>



<pre><code><b>const</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_EInvalidSplitAmount">EInvalidSplitAmount</a>: u64 = 1;
</code></pre>



<a name="haneul_accumulator_settlement_settlement_prologue"></a>

## Function `settlement_prologue`

Called by settlement transactions to ensure that the settlement transaction has a unique
digest.


<pre><code><b>fun</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_settlement_prologue">settlement_prologue</a>(_epoch: u64, _checkpoint_height: u64, _idx: u64, input_haneul: u64, output_haneul: u64, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_settlement_prologue">settlement_prologue</a>(
    _epoch: u64,
    _checkpoint_height: u64,
    _idx: u64,
    // Total input <a href="../haneul/haneul.md#haneul_haneul">haneul</a> received from user transactions
    input_haneul: u64,
    // Total output <a href="../haneul/haneul.md#haneul_haneul">haneul</a> withdrawn by user transactions
    output_haneul: u64,
    ctx: &TxContext,
) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_record_settlement_haneul_conservation">record_settlement_haneul_conservation</a>(input_haneul, output_haneul);
}
</code></pre>



</details>

<a name="haneul_accumulator_settlement_settle_u128"></a>

## Function `settle_u128`



<pre><code><b>fun</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_settle_u128">settle_u128</a>&lt;T&gt;(accumulator_root: &<b>mut</b> <a href="../haneul/accumulator.md#haneul_accumulator_AccumulatorRoot">haneul::accumulator::AccumulatorRoot</a>, owner: <b>address</b>, merge: u128, split: u128, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_settle_u128">settle_u128</a>&lt;T&gt;(
    accumulator_root: &<b>mut</b> AccumulatorRoot,
    owner: <b>address</b>,
    merge: u128,
    split: u128,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_ENotSystemAddress">ENotSystemAddress</a>);
    // Merge and split should be netted out prior to calling this function.
    <b>assert</b>!((merge == 0 ) != (split == 0), <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_EInvalidSplitAmount">EInvalidSplitAmount</a>);
    <b>let</b> name = accumulator_key&lt;T&gt;(owner);
    <b>if</b> (accumulator_root.has_accumulator&lt;T, U128&gt;(name)) {
        <b>let</b> is_zero = {
            <b>let</b> value: &<b>mut</b> U128 = accumulator_root.borrow_accumulator_mut(name);
            value.update(merge, split);
            value.is_zero()
        };
        <b>if</b> (is_zero) {
            <b>let</b> value = accumulator_root.remove_accumulator&lt;T, U128&gt;(name);
            destroy_u128(value);
            accumulator_root.remove_metadata&lt;T&gt;(owner);
        }
    } <b>else</b> {
        // cannot split <b>if</b> the field does not yet exist
        <b>assert</b>!(split == 0, <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_EInvalidSplitAmount">EInvalidSplitAmount</a>);
        <b>let</b> value = create_u128(merge);
        accumulator_root.add_accumulator(name, value);
        accumulator_root.create_metadata&lt;T&gt;(owner, ctx);
    };
}
</code></pre>



</details>

<a name="haneul_accumulator_settlement_record_settlement_haneul_conservation"></a>

## Function `record_settlement_haneul_conservation`

Called by the settlement transaction to track conservation of HANEUL.


<pre><code><b>fun</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_record_settlement_haneul_conservation">record_settlement_haneul_conservation</a>(input_haneul: u64, output_haneul: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement_record_settlement_haneul_conservation">record_settlement_haneul_conservation</a>(input_haneul: u64, output_haneul: u64);
</code></pre>



</details>
