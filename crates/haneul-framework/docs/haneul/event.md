---
title: Module `haneul::event`
---

Events module. Defines the <code><a href="../haneul/event.md#haneul_event_emit">haneul::event::emit</a></code> function which
creates and sends a custom MoveEvent as a part of the effects
certificate of the transaction.

Every MoveEvent has the following properties:
- sender
- type signature (<code>T</code>)
- event data (the value of <code>T</code>)
- timestamp (local to a node)
- transaction digest

Example:
```
module my::marketplace {
use haneul::event;
/* ... */
struct ItemPurchased has copy, drop {
item_id: ID, buyer: address
}
entry fun buy(/* .... */) {
/* ... */
event::emit(ItemPurchased { item_id: ..., buyer: .... })
}
}
```


-  [Function `emit`](#haneul_event_emit)
-  [Function `emit_authenticated`](#haneul_event_emit_authenticated)
-  [Function `emit_authenticated_impl`](#haneul_event_emit_authenticated_impl)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../haneul/accumulator.md#haneul_accumulator">haneul::accumulator</a>;
<b>use</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement">haneul::accumulator_settlement</a>;
<b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/bcs.md#haneul_bcs">haneul::bcs</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/hash.md#haneul_hash">haneul::hash</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
</code></pre>



<a name="haneul_event_emit"></a>

## Function `emit`

Emit a custom Move event, sending the data offchain.

Used for creating custom indexes and tracking onchain
activity in a way that haneults a specific application the most.

The type <code>T</code> is the main way to index the event, and can contain
phantom parameters, eg <code><a href="../haneul/event.md#haneul_event_emit">emit</a>(MyEvent&lt;<b>phantom</b> T&gt;)</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/event.md#haneul_event_emit">emit</a>&lt;T: <b>copy</b>, drop&gt;(<a href="../haneul/event.md#haneul_event">event</a>: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../haneul/event.md#haneul_event_emit">emit</a>&lt;T: <b>copy</b> + drop&gt;(<a href="../haneul/event.md#haneul_event">event</a>: T);
</code></pre>



</details>

<a name="haneul_event_emit_authenticated"></a>

## Function `emit_authenticated`

Emits a custom Move event which can be authenticated by a light client.

This method emits the authenticated event to the event stream for the Move package that
defines the event type <code>T</code>.
Only the package that defines the type <code>T</code> can emit authenticated events to this stream.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/event.md#haneul_event_emit_authenticated">emit_authenticated</a>&lt;T: <b>copy</b>, drop&gt;(<a href="../haneul/event.md#haneul_event">event</a>: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/event.md#haneul_event_emit_authenticated">emit_authenticated</a>&lt;T: <b>copy</b> + drop&gt;(<a href="../haneul/event.md#haneul_event">event</a>: T) {
    <b>let</b> stream_id = type_name::original_id&lt;T&gt;();
    <b>let</b> accumulator_addr = <a href="../haneul/accumulator.md#haneul_accumulator_accumulator_address">accumulator::accumulator_address</a>&lt;EventStreamHead&gt;(stream_id);
    <a href="../haneul/event.md#haneul_event_emit_authenticated_impl">emit_authenticated_impl</a>&lt;EventStreamHead, T&gt;(accumulator_addr, stream_id, <a href="../haneul/event.md#haneul_event">event</a>);
}
</code></pre>



</details>

<a name="haneul_event_emit_authenticated_impl"></a>

## Function `emit_authenticated_impl`



<pre><code><b>fun</b> <a href="../haneul/event.md#haneul_event_emit_authenticated_impl">emit_authenticated_impl</a>&lt;StreamHeadT, T: <b>copy</b>, drop&gt;(accumulator_id: <b>address</b>, stream: <b>address</b>, <a href="../haneul/event.md#haneul_event">event</a>: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="../haneul/event.md#haneul_event_emit_authenticated_impl">emit_authenticated_impl</a>&lt;StreamHeadT, T: <b>copy</b> + drop&gt;(
    accumulator_id: <b>address</b>,
    stream: <b>address</b>,
    <a href="../haneul/event.md#haneul_event">event</a>: T,
);
</code></pre>



</details>
