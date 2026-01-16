---
title: Module `bridge::committee`
---



-  [Struct `BlocklistValidatorEvent`](#bridge_committee_BlocklistValidatorEvent)
-  [Struct `BridgeCommittee`](#bridge_committee_BridgeCommittee)
-  [Struct `CommitteeUpdateEvent`](#bridge_committee_CommitteeUpdateEvent)
-  [Struct `CommitteeMemberUrlUpdateEvent`](#bridge_committee_CommitteeMemberUrlUpdateEvent)
-  [Struct `CommitteeMember`](#bridge_committee_CommitteeMember)
-  [Struct `CommitteeMemberRegistration`](#bridge_committee_CommitteeMemberRegistration)
-  [Constants](#@Constants_0)
-  [Function `verify_signatures`](#bridge_committee_verify_signatures)
-  [Function `create`](#bridge_committee_create)
-  [Function `register`](#bridge_committee_register)
-  [Function `try_create_next_committee`](#bridge_committee_try_create_next_committee)
-  [Function `execute_blocklist`](#bridge_committee_execute_blocklist)
-  [Function `committee_members`](#bridge_committee_committee_members)
-  [Function `update_node_url`](#bridge_committee_update_node_url)
-  [Function `check_uniqueness_bridge_keys`](#bridge_committee_check_uniqueness_bridge_keys)


<pre><code><b>use</b> <a href="../bridge/chain_ids.md#bridge_chain_ids">bridge::chain_ids</a>;
<b>use</b> <a href="../bridge/crypto.md#bridge_crypto">bridge::crypto</a>;
<b>use</b> <a href="../bridge/message.md#bridge_message">bridge::message</a>;
<b>use</b> <a href="../bridge/message_types.md#bridge_message_types">bridge::message_types</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
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
<b>use</b> <a href="../haneul/ecdsa_k1.md#haneul_ecdsa_k1">haneul::ecdsa_k1</a>;
<b>use</b> <a href="../haneul/event.md#haneul_event">haneul::event</a>;
<b>use</b> <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator">haneul::funds_accumulator</a>;
<b>use</b> <a href="../haneul/hash.md#haneul_hash">haneul::hash</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/priority_queue.md#haneul_priority_queue">haneul::priority_queue</a>;
<b>use</b> <a href="../haneul/protocol_config.md#haneul_protocol_config">haneul::protocol_config</a>;
<b>use</b> <a href="../haneul/haneul.md#haneul_haneul">haneul::haneul</a>;
<b>use</b> <a href="../haneul/table.md#haneul_table">haneul::table</a>;
<b>use</b> <a href="../haneul/table_vec.md#haneul_table_vec">haneul::table_vec</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/types.md#haneul_types">haneul::types</a>;
<b>use</b> <a href="../haneul/url.md#haneul_url">haneul::url</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../haneul/vec_set.md#haneul_vec_set">haneul::vec_set</a>;
<b>use</b> <a href="../haneul/versioned.md#haneul_versioned">haneul::versioned</a>;
<b>use</b> <a href="../haneul_system/stake_subsidy.md#haneul_system_stake_subsidy">haneul_system::stake_subsidy</a>;
<b>use</b> <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">haneul_system::staking_pool</a>;
<b>use</b> <a href="../haneul_system/storage_fund.md#haneul_system_storage_fund">haneul_system::storage_fund</a>;
<b>use</b> <a href="../haneul_system/haneul_system.md#haneul_system_haneul_system">haneul_system::haneul_system</a>;
<b>use</b> <a href="../haneul_system/haneul_system_state_inner.md#haneul_system_haneul_system_state_inner">haneul_system::haneul_system_state_inner</a>;
<b>use</b> <a href="../haneul_system/validator.md#haneul_system_validator">haneul_system::validator</a>;
<b>use</b> <a href="../haneul_system/validator_cap.md#haneul_system_validator_cap">haneul_system::validator_cap</a>;
<b>use</b> <a href="../haneul_system/validator_set.md#haneul_system_validator_set">haneul_system::validator_set</a>;
<b>use</b> <a href="../haneul_system/validator_wrapper.md#haneul_system_validator_wrapper">haneul_system::validator_wrapper</a>;
<b>use</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power">haneul_system::voting_power</a>;
</code></pre>



<a name="bridge_committee_BlocklistValidatorEvent"></a>

## Struct `BlocklistValidatorEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/committee.md#bridge_committee_BlocklistValidatorEvent">BlocklistValidatorEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>blocklisted: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>public_keys: vector&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_committee_BridgeCommittee"></a>

## Struct `BridgeCommittee`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">BridgeCommittee</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>members: <a href="../haneul/vec_map.md#haneul_vec_map_VecMap">haneul::vec_map::VecMap</a>&lt;vector&lt;u8&gt;, <a href="../bridge/committee.md#bridge_committee_CommitteeMember">bridge::committee::CommitteeMember</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>member_registrations: <a href="../haneul/vec_map.md#haneul_vec_map_VecMap">haneul::vec_map::VecMap</a>&lt;<b>address</b>, <a href="../bridge/committee.md#bridge_committee_CommitteeMemberRegistration">bridge::committee::CommitteeMemberRegistration</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>last_committee_update_epoch: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_committee_CommitteeUpdateEvent"></a>

## Struct `CommitteeUpdateEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/committee.md#bridge_committee_CommitteeUpdateEvent">CommitteeUpdateEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>members: <a href="../haneul/vec_map.md#haneul_vec_map_VecMap">haneul::vec_map::VecMap</a>&lt;vector&lt;u8&gt;, <a href="../bridge/committee.md#bridge_committee_CommitteeMember">bridge::committee::CommitteeMember</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_participation_percentage: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_committee_CommitteeMemberUrlUpdateEvent"></a>

## Struct `CommitteeMemberUrlUpdateEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/committee.md#bridge_committee_CommitteeMemberUrlUpdateEvent">CommitteeMemberUrlUpdateEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>member: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>new_url: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_committee_CommitteeMember"></a>

## Struct `CommitteeMember`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/committee.md#bridge_committee_CommitteeMember">CommitteeMember</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>haneul_address: <b>address</b></code>
</dt>
<dd>
 The Haneul Address of the validator
</dd>
<dt>
<code>bridge_pubkey_bytes: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes of the bridge key
</dd>
<dt>
<code>voting_power: u64</code>
</dt>
<dd>
 Voting power, values are voting power in the scale of 10000.
</dd>
<dt>
<code>http_rest_url: vector&lt;u8&gt;</code>
</dt>
<dd>
 The HTTP REST URL the member's node listens to
 it looks like b'https://127.0.0.1:9191'
</dd>
<dt>
<code>blocklisted: bool</code>
</dt>
<dd>
 If this member is blocklisted
</dd>
</dl>


</details>

<a name="bridge_committee_CommitteeMemberRegistration"></a>

## Struct `CommitteeMemberRegistration`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/committee.md#bridge_committee_CommitteeMemberRegistration">CommitteeMemberRegistration</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>haneul_address: <b>address</b></code>
</dt>
<dd>
 The Haneul Address of the validator
</dd>
<dt>
<code>bridge_pubkey_bytes: vector&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes of the bridge key
</dd>
<dt>
<code>http_rest_url: vector&lt;u8&gt;</code>
</dt>
<dd>
 The HTTP REST URL the member's node listens to
 it looks like b'https://127.0.0.1:9191'
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_committee_ESignatureBelowThreshold"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_ESignatureBelowThreshold">ESignatureBelowThreshold</a>: u64 = 0;
</code></pre>



<a name="bridge_committee_EDuplicatedSignature"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_EDuplicatedSignature">EDuplicatedSignature</a>: u64 = 1;
</code></pre>



<a name="bridge_committee_EInvalidSignature"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_EInvalidSignature">EInvalidSignature</a>: u64 = 2;
</code></pre>



<a name="bridge_committee_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_ENotSystemAddress">ENotSystemAddress</a>: u64 = 3;
</code></pre>



<a name="bridge_committee_EValidatorBlocklistContainsUnknownKey"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_EValidatorBlocklistContainsUnknownKey">EValidatorBlocklistContainsUnknownKey</a>: u64 = 4;
</code></pre>



<a name="bridge_committee_ESenderNotActiveValidator"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_ESenderNotActiveValidator">ESenderNotActiveValidator</a>: u64 = 5;
</code></pre>



<a name="bridge_committee_EInvalidPubkeyLength"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_EInvalidPubkeyLength">EInvalidPubkeyLength</a>: u64 = 6;
</code></pre>



<a name="bridge_committee_ECommitteeAlreadyInitiated"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_ECommitteeAlreadyInitiated">ECommitteeAlreadyInitiated</a>: u64 = 7;
</code></pre>



<a name="bridge_committee_EDuplicatePubkey"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_EDuplicatePubkey">EDuplicatePubkey</a>: u64 = 8;
</code></pre>



<a name="bridge_committee_ESenderIsNotInBridgeCommittee"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_ESenderIsNotInBridgeCommittee">ESenderIsNotInBridgeCommittee</a>: u64 = 9;
</code></pre>



<a name="bridge_committee_HANEUL_MESSAGE_PREFIX"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_HANEUL_MESSAGE_PREFIX">HANEUL_MESSAGE_PREFIX</a>: vector&lt;u8&gt; = vector[83, 85, 73, 95, 66, 82, 73, 68, 71, 69, 95, 77, 69, 83, 83, 65, 71, 69];
</code></pre>



<a name="bridge_committee_ECDSA_COMPRESSED_PUBKEY_LENGTH"></a>



<pre><code><b>const</b> <a href="../bridge/committee.md#bridge_committee_ECDSA_COMPRESSED_PUBKEY_LENGTH">ECDSA_COMPRESSED_PUBKEY_LENGTH</a>: u64 = 33;
</code></pre>



<a name="bridge_committee_verify_signatures"></a>

## Function `verify_signatures`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/committee.md#bridge_committee_verify_signatures">verify_signatures</a>(self: &<a href="../bridge/committee.md#bridge_committee_BridgeCommittee">bridge::committee::BridgeCommittee</a>, <a href="../bridge/message.md#bridge_message">message</a>: <a href="../bridge/message.md#bridge_message_BridgeMessage">bridge::message::BridgeMessage</a>, signatures: vector&lt;vector&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/committee.md#bridge_committee_verify_signatures">verify_signatures</a>(
    self: &<a href="../bridge/committee.md#bridge_committee_BridgeCommittee">BridgeCommittee</a>,
    <a href="../bridge/message.md#bridge_message">message</a>: BridgeMessage,
    signatures: vector&lt;vector&lt;u8&gt;&gt;,
) {
    <b>let</b> (<b>mut</b> i, signature_counts) = (0, vector::length(&signatures));
    <b>let</b> <b>mut</b> seen_pub_key = vec_set::empty&lt;vector&lt;u8&gt;&gt;();
    <b>let</b> required_voting_power = <a href="../bridge/message.md#bridge_message">message</a>.required_voting_power();
    // add prefix to the <a href="../bridge/message.md#bridge_message">message</a> bytes
    <b>let</b> <b>mut</b> message_bytes = <a href="../bridge/committee.md#bridge_committee_HANEUL_MESSAGE_PREFIX">HANEUL_MESSAGE_PREFIX</a>;
    message_bytes.append(<a href="../bridge/message.md#bridge_message">message</a>.serialize_message());
    <b>let</b> <b>mut</b> threshold = 0;
    <b>while</b> (i &lt; signature_counts) {
        <b>let</b> pubkey = ecdsa_k1::secp256k1_ecrecover(&signatures[i], &message_bytes, 0);
        // check duplicate
        // and make sure pub key is part of the <a href="../bridge/committee.md#bridge_committee">committee</a>
        <b>assert</b>!(!seen_pub_key.contains(&pubkey), <a href="../bridge/committee.md#bridge_committee_EDuplicatedSignature">EDuplicatedSignature</a>);
        <b>assert</b>!(self.members.contains(&pubkey), <a href="../bridge/committee.md#bridge_committee_EInvalidSignature">EInvalidSignature</a>);
        // get <a href="../bridge/committee.md#bridge_committee">committee</a> signature weight and check pubkey is part of the <a href="../bridge/committee.md#bridge_committee">committee</a>
        <b>let</b> member = &self.members[&pubkey];
        <b>if</b> (!member.blocklisted) {
            threshold = threshold + member.voting_power;
        };
        seen_pub_key.insert(pubkey);
        i = i + 1;
    };
    <b>assert</b>!(threshold &gt;= required_voting_power, <a href="../bridge/committee.md#bridge_committee_ESignatureBelowThreshold">ESignatureBelowThreshold</a>);
}
</code></pre>



</details>

<a name="bridge_committee_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_create">create</a>(ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>): <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">bridge::committee::BridgeCommittee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_create">create</a>(ctx: &TxContext): <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">BridgeCommittee</a> {
    <b>assert</b>!(tx_context::sender(ctx) == @0x0, <a href="../bridge/committee.md#bridge_committee_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">BridgeCommittee</a> {
        members: vec_map::empty(),
        member_registrations: vec_map::empty(),
        last_committee_update_epoch: 0,
    }
}
</code></pre>



</details>

<a name="bridge_committee_register"></a>

## Function `register`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_register">register</a>(self: &<b>mut</b> <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">bridge::committee::BridgeCommittee</a>, system_state: &<b>mut</b> <a href="../haneul_system/haneul_system.md#haneul_system_haneul_system_HaneulSystemState">haneul_system::haneul_system::HaneulSystemState</a>, bridge_pubkey_bytes: vector&lt;u8&gt;, http_rest_url: vector&lt;u8&gt;, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_register">register</a>(
    self: &<b>mut</b> <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">BridgeCommittee</a>,
    system_state: &<b>mut</b> HaneulSystemState,
    bridge_pubkey_bytes: vector&lt;u8&gt;,
    http_rest_url: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    // We disallow registration after <a href="../bridge/committee.md#bridge_committee">committee</a> initiated in v1
    <b>assert</b>!(self.members.is_empty(), <a href="../bridge/committee.md#bridge_committee_ECommitteeAlreadyInitiated">ECommitteeAlreadyInitiated</a>);
    // Ensure pubkey is valid
    <b>assert</b>!(bridge_pubkey_bytes.length() == <a href="../bridge/committee.md#bridge_committee_ECDSA_COMPRESSED_PUBKEY_LENGTH">ECDSA_COMPRESSED_PUBKEY_LENGTH</a>, <a href="../bridge/committee.md#bridge_committee_EInvalidPubkeyLength">EInvalidPubkeyLength</a>);
    // sender must be the same sender that created the validator object, this is to prevent DDoS from non-validator actor.
    <b>let</b> sender = ctx.sender();
    <b>let</b> validators = system_state.active_validator_addresses();
    <b>assert</b>!(validators.contains(&sender), <a href="../bridge/committee.md#bridge_committee_ESenderNotActiveValidator">ESenderNotActiveValidator</a>);
    // Sender is active validator, record the registration
    // In case validator need to update the info
    <b>let</b> registration = <b>if</b> (self.member_registrations.contains(&sender)) {
        <b>let</b> registration = &<b>mut</b> self.member_registrations[&sender];
        registration.http_rest_url = http_rest_url;
        registration.bridge_pubkey_bytes = bridge_pubkey_bytes;
        *registration
    } <b>else</b> {
        <b>let</b> registration = <a href="../bridge/committee.md#bridge_committee_CommitteeMemberRegistration">CommitteeMemberRegistration</a> {
            haneul_address: sender,
            bridge_pubkey_bytes,
            http_rest_url,
        };
        self.member_registrations.insert(sender, registration);
        registration
    };
    // check uniqueness of the <a href="../bridge/bridge.md#bridge_bridge">bridge</a> pubkey.
    // `<a href="../bridge/committee.md#bridge_committee_try_create_next_committee">try_create_next_committee</a>` will <b>abort</b> <b>if</b> bridge_pubkey_bytes are not unique and
    // that will fail the end of epoch transaction (possibly "forever", well, we
    // need to deploy proper validator changes to stop end of epoch from failing).
    <a href="../bridge/committee.md#bridge_committee_check_uniqueness_bridge_keys">check_uniqueness_bridge_keys</a>(self, bridge_pubkey_bytes);
    emit(registration)
}
</code></pre>



</details>

<a name="bridge_committee_try_create_next_committee"></a>

## Function `try_create_next_committee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_try_create_next_committee">try_create_next_committee</a>(self: &<b>mut</b> <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">bridge::committee::BridgeCommittee</a>, active_validator_voting_power: <a href="../haneul/vec_map.md#haneul_vec_map_VecMap">haneul::vec_map::VecMap</a>&lt;<b>address</b>, u64&gt;, min_stake_participation_percentage: u64, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_try_create_next_committee">try_create_next_committee</a>(
    self: &<b>mut</b> <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">BridgeCommittee</a>,
    active_validator_voting_power: VecMap&lt;<b>address</b>, u64&gt;,
    min_stake_participation_percentage: u64,
    ctx: &TxContext,
) {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> new_members = vec_map::empty();
    <b>let</b> <b>mut</b> stake_participation_percentage = 0;
    <b>while</b> (i &lt; self.member_registrations.length()) {
        // retrieve registration
        <b>let</b> (_, registration) = self.member_registrations.get_entry_by_idx(i);
        // Find validator stake amount from system state
        // Process registration <b>if</b> it's active validator
        <b>let</b> voting_power = active_validator_voting_power.try_get(&registration.haneul_address);
        <b>if</b> (voting_power.is_some()) {
            <b>let</b> voting_power = voting_power.destroy_some();
            stake_participation_percentage = stake_participation_percentage + voting_power;
            <b>let</b> member = <a href="../bridge/committee.md#bridge_committee_CommitteeMember">CommitteeMember</a> {
                haneul_address: registration.haneul_address,
                bridge_pubkey_bytes: registration.bridge_pubkey_bytes,
                voting_power: (voting_power <b>as</b> u64),
                http_rest_url: registration.http_rest_url,
                blocklisted: <b>false</b>,
            };
            new_members.insert(registration.bridge_pubkey_bytes, member)
        };
        i = i + 1;
    };
    // Make sure the new <a href="../bridge/committee.md#bridge_committee">committee</a> represent enough stakes, percentage are accurate to 2DP
    <b>if</b> (stake_participation_percentage &gt;= min_stake_participation_percentage) {
        // Clear registrations
        self.member_registrations = vec_map::empty();
        // Store new <a href="../bridge/committee.md#bridge_committee">committee</a> info
        self.members = new_members;
        self.last_committee_update_epoch = ctx.epoch();
        emit(<a href="../bridge/committee.md#bridge_committee_CommitteeUpdateEvent">CommitteeUpdateEvent</a> {
            members: new_members,
            stake_participation_percentage,
        })
    }
}
</code></pre>



</details>

<a name="bridge_committee_execute_blocklist"></a>

## Function `execute_blocklist`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_execute_blocklist">execute_blocklist</a>(self: &<b>mut</b> <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">bridge::committee::BridgeCommittee</a>, blocklist: <a href="../bridge/message.md#bridge_message_Blocklist">bridge::message::Blocklist</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_execute_blocklist">execute_blocklist</a>(self: &<b>mut</b> <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">BridgeCommittee</a>, blocklist: Blocklist) {
    <b>let</b> blocklisted = blocklist.blocklist_type() != 1;
    <b>let</b> eth_addresses = blocklist.blocklist_validator_addresses();
    <b>let</b> list_len = eth_addresses.length();
    <b>let</b> <b>mut</b> list_idx = 0;
    <b>let</b> <b>mut</b> member_idx = 0;
    <b>let</b> <b>mut</b> pub_keys = vector[];
    <b>while</b> (list_idx &lt; list_len) {
        <b>let</b> target_address = &eth_addresses[list_idx];
        <b>let</b> <b>mut</b> found = <b>false</b>;
        <b>while</b> (member_idx &lt; self.members.length()) {
            <b>let</b> (pub_key, member) = self.members.get_entry_by_idx_mut(member_idx);
            <b>let</b> eth_address = <a href="../bridge/crypto.md#bridge_crypto_ecdsa_pub_key_to_eth_address">crypto::ecdsa_pub_key_to_eth_address</a>(pub_key);
            <b>if</b> (*target_address == eth_address) {
                member.blocklisted = blocklisted;
                pub_keys.push_back(*pub_key);
                found = <b>true</b>;
                member_idx = 0;
                <b>break</b>
            };
            member_idx = member_idx + 1;
        };
        <b>assert</b>!(found, <a href="../bridge/committee.md#bridge_committee_EValidatorBlocklistContainsUnknownKey">EValidatorBlocklistContainsUnknownKey</a>);
        list_idx = list_idx + 1;
    };
    emit(<a href="../bridge/committee.md#bridge_committee_BlocklistValidatorEvent">BlocklistValidatorEvent</a> {
        blocklisted,
        public_keys: pub_keys,
    })
}
</code></pre>



</details>

<a name="bridge_committee_committee_members"></a>

## Function `committee_members`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_committee_members">committee_members</a>(self: &<a href="../bridge/committee.md#bridge_committee_BridgeCommittee">bridge::committee::BridgeCommittee</a>): &<a href="../haneul/vec_map.md#haneul_vec_map_VecMap">haneul::vec_map::VecMap</a>&lt;vector&lt;u8&gt;, <a href="../bridge/committee.md#bridge_committee_CommitteeMember">bridge::committee::CommitteeMember</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_committee_members">committee_members</a>(
    self: &<a href="../bridge/committee.md#bridge_committee_BridgeCommittee">BridgeCommittee</a>,
): &VecMap&lt;vector&lt;u8&gt;, <a href="../bridge/committee.md#bridge_committee_CommitteeMember">CommitteeMember</a>&gt; {
    &self.members
}
</code></pre>



</details>

<a name="bridge_committee_update_node_url"></a>

## Function `update_node_url`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_update_node_url">update_node_url</a>(self: &<b>mut</b> <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">bridge::committee::BridgeCommittee</a>, new_url: vector&lt;u8&gt;, ctx: &<a href="../haneul/tx_context.md#haneul_tx_context_TxContext">haneul::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/committee.md#bridge_committee_update_node_url">update_node_url</a>(
    self: &<b>mut</b> <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">BridgeCommittee</a>,
    new_url: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> <b>mut</b> idx = 0;
    <b>while</b> (idx &lt; self.members.length()) {
        <b>let</b> (_, member) = self.members.get_entry_by_idx_mut(idx);
        <b>if</b> (member.haneul_address == ctx.sender()) {
            member.http_rest_url = new_url;
            emit(<a href="../bridge/committee.md#bridge_committee_CommitteeMemberUrlUpdateEvent">CommitteeMemberUrlUpdateEvent</a> {
                member: member.bridge_pubkey_bytes,
                new_url,
            });
            <b>return</b>
        };
        idx = idx + 1;
    };
    <b>abort</b> <a href="../bridge/committee.md#bridge_committee_ESenderIsNotInBridgeCommittee">ESenderIsNotInBridgeCommittee</a>
}
</code></pre>



</details>

<a name="bridge_committee_check_uniqueness_bridge_keys"></a>

## Function `check_uniqueness_bridge_keys`



<pre><code><b>fun</b> <a href="../bridge/committee.md#bridge_committee_check_uniqueness_bridge_keys">check_uniqueness_bridge_keys</a>(self: &<a href="../bridge/committee.md#bridge_committee_BridgeCommittee">bridge::committee::BridgeCommittee</a>, bridge_pubkey_bytes: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/committee.md#bridge_committee_check_uniqueness_bridge_keys">check_uniqueness_bridge_keys</a>(self: &<a href="../bridge/committee.md#bridge_committee_BridgeCommittee">BridgeCommittee</a>, bridge_pubkey_bytes: vector&lt;u8&gt;) {
    <b>let</b> <b>mut</b> count = self.member_registrations.length();
    // bridge_pubkey_bytes must be found once and once only
    <b>let</b> <b>mut</b> bridge_key_found = <b>false</b>;
    <b>while</b> (count &gt; 0) {
        count = count - 1;
        <b>let</b> (_, registration) = self.member_registrations.get_entry_by_idx(count);
        <b>if</b> (registration.bridge_pubkey_bytes == bridge_pubkey_bytes) {
            <b>assert</b>!(!bridge_key_found, <a href="../bridge/committee.md#bridge_committee_EDuplicatePubkey">EDuplicatePubkey</a>);
            bridge_key_found = <b>true</b>; // bridge_pubkey_bytes found, we must not have another one
        }
    };
}
</code></pre>



</details>
