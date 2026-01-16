---
title: Module `haneul_system::validator_wrapper`
---



-  [Struct `ValidatorWrapper`](#haneul_system_validator_wrapper_ValidatorWrapper)
-  [Constants](#@Constants_0)
-  [Function `create_v1`](#haneul_system_validator_wrapper_create_v1)
-  [Function `load_validator_maybe_upgrade`](#haneul_system_validator_wrapper_load_validator_maybe_upgrade)
-  [Function `destroy`](#haneul_system_validator_wrapper_destroy)
-  [Function `upgrade_to_latest`](#haneul_system_validator_wrapper_upgrade_to_latest)
-  [Function `version`](#haneul_system_validator_wrapper_version)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
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
<b>use</b> <a href="../haneul/versioned.md#haneul_versioned">haneul::versioned</a>;
<b>use</b> <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">haneul_system::staking_pool</a>;
<b>use</b> <a href="../haneul_system/validator.md#haneul_system_validator">haneul_system::validator</a>;
<b>use</b> <a href="../haneul_system/validator_cap.md#haneul_system_validator_cap">haneul_system::validator_cap</a>;
</code></pre>



<a name="haneul_system_validator_wrapper_ValidatorWrapper"></a>

## Struct `ValidatorWrapper`



<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: <a href="../haneul/versioned.md#haneul_versioned_Versioned">haneul::versioned::Versioned</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_system_validator_wrapper_EInvalidVersion"></a>



<pre><code><b>const</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_EInvalidVersion">EInvalidVersion</a>: u64 = 0;
</code></pre>



<a name="haneul_system_validator_wrapper_create_v1"></a>

## Function `create_v1`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_create_v1">create_v1</a>(<a href="../haneul_system/validator.md#haneul_system_validator">validator</a>: <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">haneul_system::validator_wrapper::ValidatorWrapper</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_create_v1">create_v1</a>(<a href="../haneul_system/validator.md#haneul_system_validator">validator</a>: Validator, ctx: &<b>mut</b> TxContext): <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> {
    <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> {
        inner: versioned::create(1, <a href="../haneul_system/validator.md#haneul_system_validator">validator</a>, ctx),
    }
}
</code></pre>



</details>

<a name="haneul_system_validator_wrapper_load_validator_maybe_upgrade"></a>

## Function `load_validator_maybe_upgrade`

This function should always return the latest supported version.
If the inner version is old, we upgrade it lazily in-place.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">haneul_system::validator_wrapper::ValidatorWrapper</a>): &<b>mut</b> <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): &<b>mut</b> Validator {
    self.<a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>();
    self.inner.load_value_mut()
}
</code></pre>



</details>

<a name="haneul_system_validator_wrapper_destroy"></a>

## Function `destroy`

Destroy the wrapper and retrieve the inner validator object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_destroy">destroy</a>(self: <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">haneul_system::validator_wrapper::ValidatorWrapper</a>): <a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_destroy">destroy</a>(self: <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): Validator {
    <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(&self);
    <b>let</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> { inner } = self;
    inner.<a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_destroy">destroy</a>()
}
</code></pre>



</details>

<a name="haneul_system_validator_wrapper_upgrade_to_latest"></a>

## Function `upgrade_to_latest`



<pre><code><b>fun</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">haneul_system::validator_wrapper::ValidatorWrapper</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>) {
    <b>let</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_version">version</a> = self.<a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_version">version</a>();
    // TODO: When new versions are added, we need to explicitly upgrade here.
    <b>assert</b>!(<a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_version">version</a> == 1, <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_EInvalidVersion">EInvalidVersion</a>);
}
</code></pre>



</details>

<a name="haneul_system_validator_wrapper_version"></a>

## Function `version`



<pre><code><b>fun</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_version">version</a>(self: &<a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">haneul_system::validator_wrapper::ValidatorWrapper</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_version">version</a>(self: &<a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): u64 {
    self.inner.<a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper_version">version</a>()
}
</code></pre>



</details>
