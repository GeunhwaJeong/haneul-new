---
title: Module `haneul_system::voting_power`
---



-  [Struct `VotingPowerInfo`](#haneul_system_voting_power_VotingPowerInfo)
-  [Struct `VotingPowerInfoV2`](#haneul_system_voting_power_VotingPowerInfoV2)
-  [Constants](#@Constants_0)
-  [Function `set_voting_power`](#haneul_system_voting_power_set_voting_power)
-  [Function `init_voting_power_info`](#haneul_system_voting_power_init_voting_power_info)
-  [Function `derive_raw_voting_power`](#haneul_system_voting_power_derive_raw_voting_power)
-  [Function `insert`](#haneul_system_voting_power_insert)
-  [Function `adjust_voting_power`](#haneul_system_voting_power_adjust_voting_power)
-  [Function `update_voting_power`](#haneul_system_voting_power_update_voting_power)
-  [Function `check_invariants`](#haneul_system_voting_power_check_invariants)
-  [Function `total_voting_power`](#haneul_system_voting_power_total_voting_power)
-  [Function `quorum_threshold`](#haneul_system_voting_power_quorum_threshold)


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
<b>use</b> <a href="../haneul_system/staking_pool.md#haneul_system_staking_pool">haneul_system::staking_pool</a>;
<b>use</b> <a href="../haneul_system/validator.md#haneul_system_validator">haneul_system::validator</a>;
<b>use</b> <a href="../haneul_system/validator_cap.md#haneul_system_validator_cap">haneul_system::validator_cap</a>;
</code></pre>



<a name="haneul_system_voting_power_VotingPowerInfo"></a>

## Struct `VotingPowerInfo`

Deprecated. Use VotingPowerInfoV2 instead.


<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfo">VotingPowerInfo</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_index: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="haneul_system_voting_power_VotingPowerInfoV2"></a>

## Struct `VotingPowerInfoV2`



<pre><code><b>public</b> <b>struct</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">VotingPowerInfoV2</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator_index: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_system_voting_power_TOTAL_VOTING_POWER"></a>

Set total_voting_power as 10_000 by convention. Individual voting powers can be interpreted
as easily understandable basis points (e.g., voting_power: 100 = 1%, voting_power: 1 = 0.01%) rather than
opaque quantities whose meaning changes from epoch to epoch as the total amount staked shifts.
Fixing the total voting power allows clients to hardcode the quorum threshold and total_voting power rather
than recomputing these.


<pre><code><b>const</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>: u64 = 10000;
</code></pre>



<a name="haneul_system_voting_power_QUORUM_THRESHOLD"></a>

Quorum threshold for our fixed voting power--any message signed by this much voting power can be trusted
up to BFT assumptions


<pre><code><b>const</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_QUORUM_THRESHOLD">QUORUM_THRESHOLD</a>: u64 = 6667;
</code></pre>



<a name="haneul_system_voting_power_MAX_VOTING_POWER"></a>



<pre><code><b>const</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_MAX_VOTING_POWER">MAX_VOTING_POWER</a>: u64 = 1000;
</code></pre>



<a name="haneul_system_voting_power_ETotalPowerMismatch"></a>



<pre><code><b>const</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_ETotalPowerMismatch">ETotalPowerMismatch</a>: u64 = 1;
</code></pre>



<a name="haneul_system_voting_power_ERelativePowerMismatch"></a>



<pre><code><b>const</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_ERelativePowerMismatch">ERelativePowerMismatch</a>: u64 = 2;
</code></pre>



<a name="haneul_system_voting_power_EVotingPowerOverThreshold"></a>



<pre><code><b>const</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_EVotingPowerOverThreshold">EVotingPowerOverThreshold</a>: u64 = 3;
</code></pre>



<a name="haneul_system_voting_power_EInvalidVotingPower"></a>



<pre><code><b>const</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_EInvalidVotingPower">EInvalidVotingPower</a>: u64 = 4;
</code></pre>



<a name="haneul_system_voting_power_set_voting_power"></a>

## Function `set_voting_power`

Set the voting power of all validators.
Each validator's voting power is initialized using their stake. We then attempt to cap their voting power
at <code><a href="../haneul_system/voting_power.md#haneul_system_voting_power_MAX_VOTING_POWER">MAX_VOTING_POWER</a></code>. If <code><a href="../haneul_system/voting_power.md#haneul_system_voting_power_MAX_VOTING_POWER">MAX_VOTING_POWER</a></code> is not a feasible cap, we pick the lowest possible cap.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_set_voting_power">set_voting_power</a>(validators: &<b>mut</b> vector&lt;<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>&gt;, total_stake: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_set_voting_power">set_voting_power</a>(validators: &<b>mut</b> vector&lt;Validator&gt;, total_stake: u64) {
    // If threshold_pct is too small, it's possible that even when all validators reach the threshold we still don't
    // have 100%. So we bound the threshold_pct to be always enough to find a solution.
    <b>let</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_total_voting_power">total_voting_power</a> = <a href="../haneul_system/voting_power.md#haneul_system_voting_power_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>;
    <b>let</b> average_voting_power = <a href="../haneul_system/voting_power.md#haneul_system_voting_power_total_voting_power">total_voting_power</a>.divide_and_round_up(validators.length());
    <b>let</b> threshold = <a href="../haneul_system/voting_power.md#haneul_system_voting_power_total_voting_power">total_voting_power</a>.min(<a href="../haneul_system/voting_power.md#haneul_system_voting_power_MAX_VOTING_POWER">MAX_VOTING_POWER</a>.max(average_voting_power));
    <b>let</b> (<b>mut</b> info_list, remaining_power) = <a href="../haneul_system/voting_power.md#haneul_system_voting_power_init_voting_power_info">init_voting_power_info</a>(
        validators,
        threshold,
        total_stake,
    );
    <a href="../haneul_system/voting_power.md#haneul_system_voting_power_adjust_voting_power">adjust_voting_power</a>(&<b>mut</b> info_list, threshold, remaining_power);
    <a href="../haneul_system/voting_power.md#haneul_system_voting_power_update_voting_power">update_voting_power</a>(validators, info_list);
    <a href="../haneul_system/voting_power.md#haneul_system_voting_power_check_invariants">check_invariants</a>(validators);
}
</code></pre>



</details>

<a name="haneul_system_voting_power_init_voting_power_info"></a>

## Function `init_voting_power_info`

Create the initial voting power of each validator, set using their stake, but capped using threshold.
We also perform insertion sort while creating the voting power list, by maintaining the list in
descending order using voting power.
Anything beyond the threshold is added to the remaining_power, which is also returned.


<pre><code><b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_init_voting_power_info">init_voting_power_info</a>(validators: &vector&lt;<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>&gt;, threshold: u64, total_stake: u64): (vector&lt;<a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">haneul_system::voting_power::VotingPowerInfoV2</a>&gt;, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_init_voting_power_info">init_voting_power_info</a>(
    validators: &vector&lt;Validator&gt;,
    threshold: u64,
    total_stake: u64,
): (vector&lt;<a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">VotingPowerInfoV2</a>&gt;, u64) {
    <b>let</b> <b>mut</b> total_power = 0;
    <b>let</b> <b>mut</b> result = vector[];
    validators.length().do!(|i| {
        <b>let</b> stake = validators[i].total_stake();
        <b>let</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a> = <a href="../haneul_system/voting_power.md#haneul_system_voting_power_derive_raw_voting_power">derive_raw_voting_power</a>(stake, total_stake).min(threshold);
        <a href="../haneul_system/voting_power.md#haneul_system_voting_power_insert">insert</a>(&<b>mut</b> result, <a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">VotingPowerInfoV2</a> { validator_index: i, <a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>, stake });
        total_power = total_power + <a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>;
    });
    (result, <a href="../haneul_system/voting_power.md#haneul_system_voting_power_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a> - total_power)
}
</code></pre>



</details>

<a name="haneul_system_voting_power_derive_raw_voting_power"></a>

## Function `derive_raw_voting_power`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_derive_raw_voting_power">derive_raw_voting_power</a>(stake: u64, total_stake: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_derive_raw_voting_power">derive_raw_voting_power</a>(stake: u64, total_stake: u64): u64 {
    ((stake <b>as</b> u128 * (<a href="../haneul_system/voting_power.md#haneul_system_voting_power_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a> <b>as</b> u128) / (total_stake <b>as</b> u128)) <b>as</b> u64)
}
</code></pre>



</details>

<a name="haneul_system_voting_power_insert"></a>

## Function `insert`

Insert <code>new_info</code> to <code>info_list</code> as part of insertion sort, such that <code>info_list</code> is always sorted
using stake, in descending order.


<pre><code><b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_insert">insert</a>(info_list: &<b>mut</b> vector&lt;<a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">haneul_system::voting_power::VotingPowerInfoV2</a>&gt;, new_info: <a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">haneul_system::voting_power::VotingPowerInfoV2</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_insert">insert</a>(info_list: &<b>mut</b> vector&lt;<a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">VotingPowerInfoV2</a>&gt;, new_info: <a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">VotingPowerInfoV2</a>) {
    <b>let</b> len = info_list.length();
    <b>let</b> idx = info_list.find_index!(|info| new_info.stake &gt;= info.stake);
    info_list.<a href="../haneul_system/voting_power.md#haneul_system_voting_power_insert">insert</a>(new_info, idx.destroy_or!(len));
}
</code></pre>



</details>

<a name="haneul_system_voting_power_adjust_voting_power"></a>

## Function `adjust_voting_power`

Distribute remaining_power to validators that are not capped at threshold.


<pre><code><b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_adjust_voting_power">adjust_voting_power</a>(info_list: &<b>mut</b> vector&lt;<a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">haneul_system::voting_power::VotingPowerInfoV2</a>&gt;, threshold: u64, remaining_power: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_adjust_voting_power">adjust_voting_power</a>(
    info_list: &<b>mut</b> vector&lt;<a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">VotingPowerInfoV2</a>&gt;,
    threshold: u64,
    <b>mut</b> remaining_power: u64,
) {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> len = info_list.length();
    <b>while</b> (i &lt; len && remaining_power &gt; 0) {
        <b>let</b> v = &<b>mut</b> info_list[i];
        // planned is the amount of extra power we want to distribute to this <a href="../haneul_system/validator.md#haneul_system_validator">validator</a>.
        <b>let</b> planned = remaining_power.divide_and_round_up(len - i);
        // target is the targeting power this <a href="../haneul_system/validator.md#haneul_system_validator">validator</a> will reach, capped by threshold.
        <b>let</b> target = threshold.min(v.<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a> + planned);
        // actual is the actual amount of power we will be distributing to this <a href="../haneul_system/validator.md#haneul_system_validator">validator</a>.
        <b>let</b> actual = remaining_power.min(target - v.<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>);
        v.<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a> = v.<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a> + actual;
        <b>assert</b>!(v.<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a> &lt;= threshold, <a href="../haneul_system/voting_power.md#haneul_system_voting_power_EVotingPowerOverThreshold">EVotingPowerOverThreshold</a>);
        remaining_power = remaining_power - actual;
        i = i + 1;
    };
    <b>assert</b>!(remaining_power == 0, <a href="../haneul_system/voting_power.md#haneul_system_voting_power_ETotalPowerMismatch">ETotalPowerMismatch</a>);
}
</code></pre>



</details>

<a name="haneul_system_voting_power_update_voting_power"></a>

## Function `update_voting_power`

Update validators with the decided voting power.


<pre><code><b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_update_voting_power">update_voting_power</a>(validators: &<b>mut</b> vector&lt;<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>&gt;, info_list: vector&lt;<a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">haneul_system::voting_power::VotingPowerInfoV2</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_update_voting_power">update_voting_power</a>(validators: &<b>mut</b> vector&lt;Validator&gt;, info_list: vector&lt;<a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">VotingPowerInfoV2</a>&gt;) {
    info_list.destroy!(|<a href="../haneul_system/voting_power.md#haneul_system_voting_power_VotingPowerInfoV2">VotingPowerInfoV2</a> { validator_index, <a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>, .. }| {
        validators[validator_index].<a href="../haneul_system/voting_power.md#haneul_system_voting_power_set_voting_power">set_voting_power</a>(<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>);
    });
}
</code></pre>



</details>

<a name="haneul_system_voting_power_check_invariants"></a>

## Function `check_invariants`

Check a few invariants that must hold after setting the voting power.


<pre><code><b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_check_invariants">check_invariants</a>(v: &vector&lt;<a href="../haneul_system/validator.md#haneul_system_validator_Validator">haneul_system::validator::Validator</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_check_invariants">check_invariants</a>(v: &vector&lt;Validator&gt;) {
    <b>let</b> <b>mut</b> total = 0;
    v.do_ref!(|v| {
        <b>let</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a> = v.<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>();
        <b>assert</b>!(<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a> &gt; 0, <a href="../haneul_system/voting_power.md#haneul_system_voting_power_EInvalidVotingPower">EInvalidVotingPower</a>);
        total = total + <a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>;
    });
    <b>assert</b>!(total == <a href="../haneul_system/voting_power.md#haneul_system_voting_power_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>, <a href="../haneul_system/voting_power.md#haneul_system_voting_power_ETotalPowerMismatch">ETotalPowerMismatch</a>);
    // Second check that <b>if</b> <a href="../haneul_system/validator.md#haneul_system_validator">validator</a> A's stake is larger than B's stake, A's
    // voting power must be no less than B's voting power; similarly, <b>if</b> A's
    // stake is less than B's stake, A's voting power must be no larger than
    // B's voting power.
    <b>let</b> length = v.length();
    length.do!(|a| {
        (a + 1).range_do!(length, |b| {
            <b>let</b> validator_a = &v[a];
            <b>let</b> validator_b = &v[b];
            <b>let</b> stake_a = validator_a.total_stake();
            <b>let</b> stake_b = validator_b.total_stake();
            <b>let</b> power_a = validator_a.<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>();
            <b>let</b> power_b = validator_b.<a href="../haneul_system/voting_power.md#haneul_system_voting_power">voting_power</a>();
            <b>if</b> (stake_a &gt; stake_b) {
                <b>assert</b>!(power_a &gt;= power_b, <a href="../haneul_system/voting_power.md#haneul_system_voting_power_ERelativePowerMismatch">ERelativePowerMismatch</a>);
            };
            <b>if</b> (stake_a &lt; stake_b) {
                <b>assert</b>!(power_a &lt;= power_b, <a href="../haneul_system/voting_power.md#haneul_system_voting_power_ERelativePowerMismatch">ERelativePowerMismatch</a>);
            };
        })
    });
}
</code></pre>



</details>

<a name="haneul_system_voting_power_total_voting_power"></a>

## Function `total_voting_power`

Return the (constant) total voting power


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_total_voting_power">total_voting_power</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_total_voting_power">total_voting_power</a>(): u64 {
    <a href="../haneul_system/voting_power.md#haneul_system_voting_power_TOTAL_VOTING_POWER">TOTAL_VOTING_POWER</a>
}
</code></pre>



</details>

<a name="haneul_system_voting_power_quorum_threshold"></a>

## Function `quorum_threshold`

Return the (constant) quorum threshold


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_quorum_threshold">quorum_threshold</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul_system/voting_power.md#haneul_system_voting_power_quorum_threshold">quorum_threshold</a>(): u64 {
    <a href="../haneul_system/voting_power.md#haneul_system_voting_power_QUORUM_THRESHOLD">QUORUM_THRESHOLD</a>
}
</code></pre>



</details>
