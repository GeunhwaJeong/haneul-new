---
title: Module `haneul::kiosk_extension`
---

This module implements the Kiosk Extensions functionality. It allows
exposing previously protected (only-owner) methods to third-party apps.

A Kiosk Extension is a module that implements any functionality on top of
the <code>Kiosk</code> without discarding nor blocking the base. Given that <code>Kiosk</code>
itself is a trading primitive, most of the extensions are expected to be
related to trading. However, there's no limit to what can be built using the
<code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension">kiosk_extension</a></code> module, as it gives certain benefits such as using <code>Kiosk</code>
as the storage for any type of data / assets.


<a name="@Flow:_0"></a>

#### Flow:

- An extension can only be installed by the Kiosk Owner and requires an
authorization via the <code>KioskOwnerCap</code>.
- When installed, the extension is given a permission bitmap that allows it
to perform certain protected actions (eg <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code>, <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a></code>). However, it is
possible to install an extension that does not have any permissions.
- Kiosk Owner can <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_disable">disable</a></code> the extension at any time, which prevents it
from performing any protected actions. The storage is still available to the
extension until it is completely removed.
- A disabled extension can be <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_enable">enable</a></code>d at any time giving the permissions
back to the extension.
- An extension permissions follow the all-or-nothing policy. Either all of
the requested permissions are granted or none of them (can't install).


<a name="@Examples:_1"></a>

#### Examples:

- An Auction extension can utilize the storage to store Auction-related data
while utilizing the same <code>Kiosk</code> object that the items are stored in.
- A Marketplace extension that implements custom events and fees for the
default trading functionality.


<a name="@Notes:_2"></a>

#### Notes:

- Trading functionality can utilize the <code>PurchaseCap</code> to build a custom
logic around the purchase flow. However, it should be carefully managed to
prevent asset locking.
- <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension">kiosk_extension</a></code> is a friend module to <code><a href="../haneul/kiosk.md#haneul_kiosk">kiosk</a></code> and has access to its
internal functions (such as <code>place_internal</code> and <code>lock_internal</code> to
implement custom authorization scheme for <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code> and <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a></code> respectively).


        -  [Flow:](#@Flow:_0)
        -  [Examples:](#@Examples:_1)
        -  [Notes:](#@Notes:_2)
-  [Struct `Extension`](#haneul_kiosk_extension_Extension)
-  [Struct `ExtensionKey`](#haneul_kiosk_extension_ExtensionKey)
-  [Constants](#@Constants_3)
-  [Function `add`](#haneul_kiosk_extension_add)
-  [Function `disable`](#haneul_kiosk_extension_disable)
-  [Function `enable`](#haneul_kiosk_extension_enable)
-  [Function `remove`](#haneul_kiosk_extension_remove)
-  [Function `storage`](#haneul_kiosk_extension_storage)
-  [Function `storage_mut`](#haneul_kiosk_extension_storage_mut)
-  [Function `place`](#haneul_kiosk_extension_place)
-  [Function `lock`](#haneul_kiosk_extension_lock)
-  [Function `is_installed`](#haneul_kiosk_extension_is_installed)
-  [Function `is_enabled`](#haneul_kiosk_extension_is_enabled)
-  [Function `can_place`](#haneul_kiosk_extension_can_place)
-  [Function `can_lock`](#haneul_kiosk_extension_can_lock)
-  [Function `extension`](#haneul_kiosk_extension_extension)
-  [Function `extension_mut`](#haneul_kiosk_extension_extension_mut)


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
<b>use</b> <a href="../haneul/kiosk.md#haneul_kiosk">haneul::kiosk</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/package.md#haneul_package">haneul::package</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/protocol_config.md#haneul_protocol_config">haneul::protocol_config</a>;
<b>use</b> <a href="../haneul/haneul.md#haneul_haneul">haneul::haneul</a>;
<b>use</b> <a href="../haneul/table.md#haneul_table">haneul::table</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/transfer_policy.md#haneul_transfer_policy">haneul::transfer_policy</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/types.md#haneul_types">haneul::types</a>;
<b>use</b> <a href="../haneul/url.md#haneul_url">haneul::url</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../haneul/vec_set.md#haneul_vec_set">haneul::vec_set</a>;
</code></pre>



<a name="haneul_kiosk_extension_Extension"></a>

## Struct `Extension`

The Extension struct contains the data used by the extension and the
configuration for this extension. Stored under the <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ExtensionKey">ExtensionKey</a></code>
dynamic field.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_Extension">Extension</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_storage">storage</a>: <a href="../haneul/bag.md#haneul_bag_Bag">haneul::bag::Bag</a></code>
</dt>
<dd>
 Storage for the extension, an isolated Bag. By putting the extension
 into a single dynamic field, we reduce the amount of fields on the
 top level (eg items / listings) while giving extension developers
 the ability to store any data they want.
</dd>
<dt>
<code>permissions: u128</code>
</dt>
<dd>
 Bitmap of permissions that the extension has (can be revoked any
 moment). It's all or nothing policy - either the extension has the
 required permissions or no permissions at all.
 1st bit - <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code> - allows to place items for sale
 2nd bit - <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a></code> and <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code> - allows to lock items (and place)
 For example:
 - <code>10</code> - allows to place items and lock them.
 - <code>11</code> - allows to place items and lock them (<code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a></code> includes <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code>).
 - <code>01</code> - allows to place items, but not lock them.
 - <code>00</code> - no permissions.
</dd>
<dt>
<code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_enabled">is_enabled</a>: bool</code>
</dt>
<dd>
 Whether the extension can call protected actions. By default, all
 extensions are enabled (on <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_add">add</a></code> call), however the Kiosk
 owner can disable them at any time.
 Disabling the extension does not limit its access to the storage.
</dd>
</dl>


</details>

<a name="haneul_kiosk_extension_ExtensionKey"></a>

## Struct `ExtensionKey`

The <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ExtensionKey">ExtensionKey</a></code> is a typed dynamic field key used to store the
extension configuration and data. <code>Ext</code> is a phantom type that is used
to identify the extension witness.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ExtensionKey">ExtensionKey</a>&lt;<b>phantom</b> Ext&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_3"></a>

## Constants


<a name="haneul_kiosk_extension_ENotOwner"></a>

Trying to add an extension while not being the owner of the Kiosk.


<pre><code><b>const</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ENotOwner">ENotOwner</a>: u64 = 0;
</code></pre>



<a name="haneul_kiosk_extension_EExtensionNotAllowed"></a>

Extension is trying to access a permissioned action while not having
the required permission.


<pre><code><b>const</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotAllowed">EExtensionNotAllowed</a>: u64 = 2;
</code></pre>



<a name="haneul_kiosk_extension_EExtensionNotInstalled"></a>

Extension is not installed in the Kiosk.


<pre><code><b>const</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotInstalled">EExtensionNotInstalled</a>: u64 = 3;
</code></pre>



<a name="haneul_kiosk_extension_PLACE"></a>

Value that represents the <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code> permission in the permissions bitmap.


<pre><code><b>const</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_PLACE">PLACE</a>: u128 = 1;
</code></pre>



<a name="haneul_kiosk_extension_LOCK"></a>

Value that represents the <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a></code> and <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code> permission in the
permissions bitmap.


<pre><code><b>const</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_LOCK">LOCK</a>: u128 = 2;
</code></pre>



<a name="haneul_kiosk_extension_add"></a>

## Function `add`

Add an extension to the Kiosk. Can only be performed by the owner. The
extension witness is required to allow extensions define their set of
permissions in the custom <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_add">add</a></code> call.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_add">add</a>&lt;Ext: drop&gt;(_ext: Ext, self: &<b>mut</b> <a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>, cap: &<a href="../haneul/kiosk.md#haneul_kiosk_KioskOwnerCap">haneul::kiosk::KioskOwnerCap</a>, permissions: u128, ctx: &<b>mut</b> <a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_add">add</a>&lt;Ext: drop&gt;(
    _ext: Ext,
    self: &<b>mut</b> Kiosk,
    cap: &KioskOwnerCap,
    permissions: u128,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(self.has_access(cap), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ENotOwner">ENotOwner</a>);
    df::add(
        self.uid_mut_as_owner(cap),
        <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ExtensionKey">ExtensionKey</a>&lt;Ext&gt; {},
        <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_Extension">Extension</a> {
            <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_storage">storage</a>: <a href="../haneul/bag.md#haneul_bag_new">bag::new</a>(ctx),
            permissions,
            <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_enabled">is_enabled</a>: <b>true</b>,
        },
    )
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_disable"></a>

## Function `disable`

Revoke permissions from the extension. While it does not remove the
extension completely, it keeps it from performing any protected actions.
The storage is still available to the extension (until it's removed).


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_disable">disable</a>&lt;Ext: drop&gt;(self: &<b>mut</b> <a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>, cap: &<a href="../haneul/kiosk.md#haneul_kiosk_KioskOwnerCap">haneul::kiosk::KioskOwnerCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_disable">disable</a>&lt;Ext: drop&gt;(self: &<b>mut</b> Kiosk, cap: &KioskOwnerCap) {
    <b>assert</b>!(self.has_access(cap), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_installed">is_installed</a>&lt;Ext&gt;(self), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotInstalled">EExtensionNotInstalled</a>);
    <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension_mut">extension_mut</a>&lt;Ext&gt;(self).<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_enabled">is_enabled</a> = <b>false</b>;
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_enable"></a>

## Function `enable`

Re-enable the extension allowing it to call protected actions (eg
<code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code>, <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a></code>). By default, all added extensions are enabled. Kiosk
owner can disable them via <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_disable">disable</a></code> call.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_enable">enable</a>&lt;Ext: drop&gt;(self: &<b>mut</b> <a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>, cap: &<a href="../haneul/kiosk.md#haneul_kiosk_KioskOwnerCap">haneul::kiosk::KioskOwnerCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_enable">enable</a>&lt;Ext: drop&gt;(self: &<b>mut</b> Kiosk, cap: &KioskOwnerCap) {
    <b>assert</b>!(self.has_access(cap), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_installed">is_installed</a>&lt;Ext&gt;(self), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotInstalled">EExtensionNotInstalled</a>);
    <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension_mut">extension_mut</a>&lt;Ext&gt;(self).<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_enabled">is_enabled</a> = <b>true</b>;
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_remove"></a>

## Function `remove`

Remove an extension from the Kiosk. Can only be performed by the owner,
the extension storage must be empty for the transaction to succeed.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_remove">remove</a>&lt;Ext: drop&gt;(self: &<b>mut</b> <a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>, cap: &<a href="../haneul/kiosk.md#haneul_kiosk_KioskOwnerCap">haneul::kiosk::KioskOwnerCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_remove">remove</a>&lt;Ext: drop&gt;(self: &<b>mut</b> Kiosk, cap: &KioskOwnerCap) {
    <b>assert</b>!(self.has_access(cap), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_installed">is_installed</a>&lt;Ext&gt;(self), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotInstalled">EExtensionNotInstalled</a>);
    <b>let</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_Extension">Extension</a> {
        <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_storage">storage</a>,
        permissions: _,
        <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_enabled">is_enabled</a>: _,
    } = df::remove(self.uid_mut_as_owner(cap), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ExtensionKey">ExtensionKey</a>&lt;Ext&gt; {});
    <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_storage">storage</a>.destroy_empty();
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_storage"></a>

## Function `storage`

Get immutable access to the extension storage. Can only be performed by
the extension as long as the extension is installed.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_storage">storage</a>&lt;Ext: drop&gt;(_ext: Ext, self: &<a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>): &<a href="../haneul/bag.md#haneul_bag_Bag">haneul::bag::Bag</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_storage">storage</a>&lt;Ext: drop&gt;(_ext: Ext, self: &Kiosk): &Bag {
    <b>assert</b>!(<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_installed">is_installed</a>&lt;Ext&gt;(self), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotInstalled">EExtensionNotInstalled</a>);
    &<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension">extension</a>&lt;Ext&gt;(self).<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_storage">storage</a>
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_storage_mut"></a>

## Function `storage_mut`

Get mutable access to the extension storage. Can only be performed by
the extension as long as the extension is installed. Disabling the
extension does not prevent it from accessing the storage.

Potentially dangerous: extension developer can keep data in a Bag
therefore never really allowing the KioskOwner to remove the extension.
However, it is the case with any other solution (1) and this way we
prevent intentional extension freeze when the owner wants to ruin a
trade (2) - eg locking extension while an auction is in progress.

Extensions should be crafted carefully, and the KioskOwner should be
aware of the risks.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_storage_mut">storage_mut</a>&lt;Ext: drop&gt;(_ext: Ext, self: &<b>mut</b> <a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>): &<b>mut</b> <a href="../haneul/bag.md#haneul_bag_Bag">haneul::bag::Bag</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_storage_mut">storage_mut</a>&lt;Ext: drop&gt;(_ext: Ext, self: &<b>mut</b> Kiosk): &<b>mut</b> Bag {
    <b>assert</b>!(<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_installed">is_installed</a>&lt;Ext&gt;(self), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotInstalled">EExtensionNotInstalled</a>);
    &<b>mut</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension_mut">extension_mut</a>&lt;Ext&gt;(self).<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_storage">storage</a>
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_place"></a>

## Function `place`

Protected action: place an item into the Kiosk. Can be performed by an
authorized extension. The extension must have the <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code> permission or
a <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a></code> permission.

To prevent non-tradable items from being placed into <code>Kiosk</code> the method
requires a <code>TransferPolicy</code> for the placed type to exist.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a>&lt;Ext: drop, T: key, store&gt;(_ext: Ext, self: &<b>mut</b> <a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>, item: T, _policy: &<a href="../haneul/transfer_policy.md#haneul_transfer_policy_TransferPolicy">haneul::transfer_policy::TransferPolicy</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a>&lt;Ext: drop, T: key + store&gt;(
    _ext: Ext,
    self: &<b>mut</b> Kiosk,
    item: T,
    _policy: &TransferPolicy&lt;T&gt;,
) {
    <b>assert</b>!(<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_installed">is_installed</a>&lt;Ext&gt;(self), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotInstalled">EExtensionNotInstalled</a>);
    <b>assert</b>!(<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_can_place">can_place</a>&lt;Ext&gt;(self) || <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_can_lock">can_lock</a>&lt;Ext&gt;(self), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotAllowed">EExtensionNotAllowed</a>);
    self.place_internal(item)
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_lock"></a>

## Function `lock`

Protected action: lock an item in the Kiosk. Can be performed by an
authorized extension. The extension must have the <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a></code> permission.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a>&lt;Ext: drop, T: key, store&gt;(_ext: Ext, self: &<b>mut</b> <a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>, item: T, _policy: &<a href="../haneul/transfer_policy.md#haneul_transfer_policy_TransferPolicy">haneul::transfer_policy::TransferPolicy</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a>&lt;Ext: drop, T: key + store&gt;(
    _ext: Ext,
    self: &<b>mut</b> Kiosk,
    item: T,
    _policy: &TransferPolicy&lt;T&gt;,
) {
    <b>assert</b>!(<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_installed">is_installed</a>&lt;Ext&gt;(self), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotInstalled">EExtensionNotInstalled</a>);
    <b>assert</b>!(<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_can_lock">can_lock</a>&lt;Ext&gt;(self), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_EExtensionNotAllowed">EExtensionNotAllowed</a>);
    self.lock_internal(item)
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_is_installed"></a>

## Function `is_installed`

Check whether an extension of type <code>Ext</code> is installed.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_installed">is_installed</a>&lt;Ext: drop&gt;(self: &<a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_installed">is_installed</a>&lt;Ext: drop&gt;(self: &Kiosk): bool {
    df::exists_(self.uid(), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ExtensionKey">ExtensionKey</a>&lt;Ext&gt; {})
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_is_enabled"></a>

## Function `is_enabled`

Check whether an extension of type <code>Ext</code> is enabled.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_enabled">is_enabled</a>&lt;Ext: drop&gt;(self: &<a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_enabled">is_enabled</a>&lt;Ext: drop&gt;(self: &Kiosk): bool {
    <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension">extension</a>&lt;Ext&gt;(self).<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_enabled">is_enabled</a>
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_can_place"></a>

## Function `can_place`

Check whether an extension of type <code>Ext</code> can <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code> into Kiosk.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_can_place">can_place</a>&lt;Ext: drop&gt;(self: &<a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_can_place">can_place</a>&lt;Ext: drop&gt;(self: &Kiosk): bool {
    <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_enabled">is_enabled</a>&lt;Ext&gt;(self) && <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension">extension</a>&lt;Ext&gt;(self).permissions & <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_PLACE">PLACE</a> != 0
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_can_lock"></a>

## Function `can_lock`

Check whether an extension of type <code>Ext</code> can <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_lock">lock</a></code> items in Kiosk.
Locking also enables <code><a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_place">place</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_can_lock">can_lock</a>&lt;Ext: drop&gt;(self: &<a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_can_lock">can_lock</a>&lt;Ext: drop&gt;(self: &Kiosk): bool {
    <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_is_enabled">is_enabled</a>&lt;Ext&gt;(self) && <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension">extension</a>&lt;Ext&gt;(self).permissions & <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_LOCK">LOCK</a> != 0
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_extension"></a>

## Function `extension`

Internal: get a read-only access to the Extension.


<pre><code><b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension">extension</a>&lt;Ext: drop&gt;(self: &<a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>): &<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_Extension">haneul::kiosk_extension::Extension</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension">extension</a>&lt;Ext: drop&gt;(self: &Kiosk): &<a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_Extension">Extension</a> {
    df::borrow(self.uid(), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ExtensionKey">ExtensionKey</a>&lt;Ext&gt; {})
}
</code></pre>



</details>

<a name="haneul_kiosk_extension_extension_mut"></a>

## Function `extension_mut`

Internal: get a mutable access to the Extension.


<pre><code><b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension_mut">extension_mut</a>&lt;Ext: drop&gt;(self: &<b>mut</b> <a href="../haneul/kiosk.md#haneul_kiosk_Kiosk">haneul::kiosk::Kiosk</a>): &<b>mut</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_Extension">haneul::kiosk_extension::Extension</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_extension_mut">extension_mut</a>&lt;Ext: drop&gt;(self: &<b>mut</b> Kiosk): &<b>mut</b> <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_Extension">Extension</a> {
    df::borrow_mut(self.uid_mut_internal(), <a href="../haneul/kiosk_extension.md#haneul_kiosk_extension_ExtensionKey">ExtensionKey</a>&lt;Ext&gt; {})
}
</code></pre>



</details>
