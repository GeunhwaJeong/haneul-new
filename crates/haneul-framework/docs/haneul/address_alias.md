---
title: Module `haneul::address_alias`
---



-  [Struct `AddressAliasState`](#haneul_address_alias_AddressAliasState)
-  [Struct `AddressAliases`](#haneul_address_alias_AddressAliases)
-  [Struct `AliasKey`](#haneul_address_alias_AliasKey)
-  [Constants](#@Constants_0)
-  [Function `create`](#haneul_address_alias_create)
-  [Function `enable`](#haneul_address_alias_enable)
-  [Function `add`](#haneul_address_alias_add)
-  [Function `replace_all`](#haneul_address_alias_replace_all)
-  [Function `remove`](#haneul_address_alias_remove)


<pre><code><b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/derived_object.md#haneul_derived_object">haneul::derived_object</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../haneul/vec_set.md#haneul_vec_set">haneul::vec_set</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="haneul_address_alias_AddressAliasState"></a>

## Struct `AddressAliasState`

Singleton shared object which manages creation of AddressAliases state.
The actual alias configs are created as derived objects with this object
as the parent.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliasState">AddressAliasState</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_address_alias_AddressAliases"></a>

## Struct `AddressAliases`

Tracks the set of addresses allowed to act as a given sender.

An alias allows transactions signed by the alias address to act as the
original address. For example, if address X sets an alias of address Y, then
then a transaction signed by Y can set its sender address to X.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliases">AddressAliases</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../haneul/object.md#haneul_object_UID">haneul::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>aliases: <a href="../haneul/vec_set.md#haneul_vec_set_VecSet">haneul::vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_address_alias_AliasKey"></a>

## Struct `AliasKey`

Internal key used for derivation of AddressAliases object addresses.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/address_alias.md#haneul_address_alias_AliasKey">AliasKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>0: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_address_alias_ENotSystemAddress"></a>



<pre><code>#[error]
<b>const</b> <a href="../haneul/address_alias.md#haneul_address_alias_ENotSystemAddress">ENotSystemAddress</a>: vector&lt;u8&gt; = b"Only the system can <a href="../haneul/address_alias.md#haneul_address_alias_create">create</a> the alias state <a href="../haneul/object.md#haneul_object">object</a>.";
</code></pre>



<a name="haneul_address_alias_ENoSuchAlias"></a>



<pre><code>#[error]
<b>const</b> <a href="../haneul/address_alias.md#haneul_address_alias_ENoSuchAlias">ENoSuchAlias</a>: vector&lt;u8&gt; = b"Given alias does not exist.";
</code></pre>



<a name="haneul_address_alias_EAliasAlreadyExists"></a>



<pre><code>#[error]
<b>const</b> <a href="../haneul/address_alias.md#haneul_address_alias_EAliasAlreadyExists">EAliasAlreadyExists</a>: vector&lt;u8&gt; = b"Alias already exists.";
</code></pre>



<a name="haneul_address_alias_ECannotRemoveLastAlias"></a>



<pre><code>#[error]
<b>const</b> <a href="../haneul/address_alias.md#haneul_address_alias_ECannotRemoveLastAlias">ECannotRemoveLastAlias</a>: vector&lt;u8&gt; = b"Cannot <a href="../haneul/address_alias.md#haneul_address_alias_remove">remove</a> the last alias.";
</code></pre>



<a name="haneul_address_alias_ETooManyAliases"></a>



<pre><code>#[error]
<b>const</b> <a href="../haneul/address_alias.md#haneul_address_alias_ETooManyAliases">ETooManyAliases</a>: vector&lt;u8&gt; = b"The number of aliases exceeds the maximum allowed.";
</code></pre>



<a name="haneul_address_alias_CURRENT_VERSION"></a>



<pre><code><b>const</b> <a href="../haneul/address_alias.md#haneul_address_alias_CURRENT_VERSION">CURRENT_VERSION</a>: u64 = 0;
</code></pre>



<a name="haneul_address_alias_MAX_ALIASES"></a>



<pre><code><b>const</b> <a href="../haneul/address_alias.md#haneul_address_alias_MAX_ALIASES">MAX_ALIASES</a>: u64 = 8;
</code></pre>



<a name="haneul_address_alias_create"></a>

## Function `create`

Create and share the AddressAliasState object. This function is called exactly once, when
the address alias state object is first created.
Can only be called by genesis or change_epoch transactions.


<pre><code><b>fun</b> <a href="../haneul/address_alias.md#haneul_address_alias_create">create</a>(ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/address_alias.md#haneul_address_alias_create">create</a>(ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../haneul/address_alias.md#haneul_address_alias_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> self = <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliasState">AddressAliasState</a> {
        id: <a href="../haneul/object.md#haneul_object_address_alias_state">object::address_alias_state</a>(),
        version: <a href="../haneul/address_alias.md#haneul_address_alias_CURRENT_VERSION">CURRENT_VERSION</a>,
    };
    <a href="../haneul/transfer.md#haneul_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="haneul_address_alias_enable"></a>

## Function `enable`

Enables address alias configuration for the sender address.

By default, an address is its own alias. The provided <code><a href="../haneul/address_alias.md#haneul_address_alias_AddressAliases">AddressAliases</a></code>
object can be used to change the set of allowed aliases after enabling.


<pre><code><b>entry</b> <b>fun</b> <a href="../haneul/address_alias.md#haneul_address_alias_enable">enable</a>(address_alias_state: &<b>mut</b> <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliasState">haneul::address_alias::AddressAliasState</a>, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>fun</b> <a href="../haneul/address_alias.md#haneul_address_alias_enable">enable</a>(address_alias_state: &<b>mut</b> <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliasState">AddressAliasState</a>, ctx: &TxContext) {
    <b>assert</b>!(
        !<a href="../haneul/derived_object.md#haneul_derived_object_exists">derived_object::exists</a>(&address_alias_state.id, <a href="../haneul/address_alias.md#haneul_address_alias_AliasKey">AliasKey</a>(ctx.sender())),
        <a href="../haneul/address_alias.md#haneul_address_alias_EAliasAlreadyExists">EAliasAlreadyExists</a>,
    );
    <a href="../haneul/transfer.md#haneul_transfer_party_transfer">transfer::party_transfer</a>(
        <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliases">AddressAliases</a> {
            id: <a href="../haneul/derived_object.md#haneul_derived_object_claim">derived_object::claim</a>(&<b>mut</b> address_alias_state.id, <a href="../haneul/address_alias.md#haneul_address_alias_AliasKey">AliasKey</a>(ctx.sender())),
            aliases: <a href="../haneul/vec_set.md#haneul_vec_set_singleton">vec_set::singleton</a>(ctx.sender()),
        },
        <a href="../haneul/party.md#haneul_party_single_owner">party::single_owner</a>(ctx.sender()),
    );
}
</code></pre>



</details>

<a name="haneul_address_alias_add"></a>

## Function `add`

Adds the provided address to the set of aliases for the sender.


<pre><code><b>entry</b> <b>fun</b> <a href="../haneul/address_alias.md#haneul_address_alias_add">add</a>(aliases: &<b>mut</b> <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliases">haneul::address_alias::AddressAliases</a>, alias: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>fun</b> <a href="../haneul/address_alias.md#haneul_address_alias_add">add</a>(aliases: &<b>mut</b> <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliases">AddressAliases</a>, alias: <b>address</b>) {
    <b>assert</b>!(!aliases.aliases.contains(&alias), <a href="../haneul/address_alias.md#haneul_address_alias_EAliasAlreadyExists">EAliasAlreadyExists</a>);
    aliases.aliases.insert(alias);
    <b>assert</b>!(aliases.aliases.length() &lt;= <a href="../haneul/address_alias.md#haneul_address_alias_MAX_ALIASES">MAX_ALIASES</a>, <a href="../haneul/address_alias.md#haneul_address_alias_ETooManyAliases">ETooManyAliases</a>);
}
</code></pre>



</details>

<a name="haneul_address_alias_replace_all"></a>

## Function `replace_all`

Overwrites the aliases for the sender's address with the given set.


<pre><code><b>entry</b> <b>fun</b> <a href="../haneul/address_alias.md#haneul_address_alias_replace_all">replace_all</a>(aliases: &<b>mut</b> <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliases">haneul::address_alias::AddressAliases</a>, new_aliases: vector&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>fun</b> <a href="../haneul/address_alias.md#haneul_address_alias_replace_all">replace_all</a>(aliases: &<b>mut</b> <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliases">AddressAliases</a>, new_aliases: vector&lt;<b>address</b>&gt;) {
    <b>let</b> new_aliases = <a href="../haneul/vec_set.md#haneul_vec_set_from_keys">vec_set::from_keys</a>(new_aliases);
    <b>assert</b>!(new_aliases.length() &gt; 0, <a href="../haneul/address_alias.md#haneul_address_alias_ECannotRemoveLastAlias">ECannotRemoveLastAlias</a>);
    <b>assert</b>!(new_aliases.length() &lt;= <a href="../haneul/address_alias.md#haneul_address_alias_MAX_ALIASES">MAX_ALIASES</a>, <a href="../haneul/address_alias.md#haneul_address_alias_ETooManyAliases">ETooManyAliases</a>);
    aliases.aliases = new_aliases;
}
</code></pre>



</details>

<a name="haneul_address_alias_remove"></a>

## Function `remove`

Removes the given alias from the set of aliases for the sender's address.


<pre><code><b>entry</b> <b>fun</b> <a href="../haneul/address_alias.md#haneul_address_alias_remove">remove</a>(aliases: &<b>mut</b> <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliases">haneul::address_alias::AddressAliases</a>, alias: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>fun</b> <a href="../haneul/address_alias.md#haneul_address_alias_remove">remove</a>(aliases: &<b>mut</b> <a href="../haneul/address_alias.md#haneul_address_alias_AddressAliases">AddressAliases</a>, alias: <b>address</b>) {
    <b>assert</b>!(aliases.aliases.contains(&alias), <a href="../haneul/address_alias.md#haneul_address_alias_ENoSuchAlias">ENoSuchAlias</a>);
    <b>assert</b>!(aliases.aliases.length() &gt; 1, <a href="../haneul/address_alias.md#haneul_address_alias_ECannotRemoveLastAlias">ECannotRemoveLastAlias</a>);
    aliases.aliases.<a href="../haneul/address_alias.md#haneul_address_alias_remove">remove</a>(&alias);
}
</code></pre>



</details>
