---
title: Module `haneul::coin_registry`
---

Defines the system object for managing coin data in a central
registry. This module provides a centralized way to store and manage
metadata for all currencies in the Haneul ecosystem, including their
supply information, regulatory status, and metadata capabilities.


-  [Struct `CoinRegistry`](#haneul_coin_registry_CoinRegistry)
-  [Struct `ExtraField`](#haneul_coin_registry_ExtraField)
-  [Struct `CurrencyKey`](#haneul_coin_registry_CurrencyKey)
-  [Struct `MetadataCap`](#haneul_coin_registry_MetadataCap)
-  [Struct `Currency`](#haneul_coin_registry_Currency)
-  [Struct `CurrencyInitializer`](#haneul_coin_registry_CurrencyInitializer)
-  [Enum `SupplyState`](#haneul_coin_registry_SupplyState)
-  [Enum `RegulatedState`](#haneul_coin_registry_RegulatedState)
-  [Enum `MetadataCapState`](#haneul_coin_registry_MetadataCapState)
-  [Constants](#@Constants_0)
-  [Function `new_currency`](#haneul_coin_registry_new_currency)
-  [Function `new_currency_with_otw`](#haneul_coin_registry_new_currency_with_otw)
-  [Function `claim_metadata_cap`](#haneul_coin_registry_claim_metadata_cap)
-  [Function `make_regulated`](#haneul_coin_registry_make_regulated)
-  [Function `make_supply_fixed_init`](#haneul_coin_registry_make_supply_fixed_init)
-  [Function `make_supply_burn_only_init`](#haneul_coin_registry_make_supply_burn_only_init)
-  [Function `make_supply_fixed`](#haneul_coin_registry_make_supply_fixed)
-  [Function `make_supply_burn_only`](#haneul_coin_registry_make_supply_burn_only)
-  [Function `finalize`](#haneul_coin_registry_finalize)
-  [Function `finalize_registration`](#haneul_coin_registry_finalize_registration)
-  [Function `delete_metadata_cap`](#haneul_coin_registry_delete_metadata_cap)
-  [Function `burn`](#haneul_coin_registry_burn)
-  [Function `burn_balance`](#haneul_coin_registry_burn_balance)
-  [Function `set_name`](#haneul_coin_registry_set_name)
-  [Function `set_description`](#haneul_coin_registry_set_description)
-  [Function `set_icon_url`](#haneul_coin_registry_set_icon_url)
-  [Function `set_treasury_cap_id`](#haneul_coin_registry_set_treasury_cap_id)
-  [Function `migrate_legacy_metadata`](#haneul_coin_registry_migrate_legacy_metadata)
-  [Function `update_from_legacy_metadata`](#haneul_coin_registry_update_from_legacy_metadata)
-  [Function `delete_migrated_legacy_metadata`](#haneul_coin_registry_delete_migrated_legacy_metadata)
-  [Function `migrate_regulated_state_by_metadata`](#haneul_coin_registry_migrate_regulated_state_by_metadata)
-  [Function `migrate_regulated_state_by_cap`](#haneul_coin_registry_migrate_regulated_state_by_cap)
-  [Function `decimals`](#haneul_coin_registry_decimals)
-  [Function `name`](#haneul_coin_registry_name)
-  [Function `symbol`](#haneul_coin_registry_symbol)
-  [Function `description`](#haneul_coin_registry_description)
-  [Function `icon_url`](#haneul_coin_registry_icon_url)
-  [Function `is_metadata_cap_claimed`](#haneul_coin_registry_is_metadata_cap_claimed)
-  [Function `is_metadata_cap_deleted`](#haneul_coin_registry_is_metadata_cap_deleted)
-  [Function `metadata_cap_id`](#haneul_coin_registry_metadata_cap_id)
-  [Function `treasury_cap_id`](#haneul_coin_registry_treasury_cap_id)
-  [Function `deny_cap_id`](#haneul_coin_registry_deny_cap_id)
-  [Function `is_supply_fixed`](#haneul_coin_registry_is_supply_fixed)
-  [Function `is_supply_burn_only`](#haneul_coin_registry_is_supply_burn_only)
-  [Function `is_regulated`](#haneul_coin_registry_is_regulated)
-  [Function `total_supply`](#haneul_coin_registry_total_supply)
-  [Function `exists`](#haneul_coin_registry_exists)
-  [Function `create`](#haneul_coin_registry_create)
-  [Macro function `is_ascii_printable`](#haneul_coin_registry_is_ascii_printable)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../haneul/accumulator.md#haneul_accumulator">haneul::accumulator</a>;
<b>use</b> <a href="../haneul/accumulator_metadata.md#haneul_accumulator_metadata">haneul::accumulator_metadata</a>;
<b>use</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement">haneul::accumulator_settlement</a>;
<b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/bag.md#haneul_bag">haneul::bag</a>;
<b>use</b> <a href="../haneul/balance.md#haneul_balance">haneul::balance</a>;
<b>use</b> <a href="../haneul/bcs.md#haneul_bcs">haneul::bcs</a>;
<b>use</b> <a href="../haneul/coin.md#haneul_coin">haneul::coin</a>;
<b>use</b> <a href="../haneul/config.md#haneul_config">haneul::config</a>;
<b>use</b> <a href="../haneul/deny_list.md#haneul_deny_list">haneul::deny_list</a>;
<b>use</b> <a href="../haneul/derived_object.md#haneul_derived_object">haneul::derived_object</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/dynamic_object_field.md#haneul_dynamic_object_field">haneul::dynamic_object_field</a>;
<b>use</b> <a href="../haneul/event.md#haneul_event">haneul::event</a>;
<b>use</b> <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator">haneul::funds_accumulator</a>;
<b>use</b> <a href="../haneul/hash.md#haneul_hash">haneul::hash</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/table.md#haneul_table">haneul::table</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/types.md#haneul_types">haneul::types</a>;
<b>use</b> <a href="../haneul/url.md#haneul_url">haneul::url</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../haneul/vec_set.md#haneul_vec_set">haneul::vec_set</a>;
</code></pre>



<a name="haneul_coin_registry_CoinRegistry"></a>

## Struct `CoinRegistry`

System object found at address <code>0xc</code> that stores coin data for all
registered coin types. This is a shared object that acts as a central
registry for coin metadata, supply information, and regulatory status.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">CoinRegistry</a> <b>has</b> key
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

<a name="haneul_coin_registry_ExtraField"></a>

## Struct `ExtraField`

Store only object that enables more flexible coin data
registration, allowing for additional fields to be added
without changing the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> structure.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_ExtraField">ExtraField</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>0: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>1: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_coin_registry_CurrencyKey"></a>

## Struct `CurrencyKey`

Key used to derive addresses when creating <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;</code> objects.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyKey">CurrencyKey</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="haneul_coin_registry_MetadataCap"></a>

## Struct `MetadataCap`

Capability object that gates metadata (name, description, icon_url, symbol)
changes in the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code>. It can only be created (or claimed) once, and can
be deleted to prevent changes to the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> metadata.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
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

<a name="haneul_coin_registry_Currency"></a>

## Struct `Currency`

Currency stores metadata such as name, symbol, decimals, icon_url and description,
as well as supply states (optional) and regulatory status.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;<b>phantom</b> T&gt; <b>has</b> key
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
<code><a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>: u8</code>
</dt>
<dd>
 Number of decimal places the coin uses for display purposes.
</dd>
<dt>
<code><a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Human-readable name for the coin.
</dd>
<dt>
<code><a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Short symbol/ticker for the coin.
</dd>
<dt>
<code><a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Detailed description of the coin.
</dd>
<dt>
<code><a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 URL for the coin's icon/logo.
</dd>
<dt>
<code>supply: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../haneul/coin_registry.md#haneul_coin_registry_SupplyState">haneul::coin_registry::SupplyState</a>&lt;T&gt;&gt;</code>
</dt>
<dd>
 Current supply state of the coin (fixed supply or unknown)
 Note: We're using <code>Option</code> because <code><a href="../haneul/coin_registry.md#haneul_coin_registry_SupplyState">SupplyState</a></code> does not have drop,
 meaning we cannot swap out its value at a later state.
</dd>
<dt>
<code>regulated: <a href="../haneul/coin_registry.md#haneul_coin_registry_RegulatedState">haneul::coin_registry::RegulatedState</a></code>
</dt>
<dd>
 Regulatory status of the coin (regulated with deny cap or unknown)
</dd>
<dt>
<code><a href="../haneul/coin_registry.md#haneul_coin_registry_treasury_cap_id">treasury_cap_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>&gt;</code>
</dt>
<dd>
 ID of the treasury cap for this coin type, if registered.
</dd>
<dt>
<code><a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>: <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCapState">haneul::coin_registry::MetadataCapState</a></code>
</dt>
<dd>
 ID of the metadata capability for this coin type, if claimed.
</dd>
<dt>
<code>extra_fields: <a href="../haneul/vec_map.md#haneul_vec_map_VecMap">haneul::vec_map::VecMap</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul/coin_registry.md#haneul_coin_registry_ExtraField">haneul::coin_registry::ExtraField</a>&gt;</code>
</dt>
<dd>
 Additional fields for extensibility.
</dd>
</dl>


</details>

<a name="haneul_coin_registry_CurrencyInitializer"></a>

## Struct `CurrencyInitializer`

Hot potato wrapper to enforce registration after "new_currency" data creation.
Destroyed in the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_finalize">finalize</a></code> call and either transferred to the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">CoinRegistry</a></code>
(in case of an OTW registration) or shared directly (for dynamically created
currencies).


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">CurrencyInitializer</a>&lt;<b>phantom</b> T&gt;
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>currency: <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>extra_fields: <a href="../haneul/bag.md#haneul_bag_Bag">haneul::bag::Bag</a></code>
</dt>
<dd>
</dd>
<dt>
<code>is_otw: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_coin_registry_SupplyState"></a>

## Enum `SupplyState`

Supply state marks the type of Currency Supply, which can be
- Fixed: no minting or burning;
- BurnOnly: no minting, burning is allowed;
- Unknown: flexible (supply is controlled by its <code>TreasuryCap</code>);


<pre><code><b>public</b> <b>enum</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_SupplyState">SupplyState</a>&lt;<b>phantom</b> T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Fixed</code>
</dt>
<dd>
 Coin has a fixed supply with the given Supply object.
</dd>

<dl>
<dt>
<code>0: <a href="../haneul/balance.md#haneul_balance_Supply">haneul::balance::Supply</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>BurnOnly</code>
</dt>
<dd>
 Coin has a supply that can ONLY decrease.
</dd>

<dl>
<dt>
<code>0: <a href="../haneul/balance.md#haneul_balance_Supply">haneul::balance::Supply</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>Unknown</code>
</dt>
<dd>
 Supply information is not yet known or registered.
</dd>
</dl>


</details>

<a name="haneul_coin_registry_RegulatedState"></a>

## Enum `RegulatedState`

Regulated state of a coin type.
- Regulated: <code>DenyCap</code> exists or a <code>RegulatedCoinMetadata</code> used to mark currency as regulated;
- Unregulated: the currency was created without deny list;
- Unknown: the regulatory status is unknown.


<pre><code><b>public</b> <b>enum</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_RegulatedState">RegulatedState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Regulated</code>
</dt>
<dd>
 Coin is regulated with a deny cap for address restrictions.
 <code>allow_global_pause</code> is <code>None</code> if the information is unknown (has not been migrated from <code>DenyCapV2</code>).
</dd>

<dl>
<dt>
<code>cap: <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


<dl>
<dt>
<code>allow_global_pause: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;bool&gt;</code>
</dt>
<dd>
</dd>
</dl>


<dl>
<dt>
<code>variant: u8</code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>Unregulated</code>
</dt>
<dd>
 The coin has been created without deny list.
</dd>
<dt>
Variant <code>Unknown</code>
</dt>
<dd>
 Regulatory status is unknown.
 Result of a legacy migration for that coin (from <code><a href="../haneul/coin.md#haneul_coin">coin</a>.<b>move</b></code> constructors)
</dd>
</dl>


</details>

<a name="haneul_coin_registry_MetadataCapState"></a>

## Enum `MetadataCapState`

State of the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a></code> for a single <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code>.


<pre><code><b>public</b> <b>enum</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCapState">MetadataCapState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>Claimed</code>
</dt>
<dd>
 The metadata cap has been claimed.
</dd>

<dl>
<dt>
<code>0: <a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>Unclaimed</code>
</dt>
<dd>
 The metadata cap has not been claimed.
</dd>
<dt>
Variant <code>Deleted</code>
</dt>
<dd>
 The metadata cap has been claimed and then deleted.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_coin_registry_EMetadataCapAlreadyClaimed"></a>

Metadata cap already claimed


<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_EMetadataCapAlreadyClaimed">EMetadataCapAlreadyClaimed</a>: vector&lt;u8&gt; = b"Metadata cap already claimed.";
</code></pre>



<a name="haneul_coin_registry_ENotSystemAddress"></a>

Only the system address can create the registry


<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_ENotSystemAddress">ENotSystemAddress</a>: vector&lt;u8&gt; = b"Only the system can <a href="../haneul/coin_registry.md#haneul_coin_registry_create">create</a> the registry.";
</code></pre>



<a name="haneul_coin_registry_ECurrencyAlreadyExists"></a>

Currency for this coin type already exists


<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_ECurrencyAlreadyExists">ECurrencyAlreadyExists</a>: vector&lt;u8&gt; = b"<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a> <b>for</b> this <a href="../haneul/coin.md#haneul_coin">coin</a> type already <a href="../haneul/coin_registry.md#haneul_coin_registry_exists">exists</a>.";
</code></pre>



<a name="haneul_coin_registry_EDenyListStateAlreadySet"></a>

Attempt to set the deny list state permissionlessly while it has already been set.


<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_EDenyListStateAlreadySet">EDenyListStateAlreadySet</a>: vector&lt;u8&gt; = b"Cannot set the deny list state <b>as</b> it <b>has</b> already been set.";
</code></pre>



<a name="haneul_coin_registry_EMetadataCapNotClaimed"></a>



<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_EMetadataCapNotClaimed">EMetadataCapNotClaimed</a>: vector&lt;u8&gt; = b"Cannot delete legacy metadata before claiming the `<a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a>`.";
</code></pre>



<a name="haneul_coin_registry_ECannotUpdateManagedMetadata"></a>

Attempt to update <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> with legacy metadata after the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a></code> has
been claimed. Updates are only allowed if the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a></code> has not yet been
claimed or deleted.


<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_ECannotUpdateManagedMetadata">ECannotUpdateManagedMetadata</a>: vector&lt;u8&gt; = b"Cannot update metadata whose `<a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a>` <b>has</b> already been claimed.";
</code></pre>



<a name="haneul_coin_registry_EInvalidSymbol"></a>

Attempt to set the symbol to a non-ASCII printable character


<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_EInvalidSymbol">EInvalidSymbol</a>: vector&lt;u8&gt; = b"Symbol <b>has</b> to be ASCII printable.";
</code></pre>



<a name="haneul_coin_registry_EDenyCapAlreadyCreated"></a>



<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_EDenyCapAlreadyCreated">EDenyCapAlreadyCreated</a>: vector&lt;u8&gt; = b"Cannot claim the deny cap twice.";
</code></pre>



<a name="haneul_coin_registry_ECurrencyAlreadyRegistered"></a>

Attempt to migrate legacy metadata for a <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> that already exists.


<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_ECurrencyAlreadyRegistered">ECurrencyAlreadyRegistered</a>: vector&lt;u8&gt; = b"<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a> already registered.";
</code></pre>



<a name="haneul_coin_registry_EEmptySupply"></a>



<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_EEmptySupply">EEmptySupply</a>: vector&lt;u8&gt; = b"Supply cannot be empty.";
</code></pre>



<a name="haneul_coin_registry_ESupplyNotBurnOnly"></a>



<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_ESupplyNotBurnOnly">ESupplyNotBurnOnly</a>: vector&lt;u8&gt; = b"Cannot <a href="../haneul/coin_registry.md#haneul_coin_registry_burn">burn</a> on a non <a href="../haneul/coin_registry.md#haneul_coin_registry_burn">burn</a>-only supply.";
</code></pre>



<a name="haneul_coin_registry_EInvariantViolation"></a>



<pre><code>#[error]
<b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_EInvariantViolation">EInvariantViolation</a>: vector&lt;u8&gt; = b"Code <b>invariant</b> violation.";
</code></pre>



<a name="haneul_coin_registry_REGULATED_COIN_VERSION"></a>

Incremental identifier for regulated coin versions in the deny list.
We start from <code>0</code> in the new system, which aligns with the state of <code>DenyCapV2</code>.


<pre><code><b>const</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_REGULATED_COIN_VERSION">REGULATED_COIN_VERSION</a>: u8 = 0;
</code></pre>



<a name="haneul_coin_registry_new_currency"></a>

## Function `new_currency`

Creates a new currency.

Note: This constructor has no long term difference from <code><a href="../haneul/coin_registry.md#haneul_coin_registry_new_currency_with_otw">new_currency_with_otw</a></code>.
This can be called from the module that defines <code>T</code> any time after it has been published.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_new_currency">new_currency</a>&lt;T: key&gt;(registry: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">haneul::coin_registry::CoinRegistry</a>, <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>: u8, <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): (<a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">haneul::coin_registry::CurrencyInitializer</a>&lt;T&gt;, <a href="../haneul/coin.md#haneul_coin_TreasuryCap">haneul::coin::TreasuryCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_new_currency">new_currency</a>&lt;T: /* internal */ key&gt;(
    registry: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">CoinRegistry</a>,
    <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>: u8,
    <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>: String,
    <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>: String,
    <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>: String,
    <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>: String,
    ctx: &<b>mut</b> TxContext,
): (<a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">CurrencyInitializer</a>&lt;T&gt;, TreasuryCap&lt;T&gt;) {
    <b>assert</b>!(!registry.<a href="../haneul/coin_registry.md#haneul_coin_registry_exists">exists</a>&lt;T&gt;(), <a href="../haneul/coin_registry.md#haneul_coin_registry_ECurrencyAlreadyExists">ECurrencyAlreadyExists</a>);
    <b>assert</b>!(<a href="../haneul/coin_registry.md#haneul_coin_registry_is_ascii_printable">is_ascii_printable</a>!(&<a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>), <a href="../haneul/coin_registry.md#haneul_coin_registry_EInvalidSymbol">EInvalidSymbol</a>);
    <b>let</b> treasury_cap = <a href="../haneul/coin.md#haneul_coin_new_treasury_cap">coin::new_treasury_cap</a>(ctx);
    <b>let</b> currency = <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt; {
        id: <a href="../haneul/derived_object.md#haneul_derived_object_claim">derived_object::claim</a>(&<b>mut</b> registry.id, <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyKey">CurrencyKey</a>&lt;T&gt;()),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>,
        supply: option::some(SupplyState::Unknown),
        regulated: RegulatedState::Unregulated,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_treasury_cap_id">treasury_cap_id</a>: option::some(<a href="../haneul/object.md#haneul_object_id">object::id</a>(&treasury_cap)),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>: MetadataCapState::Unclaimed,
        extra_fields: <a href="../haneul/vec_map.md#haneul_vec_map_empty">vec_map::empty</a>(),
    };
    (<a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">CurrencyInitializer</a> { currency, is_otw: <b>false</b>, extra_fields: <a href="../haneul/bag.md#haneul_bag_new">bag::new</a>(ctx) }, treasury_cap)
}
</code></pre>



</details>

<a name="haneul_coin_registry_new_currency_with_otw"></a>

## Function `new_currency_with_otw`

Creates a new currency with using an OTW as proof of uniqueness.

This is a two-step operation:
1. <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> is constructed in the <code>init</code> function and sent to the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">CoinRegistry</a></code>;
2. <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> is promoted to a shared object in the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_finalize_registration">finalize_registration</a></code> call;


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_new_currency_with_otw">new_currency_with_otw</a>&lt;T: drop&gt;(otw: T, <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>: u8, <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): (<a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">haneul::coin_registry::CurrencyInitializer</a>&lt;T&gt;, <a href="../haneul/coin.md#haneul_coin_TreasuryCap">haneul::coin::TreasuryCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_new_currency_with_otw">new_currency_with_otw</a>&lt;T: drop&gt;(
    otw: T,
    <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>: u8,
    <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>: String,
    <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>: String,
    <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>: String,
    <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>: String,
    ctx: &<b>mut</b> TxContext,
): (<a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">CurrencyInitializer</a>&lt;T&gt;, TreasuryCap&lt;T&gt;) {
    <b>assert</b>!(<a href="../haneul/types.md#haneul_types_is_one_time_witness">haneul::types::is_one_time_witness</a>(&otw));
    <b>assert</b>!(<a href="../haneul/coin_registry.md#haneul_coin_registry_is_ascii_printable">is_ascii_printable</a>!(&<a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>), <a href="../haneul/coin_registry.md#haneul_coin_registry_EInvalidSymbol">EInvalidSymbol</a>);
    <b>let</b> treasury_cap = <a href="../haneul/coin.md#haneul_coin_new_treasury_cap">coin::new_treasury_cap</a>(ctx);
    <b>let</b> currency = <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt; {
        id: <a href="../haneul/object.md#haneul_object_new">object::new</a>(ctx),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>,
        supply: option::some(SupplyState::Unknown),
        regulated: RegulatedState::Unregulated,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_treasury_cap_id">treasury_cap_id</a>: option::some(<a href="../haneul/object.md#haneul_object_id">object::id</a>(&treasury_cap)),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>: MetadataCapState::Unclaimed,
        extra_fields: <a href="../haneul/vec_map.md#haneul_vec_map_empty">vec_map::empty</a>(),
    };
    (<a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">CurrencyInitializer</a> { currency, is_otw: <b>true</b>, extra_fields: <a href="../haneul/bag.md#haneul_bag_new">bag::new</a>(ctx) }, treasury_cap)
}
</code></pre>



</details>

<a name="haneul_coin_registry_claim_metadata_cap"></a>

## Function `claim_metadata_cap`

Claim a <code><a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a></code> for a coin type.
Only allowed from the owner of <code>TreasuryCap</code>, and only once.

Aborts if the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a></code> has already been claimed.
Deleted <code><a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a></code> cannot be reclaimed.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_claim_metadata_cap">claim_metadata_cap</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, _: &<a href="../haneul/coin.md#haneul_coin_TreasuryCap">haneul::coin::TreasuryCap</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">haneul::coin_registry::MetadataCap</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_claim_metadata_cap">claim_metadata_cap</a>&lt;T&gt;(
    currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;,
    _: &TreasuryCap&lt;T&gt;,
    ctx: &<b>mut</b> TxContext,
): <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a>&lt;T&gt; {
    <b>assert</b>!(!currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_is_metadata_cap_claimed">is_metadata_cap_claimed</a>(), <a href="../haneul/coin_registry.md#haneul_coin_registry_EMetadataCapAlreadyClaimed">EMetadataCapAlreadyClaimed</a>);
    <b>let</b> id = <a href="../haneul/object.md#haneul_object_new">object::new</a>(ctx);
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a> = MetadataCapState::Claimed(id.to_inner());
    <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a> { id }
}
</code></pre>



</details>

<a name="haneul_coin_registry_make_regulated"></a>

## Function `make_regulated`

Allows converting a currency, on init, to regulated, which creates
a <code>DenyCapV2</code> object, and a denylist entry. Sets regulated state to
<code>Regulated</code>.

This action is irreversible.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_make_regulated">make_regulated</a>&lt;T&gt;(init: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">haneul::coin_registry::CurrencyInitializer</a>&lt;T&gt;, allow_global_pause: bool, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/coin.md#haneul_coin_DenyCapV2">haneul::coin::DenyCapV2</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_make_regulated">make_regulated</a>&lt;T&gt;(
    init: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">CurrencyInitializer</a>&lt;T&gt;,
    allow_global_pause: bool,
    ctx: &<b>mut</b> TxContext,
): DenyCapV2&lt;T&gt; {
    <b>assert</b>!(init.currency.regulated == RegulatedState::Unregulated, <a href="../haneul/coin_registry.md#haneul_coin_registry_EDenyCapAlreadyCreated">EDenyCapAlreadyCreated</a>);
    <b>let</b> deny_cap = <a href="../haneul/coin.md#haneul_coin_new_deny_cap_v2">coin::new_deny_cap_v2</a>&lt;T&gt;(allow_global_pause, ctx);
    init.currency.regulated =
        RegulatedState::Regulated {
            cap: <a href="../haneul/object.md#haneul_object_id">object::id</a>(&deny_cap),
            allow_global_pause: option::some(allow_global_pause),
            variant: <a href="../haneul/coin_registry.md#haneul_coin_registry_REGULATED_COIN_VERSION">REGULATED_COIN_VERSION</a>,
        };
    deny_cap
}
</code></pre>



</details>

<a name="haneul_coin_registry_make_supply_fixed_init"></a>

## Function `make_supply_fixed_init`

Initializer function to make the supply fixed.
Aborts if Supply is <code>0</code> to enforce minting during initialization.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_make_supply_fixed_init">make_supply_fixed_init</a>&lt;T&gt;(init: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">haneul::coin_registry::CurrencyInitializer</a>&lt;T&gt;, cap: <a href="../haneul/coin.md#haneul_coin_TreasuryCap">haneul::coin::TreasuryCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_make_supply_fixed_init">make_supply_fixed_init</a>&lt;T&gt;(init: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">CurrencyInitializer</a>&lt;T&gt;, cap: TreasuryCap&lt;T&gt;) {
    <b>assert</b>!(cap.<a href="../haneul/coin_registry.md#haneul_coin_registry_total_supply">total_supply</a>() &gt; 0, <a href="../haneul/coin_registry.md#haneul_coin_registry_EEmptySupply">EEmptySupply</a>);
    init.currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_make_supply_fixed">make_supply_fixed</a>(cap)
}
</code></pre>



</details>

<a name="haneul_coin_registry_make_supply_burn_only_init"></a>

## Function `make_supply_burn_only_init`

Initializer function to make the supply burn-only.
Aborts if Supply is <code>0</code> to enforce minting during initialization.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_make_supply_burn_only_init">make_supply_burn_only_init</a>&lt;T&gt;(init: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">haneul::coin_registry::CurrencyInitializer</a>&lt;T&gt;, cap: <a href="../haneul/coin.md#haneul_coin_TreasuryCap">haneul::coin::TreasuryCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_make_supply_burn_only_init">make_supply_burn_only_init</a>&lt;T&gt;(init: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">CurrencyInitializer</a>&lt;T&gt;, cap: TreasuryCap&lt;T&gt;) {
    <b>assert</b>!(cap.<a href="../haneul/coin_registry.md#haneul_coin_registry_total_supply">total_supply</a>() &gt; 0, <a href="../haneul/coin_registry.md#haneul_coin_registry_EEmptySupply">EEmptySupply</a>);
    init.currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_make_supply_burn_only">make_supply_burn_only</a>(cap)
}
</code></pre>



</details>

<a name="haneul_coin_registry_make_supply_fixed"></a>

## Function `make_supply_fixed`

Freeze the supply by destroying the <code>TreasuryCap</code> and storing it in the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_make_supply_fixed">make_supply_fixed</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, cap: <a href="../haneul/coin.md#haneul_coin_TreasuryCap">haneul::coin::TreasuryCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_make_supply_fixed">make_supply_fixed</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, cap: TreasuryCap&lt;T&gt;) {
    match (currency.supply.swap(SupplyState::Fixed(cap.into_supply()))) {
        // Impossible: We cannot fix a supply or make a supply <a href="../haneul/coin_registry.md#haneul_coin_registry_burn">burn</a>-only twice.
        SupplyState::Fixed(_supply) | SupplyState::BurnOnly(_supply) =&gt; <b>abort</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_EInvariantViolation">EInvariantViolation</a>,
        // We replaced "unknown" with fixed supply.
        SupplyState::Unknown =&gt; (),
    };
}
</code></pre>



</details>

<a name="haneul_coin_registry_make_supply_burn_only"></a>

## Function `make_supply_burn_only`

Make the supply <code>BurnOnly</code> by giving up the <code>TreasuryCap</code>, and allowing
burning of Coins through the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_make_supply_burn_only">make_supply_burn_only</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, cap: <a href="../haneul/coin.md#haneul_coin_TreasuryCap">haneul::coin::TreasuryCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_make_supply_burn_only">make_supply_burn_only</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, cap: TreasuryCap&lt;T&gt;) {
    match (currency.supply.swap(SupplyState::BurnOnly(cap.into_supply()))) {
        // Impossible: We cannot fix a supply or make a supply <a href="../haneul/coin_registry.md#haneul_coin_registry_burn">burn</a>-only twice.
        SupplyState::Fixed(_supply) | SupplyState::BurnOnly(_supply) =&gt; <b>abort</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_EInvariantViolation">EInvariantViolation</a>,
        // We replaced "unknown" with frozen supply.
        SupplyState::Unknown =&gt; (),
    };
}
</code></pre>



</details>

<a name="haneul_coin_registry_finalize"></a>

## Function `finalize`

Finalize the coin initialization, returning <code><a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_finalize">finalize</a>&lt;T&gt;(builder: <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">haneul::coin_registry::CurrencyInitializer</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">haneul::coin_registry::MetadataCap</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_finalize">finalize</a>&lt;T&gt;(builder: <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">CurrencyInitializer</a>&lt;T&gt;, ctx: &<b>mut</b> TxContext): <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a>&lt;T&gt; {
    <b>let</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyInitializer">CurrencyInitializer</a> { <b>mut</b> currency, is_otw, extra_fields } = builder;
    extra_fields.destroy_empty();
    <b>let</b> id = <a href="../haneul/object.md#haneul_object_new">object::new</a>(ctx);
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a> = MetadataCapState::Claimed(id.to_inner());
    <b>if</b> (is_otw) <a href="../haneul/transfer.md#haneul_transfer_transfer">transfer::transfer</a>(currency, <a href="../haneul/object.md#haneul_object_haneul_coin_registry_address">object::haneul_coin_registry_address</a>())
    <b>else</b> <a href="../haneul/transfer.md#haneul_transfer_share_object">transfer::share_object</a>(currency);
    <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a>&lt;T&gt; { id }
}
</code></pre>



</details>

<a name="haneul_coin_registry_finalize_registration"></a>

## Function `finalize_registration`

The second step in the "otw" initialization of coin metadata, that takes in
the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;</code> that was transferred from init, and transforms it in to a
"derived address" shared object.

Can be performed by anyone.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_finalize_registration">finalize_registration</a>&lt;T&gt;(registry: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">haneul::coin_registry::CoinRegistry</a>, currency: <a href="../haneul/transfer.md#haneul_transfer_Receiving">haneul::transfer::Receiving</a>&lt;<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;&gt;, _ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_finalize_registration">finalize_registration</a>&lt;T&gt;(
    registry: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">CoinRegistry</a>,
    currency: Receiving&lt;<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;&gt;,
    _ctx: &<b>mut</b> TxContext,
) {
    // 1. Consume <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>
    // 2. Re-<a href="../haneul/coin_registry.md#haneul_coin_registry_create">create</a> it with a "derived" <b>address</b>.
    <b>let</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a> {
        id,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>,
        supply,
        regulated,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_treasury_cap_id">treasury_cap_id</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>,
        extra_fields,
    } = <a href="../haneul/transfer.md#haneul_transfer_receive">transfer::receive</a>(&<b>mut</b> registry.id, currency);
    id.delete();
    // Now, <a href="../haneul/coin_registry.md#haneul_coin_registry_create">create</a> the derived version of the <a href="../haneul/coin.md#haneul_coin">coin</a> currency.
    <a href="../haneul/transfer.md#haneul_transfer_share_object">transfer::share_object</a>(<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a> {
        id: <a href="../haneul/derived_object.md#haneul_derived_object_claim">derived_object::claim</a>(&<b>mut</b> registry.id, <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyKey">CurrencyKey</a>&lt;T&gt;()),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>,
        supply,
        regulated,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_treasury_cap_id">treasury_cap_id</a>,
        <a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>,
        extra_fields,
    })
}
</code></pre>



</details>

<a name="haneul_coin_registry_delete_metadata_cap"></a>

## Function `delete_metadata_cap`

Delete the metadata cap making further updates of <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> metadata impossible.
This action is IRREVERSIBLE, and the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a></code> can no longer be claimed.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_delete_metadata_cap">delete_metadata_cap</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, cap: <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">haneul::coin_registry::MetadataCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_delete_metadata_cap">delete_metadata_cap</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, cap: <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a>&lt;T&gt;) {
    <b>let</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a> { id } = cap;
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a> = MetadataCapState::Deleted;
    id.delete();
}
</code></pre>



</details>

<a name="haneul_coin_registry_burn"></a>

## Function `burn`

Burn the <code>Coin</code> if the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> has a <code>BurnOnly</code> supply state.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_burn">burn</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, <a href="../haneul/coin.md#haneul_coin">coin</a>: <a href="../haneul/coin.md#haneul_coin_Coin">haneul::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_burn">burn</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, <a href="../haneul/coin.md#haneul_coin">coin</a>: Coin&lt;T&gt;) {
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_burn_balance">burn_balance</a>(<a href="../haneul/coin.md#haneul_coin">coin</a>.into_balance());
}
</code></pre>



</details>

<a name="haneul_coin_registry_burn_balance"></a>

## Function `burn_balance`

Burn the <code>Balance</code> if the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> has a <code>BurnOnly</code> supply state.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_burn_balance">burn_balance</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, <a href="../haneul/balance.md#haneul_balance">balance</a>: <a href="../haneul/balance.md#haneul_balance_Balance">haneul::balance::Balance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_burn_balance">burn_balance</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, <a href="../haneul/balance.md#haneul_balance">balance</a>: Balance&lt;T&gt;) {
    <b>assert</b>!(currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_is_supply_burn_only">is_supply_burn_only</a>(), <a href="../haneul/coin_registry.md#haneul_coin_registry_ESupplyNotBurnOnly">ESupplyNotBurnOnly</a>);
    match (currency.supply.borrow_mut()) {
        SupplyState::BurnOnly(supply) =&gt; { supply.decrease_supply(<a href="../haneul/balance.md#haneul_balance">balance</a>); },
        _ =&gt; <b>abort</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_EInvariantViolation">EInvariantViolation</a>, // unreachable
    }
}
</code></pre>



</details>

<a name="haneul_coin_registry_set_name"></a>

## Function `set_name`

Update the name of the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_set_name">set_name</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, _: &<a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">haneul::coin_registry::MetadataCap</a>&lt;T&gt;, <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_set_name">set_name</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, _: &<a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a>&lt;T&gt;, <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>: String) {
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a> = <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>;
}
</code></pre>



</details>

<a name="haneul_coin_registry_set_description"></a>

## Function `set_description`

Update the description of the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_set_description">set_description</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, _: &<a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">haneul::coin_registry::MetadataCap</a>&lt;T&gt;, <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_set_description">set_description</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, _: &<a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a>&lt;T&gt;, <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>: String) {
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a> = <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>;
}
</code></pre>



</details>

<a name="haneul_coin_registry_set_icon_url"></a>

## Function `set_icon_url`

Update the icon URL of the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_set_icon_url">set_icon_url</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, _: &<a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">haneul::coin_registry::MetadataCap</a>&lt;T&gt;, <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_set_icon_url">set_icon_url</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, _: &<a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a>&lt;T&gt;, <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>: String) {
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a> = <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>;
}
</code></pre>



</details>

<a name="haneul_coin_registry_set_treasury_cap_id"></a>

## Function `set_treasury_cap_id`

Register the treasury cap ID for a migrated <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code>. All currencies created with
<code><a href="../haneul/coin_registry.md#haneul_coin_registry_new_currency">new_currency</a></code> or <code><a href="../haneul/coin_registry.md#haneul_coin_registry_new_currency_with_otw">new_currency_with_otw</a></code> have their treasury cap ID set during
initialization.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_set_treasury_cap_id">set_treasury_cap_id</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, cap: &<a href="../haneul/coin.md#haneul_coin_TreasuryCap">haneul::coin::TreasuryCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_set_treasury_cap_id">set_treasury_cap_id</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, cap: &TreasuryCap&lt;T&gt;) {
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_treasury_cap_id">treasury_cap_id</a>.fill(<a href="../haneul/object.md#haneul_object_id">object::id</a>(cap));
}
</code></pre>



</details>

<a name="haneul_coin_registry_migrate_legacy_metadata"></a>

## Function `migrate_legacy_metadata`

Register <code>CoinMetadata</code> in the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">CoinRegistry</a></code>. This can happen only once, if the
<code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> did not exist yet. Further updates are possible through
<code><a href="../haneul/coin_registry.md#haneul_coin_registry_update_from_legacy_metadata">update_from_legacy_metadata</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_migrate_legacy_metadata">migrate_legacy_metadata</a>&lt;T&gt;(registry: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">haneul::coin_registry::CoinRegistry</a>, legacy: &<a href="../haneul/coin.md#haneul_coin_CoinMetadata">haneul::coin::CoinMetadata</a>&lt;T&gt;, _ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_migrate_legacy_metadata">migrate_legacy_metadata</a>&lt;T&gt;(
    registry: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">CoinRegistry</a>,
    legacy: &CoinMetadata&lt;T&gt;,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(!registry.<a href="../haneul/coin_registry.md#haneul_coin_registry_exists">exists</a>&lt;T&gt;(), <a href="../haneul/coin_registry.md#haneul_coin_registry_ECurrencyAlreadyRegistered">ECurrencyAlreadyRegistered</a>);
    <b>assert</b>!(<a href="../haneul/coin_registry.md#haneul_coin_registry_is_ascii_printable">is_ascii_printable</a>!(&legacy.get_symbol().to_string()), <a href="../haneul/coin_registry.md#haneul_coin_registry_EInvalidSymbol">EInvalidSymbol</a>);
    <a href="../haneul/transfer.md#haneul_transfer_share_object">transfer::share_object</a>(<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt; {
        id: <a href="../haneul/derived_object.md#haneul_derived_object_claim">derived_object::claim</a>(&<b>mut</b> registry.id, <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyKey">CurrencyKey</a>&lt;T&gt;()),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>: legacy.get_decimals(),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>: legacy.get_name(),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>: legacy.get_symbol().to_string(),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>: legacy.get_description(),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>: legacy
            .get_icon_url()
            .map!(|<a href="../haneul/url.md#haneul_url">url</a>| <a href="../haneul/url.md#haneul_url">url</a>.inner_url().to_string())
            .destroy_or!(b"".to_string()),
        supply: option::some(SupplyState::Unknown),
        regulated: RegulatedState::Unknown, // We don't know <b>if</b> it's regulated or not!
        <a href="../haneul/coin_registry.md#haneul_coin_registry_treasury_cap_id">treasury_cap_id</a>: option::none(),
        <a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>: MetadataCapState::Unclaimed,
        extra_fields: <a href="../haneul/vec_map.md#haneul_vec_map_empty">vec_map::empty</a>(),
    });
}
</code></pre>



</details>

<a name="haneul_coin_registry_update_from_legacy_metadata"></a>

## Function `update_from_legacy_metadata`

Update <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> from <code>CoinMetadata</code> if the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a></code> is not claimed. After
the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_MetadataCap">MetadataCap</a></code> is claimed, updates can only be made through <code>set_*</code> functions.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_update_from_legacy_metadata">update_from_legacy_metadata</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, legacy: &<a href="../haneul/coin.md#haneul_coin_CoinMetadata">haneul::coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_update_from_legacy_metadata">update_from_legacy_metadata</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, legacy: &CoinMetadata&lt;T&gt;) {
    <b>assert</b>!(!currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_is_metadata_cap_claimed">is_metadata_cap_claimed</a>(), <a href="../haneul/coin_registry.md#haneul_coin_registry_ECannotUpdateManagedMetadata">ECannotUpdateManagedMetadata</a>);
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a> = legacy.get_name();
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a> = legacy.get_symbol().to_string();
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a> = legacy.get_description();
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a> = legacy.get_decimals();
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a> =
        legacy.get_icon_url().map!(|<a href="../haneul/url.md#haneul_url">url</a>| <a href="../haneul/url.md#haneul_url">url</a>.inner_url().to_string()).destroy_or!(b"".to_string());
}
</code></pre>



</details>

<a name="haneul_coin_registry_delete_migrated_legacy_metadata"></a>

## Function `delete_migrated_legacy_metadata`

Delete the legacy <code>CoinMetadata</code> object if the metadata cap for the new registry
has already been claimed.

This function is only callable after there's "proof" that the author of the coin
can manage the metadata using the registry system (so having a metadata cap claimed).


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_delete_migrated_legacy_metadata">delete_migrated_legacy_metadata</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, legacy: <a href="../haneul/coin.md#haneul_coin_CoinMetadata">haneul::coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_delete_migrated_legacy_metadata">delete_migrated_legacy_metadata</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, legacy: CoinMetadata&lt;T&gt;) {
    <b>assert</b>!(currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_is_metadata_cap_claimed">is_metadata_cap_claimed</a>(), <a href="../haneul/coin_registry.md#haneul_coin_registry_EMetadataCapNotClaimed">EMetadataCapNotClaimed</a>);
    legacy.destroy_metadata();
}
</code></pre>



</details>

<a name="haneul_coin_registry_migrate_regulated_state_by_metadata"></a>

## Function `migrate_regulated_state_by_metadata`

Allow migrating the regulated state by access to <code>RegulatedCoinMetadata</code> frozen object.
This is a permissionless operation which can be performed only once.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_migrate_regulated_state_by_metadata">migrate_regulated_state_by_metadata</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, metadata: &<a href="../haneul/coin.md#haneul_coin_RegulatedCoinMetadata">haneul::coin::RegulatedCoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_migrate_regulated_state_by_metadata">migrate_regulated_state_by_metadata</a>&lt;T&gt;(
    currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;,
    metadata: &RegulatedCoinMetadata&lt;T&gt;,
) {
    // Only allow <b>if</b> this hasn't been migrated before.
    <b>assert</b>!(currency.regulated == RegulatedState::Unknown, <a href="../haneul/coin_registry.md#haneul_coin_registry_EDenyListStateAlreadySet">EDenyListStateAlreadySet</a>);
    currency.regulated =
        RegulatedState::Regulated {
            cap: metadata.<a href="../haneul/coin_registry.md#haneul_coin_registry_deny_cap_id">deny_cap_id</a>(),
            allow_global_pause: option::none(),
            variant: <a href="../haneul/coin_registry.md#haneul_coin_registry_REGULATED_COIN_VERSION">REGULATED_COIN_VERSION</a>,
        };
}
</code></pre>



</details>

<a name="haneul_coin_registry_migrate_regulated_state_by_cap"></a>

## Function `migrate_regulated_state_by_cap`

Mark regulated state by showing the <code>DenyCapV2</code> object for the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_migrate_regulated_state_by_cap">migrate_regulated_state_by_cap</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;, cap: &<a href="../haneul/coin.md#haneul_coin_DenyCapV2">haneul::coin::DenyCapV2</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_migrate_regulated_state_by_cap">migrate_regulated_state_by_cap</a>&lt;T&gt;(currency: &<b>mut</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;, cap: &DenyCapV2&lt;T&gt;) {
    currency.regulated =
        RegulatedState::Regulated {
            cap: <a href="../haneul/object.md#haneul_object_id">object::id</a>(cap),
            allow_global_pause: option::some(cap.allow_global_pause()),
            variant: <a href="../haneul/coin_registry.md#haneul_coin_registry_REGULATED_COIN_VERSION">REGULATED_COIN_VERSION</a>,
        };
}
</code></pre>



</details>

<a name="haneul_coin_registry_decimals"></a>

## Function `decimals`

Get the number of decimal places for the coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): u8 { currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_decimals">decimals</a> }
</code></pre>



</details>

<a name="haneul_coin_registry_name"></a>

## Function `name`

Get the human-readable name of the coin.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): String { currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_name">name</a> }
</code></pre>



</details>

<a name="haneul_coin_registry_symbol"></a>

## Function `symbol`

Get the symbol/ticker of the coin.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): String { currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_symbol">symbol</a> }
</code></pre>



</details>

<a name="haneul_coin_registry_description"></a>

## Function `description`

Get the description of the coin.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): String { currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_description">description</a> }
</code></pre>



</details>

<a name="haneul_coin_registry_icon_url"></a>

## Function `icon_url`

Get the icon URL for the coin.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): String { currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_icon_url">icon_url</a> }
</code></pre>



</details>

<a name="haneul_coin_registry_is_metadata_cap_claimed"></a>

## Function `is_metadata_cap_claimed`

Check if the metadata capability has been claimed for this <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> type.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_metadata_cap_claimed">is_metadata_cap_claimed</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_metadata_cap_claimed">is_metadata_cap_claimed</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): bool {
    match (currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>) {
        MetadataCapState::Claimed(_) | MetadataCapState::Deleted =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="haneul_coin_registry_is_metadata_cap_deleted"></a>

## Function `is_metadata_cap_deleted`

Check if the metadata capability has been deleted for this <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> type.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_metadata_cap_deleted">is_metadata_cap_deleted</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_metadata_cap_deleted">is_metadata_cap_deleted</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): bool {
    match (currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>) {
        MetadataCapState::Deleted =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="haneul_coin_registry_metadata_cap_id"></a>

## Function `metadata_cap_id`

Get the metadata cap ID, or none if it has not been claimed.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): Option&lt;ID&gt; {
    match (currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_metadata_cap_id">metadata_cap_id</a>) {
        MetadataCapState::Claimed(id) =&gt; option::some(id),
        _ =&gt; option::none(),
    }
}
</code></pre>



</details>

<a name="haneul_coin_registry_treasury_cap_id"></a>

## Function `treasury_cap_id`

Get the treasury cap ID for this coin type, if registered.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_treasury_cap_id">treasury_cap_id</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_treasury_cap_id">treasury_cap_id</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): Option&lt;ID&gt; {
    currency.<a href="../haneul/coin_registry.md#haneul_coin_registry_treasury_cap_id">treasury_cap_id</a>
}
</code></pre>



</details>

<a name="haneul_coin_registry_deny_cap_id"></a>

## Function `deny_cap_id`

Get the deny cap ID for this coin type, if it's a regulated coin.
Returns <code>None</code> if:
- The <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> is not regulated;
- The <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a></code> is migrated from legacy, and its regulated state has not been set;


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_deny_cap_id">deny_cap_id</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../haneul/object.md#haneul_object_ID">haneul::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_deny_cap_id">deny_cap_id</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): Option&lt;ID&gt; {
    match (currency.regulated) {
        RegulatedState::Regulated { cap, .. } =&gt; option::some(cap),
        RegulatedState::Unregulated | RegulatedState::Unknown =&gt; option::none(),
    }
}
</code></pre>



</details>

<a name="haneul_coin_registry_is_supply_fixed"></a>

## Function `is_supply_fixed`

Check if the supply is fixed.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_supply_fixed">is_supply_fixed</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_supply_fixed">is_supply_fixed</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): bool {
    match (currency.supply.<a href="../haneul/borrow.md#haneul_borrow">borrow</a>()) {
        SupplyState::Fixed(_) =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="haneul_coin_registry_is_supply_burn_only"></a>

## Function `is_supply_burn_only`

Check if the supply is burn-only.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_supply_burn_only">is_supply_burn_only</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_supply_burn_only">is_supply_burn_only</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): bool {
    match (currency.supply.<a href="../haneul/borrow.md#haneul_borrow">borrow</a>()) {
        SupplyState::BurnOnly(_) =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="haneul_coin_registry_is_regulated"></a>

## Function `is_regulated`

Check if the currency is regulated.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_regulated">is_regulated</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_regulated">is_regulated</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): bool {
    match (currency.regulated) {
        RegulatedState::Regulated { .. } =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="haneul_coin_registry_total_supply"></a>

## Function `total_supply`

Get the total supply for the <code><a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;</code> if the Supply is in fixed or
burn-only state. Returns <code>None</code> if the SupplyState is Unknown.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_total_supply">total_supply</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">haneul::coin_registry::Currency</a>&lt;T&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_total_supply">total_supply</a>&lt;T&gt;(currency: &<a href="../haneul/coin_registry.md#haneul_coin_registry_Currency">Currency</a>&lt;T&gt;): Option&lt;u64&gt; {
    match (currency.supply.<a href="../haneul/borrow.md#haneul_borrow">borrow</a>()) {
        SupplyState::Fixed(supply) =&gt; option::some(supply.value()),
        SupplyState::BurnOnly(supply) =&gt; option::some(supply.value()),
        SupplyState::Unknown =&gt; option::none(),
    }
}
</code></pre>



</details>

<a name="haneul_coin_registry_exists"></a>

## Function `exists`

Check if coin data exists for the given type T in the registry.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_exists">exists</a>&lt;T&gt;(registry: &<a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">haneul::coin_registry::CoinRegistry</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_exists">exists</a>&lt;T&gt;(registry: &<a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">CoinRegistry</a>): bool {
    <a href="../haneul/derived_object.md#haneul_derived_object_exists">derived_object::exists</a>(&registry.id, <a href="../haneul/coin_registry.md#haneul_coin_registry_CurrencyKey">CurrencyKey</a>&lt;T&gt;())
}
</code></pre>



</details>

<a name="haneul_coin_registry_create"></a>

## Function `create`

Create and share the singleton <code><a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">CoinRegistry</a></code> -- this function is
called exactly once, during the upgrade epoch.
Only the system address (0x0) can create the registry.


<pre><code><b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_create">create</a>(ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_create">create</a>(ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../haneul/coin_registry.md#haneul_coin_registry_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../haneul/transfer.md#haneul_transfer_share_object">transfer::share_object</a>(<a href="../haneul/coin_registry.md#haneul_coin_registry_CoinRegistry">CoinRegistry</a> {
        id: <a href="../haneul/object.md#haneul_object_haneul_coin_registry_object_id">object::haneul_coin_registry_object_id</a>(),
    });
}
</code></pre>



</details>

<a name="haneul_coin_registry_is_ascii_printable"></a>

## Macro function `is_ascii_printable`

Nit: consider adding this function to <code><a href="../std/string.md#std_string">std::string</a></code> in the future.


<pre><code><b>macro</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_ascii_printable">is_ascii_printable</a>($s: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>macro</b> <b>fun</b> <a href="../haneul/coin_registry.md#haneul_coin_registry_is_ascii_printable">is_ascii_printable</a>($s: &String): bool {
    <b>let</b> s = $s;
    s.as_bytes().all!(|b| ascii::is_printable_char(*b))
}
</code></pre>



</details>
