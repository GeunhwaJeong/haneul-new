---
title: Module `deepbook::order_query`
---



-  [Struct `OrderPage`](#deepbook_order_query_OrderPage)
-  [Constants](#@Constants_0)
-  [Function `iter_bids`](#deepbook_order_query_iter_bids)
-  [Function `iter_asks`](#deepbook_order_query_iter_asks)
-  [Function `iter_ticks_internal`](#deepbook_order_query_iter_ticks_internal)
-  [Function `orders`](#deepbook_order_query_orders)
-  [Function `has_next_page`](#deepbook_order_query_has_next_page)
-  [Function `next_tick_level`](#deepbook_order_query_next_tick_level)
-  [Function `next_order_id`](#deepbook_order_query_next_order_id)
-  [Function `order_id`](#deepbook_order_query_order_id)
-  [Function `tick_level`](#deepbook_order_query_tick_level)


<pre><code><b>use</b> <a href="../deepbook/clob_v2.md#deepbook_clob_v2">deepbook::clob_v2</a>;
<b>use</b> <a href="../deepbook/critbit.md#deepbook_critbit">deepbook::critbit</a>;
<b>use</b> <a href="../deepbook/custodian_v2.md#deepbook_custodian_v2">deepbook::custodian_v2</a>;
<b>use</b> <a href="../deepbook/math.md#deepbook_math">deepbook::math</a>;
<b>use</b> <a href="../haneul/accumulator.md#haneul_accumulator">haneul::accumulator</a>;
<b>use</b> <a href="../haneul/accumulator_settlement.md#haneul_accumulator_settlement">haneul::accumulator_settlement</a>;
<b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/bag.md#haneul_bag">haneul::bag</a>;
<b>use</b> <a href="../haneul/balance.md#haneul_balance">haneul::balance</a>;
<b>use</b> <a href="../haneul/bcs.md#haneul_bcs">haneul::bcs</a>;
<b>use</b> <a href="../haneul/clock.md#haneul_clock">haneul::clock</a>;
<b>use</b> <a href="../haneul/coin.md#haneul_coin">haneul::coin</a>;
<b>use</b> <a href="../haneul/config.md#haneul_config">haneul::config</a>;
<b>use</b> <a href="../haneul/deny_list.md#haneul_deny_list">haneul::deny_list</a>;
<b>use</b> <a href="../haneul/dynamic_field.md#haneul_dynamic_field">haneul::dynamic_field</a>;
<b>use</b> <a href="../haneul/dynamic_object_field.md#haneul_dynamic_object_field">haneul::dynamic_object_field</a>;
<b>use</b> <a href="../haneul/event.md#haneul_event">haneul::event</a>;
<b>use</b> <a href="../haneul/funds_accumulator.md#haneul_funds_accumulator">haneul::funds_accumulator</a>;
<b>use</b> <a href="../haneul/haneul.md#haneul_haneul">haneul::haneul</a>;
<b>use</b> <a href="../haneul/hash.md#haneul_hash">haneul::hash</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../haneul/linked_table.md#haneul_linked_table">haneul::linked_table</a>;
<b>use</b> <a href="../haneul/object.md#haneul_object">haneul::object</a>;
<b>use</b> <a href="../haneul/party.md#haneul_party">haneul::party</a>;
<b>use</b> <a href="../haneul/protocol_config.md#haneul_protocol_config">haneul::protocol_config</a>;
<b>use</b> <a href="../haneul/table.md#haneul_table">haneul::table</a>;
<b>use</b> <a href="../haneul/transfer.md#haneul_transfer">haneul::transfer</a>;
<b>use</b> <a href="../haneul/tx_context.md#haneul_tx_context">haneul::tx_context</a>;
<b>use</b> <a href="../haneul/types.md#haneul_types">haneul::types</a>;
<b>use</b> <a href="../haneul/url.md#haneul_url">haneul::url</a>;
<b>use</b> <a href="../haneul/vec_map.md#haneul_vec_map">haneul::vec_map</a>;
<b>use</b> <a href="../haneul/vec_set.md#haneul_vec_set">haneul::vec_set</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="deepbook_order_query_OrderPage"></a>

## Struct `OrderPage`



<pre><code><b>public</b> <b>struct</b> <a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">OrderPage</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>: vector&lt;<a href="../deepbook/clob_v2.md#deepbook_clob_v2_Order">deepbook::clob_v2::Order</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../deepbook/order_query.md#deepbook_order_query_has_next_page">has_next_page</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../deepbook/order_query.md#deepbook_order_query_next_tick_level">next_tick_level</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../deepbook/order_query.md#deepbook_order_query_next_order_id">next_order_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="deepbook_order_query_PAGE_LIMIT"></a>



<pre><code><b>const</b> <a href="../deepbook/order_query.md#deepbook_order_query_PAGE_LIMIT">PAGE_LIMIT</a>: u64 = 100;
</code></pre>



<a name="deepbook_order_query_iter_bids"></a>

## Function `iter_bids`



<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_iter_bids">iter_bids</a>&lt;T1, T2&gt;(pool: &<a href="../deepbook/clob_v2.md#deepbook_clob_v2_Pool">deepbook::clob_v2::Pool</a>&lt;T1, T2&gt;, start_tick_level: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, start_order_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, min_expire_timestamp: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, max_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ascending: bool): <a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">deepbook::order_query::OrderPage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_iter_bids">iter_bids</a>&lt;T1, T2&gt;(
    pool: &Pool&lt;T1, T2&gt;,
    // tick level to start from
    start_tick_level: Option&lt;u64&gt;,
    // order id within that tick level to start from
    start_order_id: Option&lt;u64&gt;,
    // <b>if</b> provided, do not include <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> with an expire timestamp less than the provided value (expired order),
    // value is in microseconds
    min_expire_timestamp: Option&lt;u64&gt;,
    // do not show <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> with an ID larger than max_id--
    // i.e., <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> added later than this one
    max_id: Option&lt;u64&gt;,
    // <b>if</b> <b>true</b>, the <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> are returned in ascending tick level.
    ascending: bool,
): <a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">OrderPage</a> {
    <b>let</b> bids = <a href="../deepbook/clob_v2.md#deepbook_clob_v2_bids">clob_v2::bids</a>(pool);
    <b>let</b> <b>mut</b> <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> = <a href="../deepbook/order_query.md#deepbook_order_query_iter_ticks_internal">iter_ticks_internal</a>(
        bids,
        start_tick_level,
        start_order_id,
        min_expire_timestamp,
        max_id,
        ascending
    );
    <b>let</b> (<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>, <a href="../deepbook/order_query.md#deepbook_order_query_has_next_page">has_next_page</a>, <a href="../deepbook/order_query.md#deepbook_order_query_next_tick_level">next_tick_level</a>, <a href="../deepbook/order_query.md#deepbook_order_query_next_order_id">next_order_id</a>) = <b>if</b> (vector::length(&<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>) &gt; <a href="../deepbook/order_query.md#deepbook_order_query_PAGE_LIMIT">PAGE_LIMIT</a>) {
        <b>let</b> last_order = vector::pop_back(&<b>mut</b> <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>);
        (<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>, <b>true</b>, some(<a href="../deepbook/clob_v2.md#deepbook_clob_v2_tick_level">clob_v2::tick_level</a>(&last_order)), some(<a href="../deepbook/clob_v2.md#deepbook_clob_v2_order_id">clob_v2::order_id</a>(&last_order)))
    } <b>else</b> {
        (<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>, <b>false</b>, none(), none())
    };
    <a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">OrderPage</a> {
        <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>,
        <a href="../deepbook/order_query.md#deepbook_order_query_has_next_page">has_next_page</a>,
        <a href="../deepbook/order_query.md#deepbook_order_query_next_tick_level">next_tick_level</a>,
        <a href="../deepbook/order_query.md#deepbook_order_query_next_order_id">next_order_id</a>
    }
}
</code></pre>



</details>

<a name="deepbook_order_query_iter_asks"></a>

## Function `iter_asks`



<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_iter_asks">iter_asks</a>&lt;T1, T2&gt;(pool: &<a href="../deepbook/clob_v2.md#deepbook_clob_v2_Pool">deepbook::clob_v2::Pool</a>&lt;T1, T2&gt;, start_tick_level: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, start_order_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, min_expire_timestamp: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, max_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ascending: bool): <a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">deepbook::order_query::OrderPage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_iter_asks">iter_asks</a>&lt;T1, T2&gt;(
    pool: &Pool&lt;T1, T2&gt;,
    // tick level to start from
    start_tick_level: Option&lt;u64&gt;,
    // order id within that tick level to start from
    start_order_id: Option&lt;u64&gt;,
    // <b>if</b> provided, do not include <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> with an expire timestamp less than the provided value (expired order),
    // value is in microseconds
    min_expire_timestamp: Option&lt;u64&gt;,
    // do not show <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> with an ID larger than max_id--
    // i.e., <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> added later than this one
    max_id: Option&lt;u64&gt;,
    // <b>if</b> <b>true</b>, the <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> are returned in ascending tick level.
    ascending: bool,
): <a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">OrderPage</a> {
    <b>let</b> asks = <a href="../deepbook/clob_v2.md#deepbook_clob_v2_asks">clob_v2::asks</a>(pool);
    <b>let</b> <b>mut</b> <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> = <a href="../deepbook/order_query.md#deepbook_order_query_iter_ticks_internal">iter_ticks_internal</a>(
        asks,
        start_tick_level,
        start_order_id,
        min_expire_timestamp,
        max_id,
        ascending
    );
    <b>let</b> (<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>, <a href="../deepbook/order_query.md#deepbook_order_query_has_next_page">has_next_page</a>, <a href="../deepbook/order_query.md#deepbook_order_query_next_tick_level">next_tick_level</a>, <a href="../deepbook/order_query.md#deepbook_order_query_next_order_id">next_order_id</a>) = <b>if</b> (vector::length(&<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>) &gt; <a href="../deepbook/order_query.md#deepbook_order_query_PAGE_LIMIT">PAGE_LIMIT</a>) {
        <b>let</b> last_order = vector::pop_back(&<b>mut</b> <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>);
        (<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>, <b>true</b>, some(<a href="../deepbook/clob_v2.md#deepbook_clob_v2_tick_level">clob_v2::tick_level</a>(&last_order)), some(<a href="../deepbook/clob_v2.md#deepbook_clob_v2_order_id">clob_v2::order_id</a>(&last_order)))
    } <b>else</b> {
        (<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>, <b>false</b>, none(), none())
    };
    <a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">OrderPage</a> {
        <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>,
        <a href="../deepbook/order_query.md#deepbook_order_query_has_next_page">has_next_page</a>,
        <a href="../deepbook/order_query.md#deepbook_order_query_next_tick_level">next_tick_level</a>,
        <a href="../deepbook/order_query.md#deepbook_order_query_next_order_id">next_order_id</a>
    }
}
</code></pre>



</details>

<a name="deepbook_order_query_iter_ticks_internal"></a>

## Function `iter_ticks_internal`



<pre><code><b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_iter_ticks_internal">iter_ticks_internal</a>(ticks: &<a href="../deepbook/critbit.md#deepbook_critbit_CritbitTree">deepbook::critbit::CritbitTree</a>&lt;<a href="../deepbook/clob_v2.md#deepbook_clob_v2_TickLevel">deepbook::clob_v2::TickLevel</a>&gt;, start_tick_level: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, start_order_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, min_expire_timestamp: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, max_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, ascending: bool): vector&lt;<a href="../deepbook/clob_v2.md#deepbook_clob_v2_Order">deepbook::clob_v2::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_iter_ticks_internal">iter_ticks_internal</a>(
    ticks: &CritbitTree&lt;TickLevel&gt;,
    // tick level to start from
    start_tick_level: Option&lt;u64&gt;,
    // order id within that tick level to start from
    <b>mut</b> start_order_id: Option&lt;u64&gt;,
    // <b>if</b> provided, do not include <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> with an expire timestamp less than the provided value (expired order),
    // value is in microseconds
    min_expire_timestamp: Option&lt;u64&gt;,
    // do not show <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> with an ID larger than max_id--
    // i.e., <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> added later than this one
    max_id: Option&lt;u64&gt;,
    // <b>if</b> <b>true</b>, the <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> are returned in ascending tick level.
    ascending: bool,
): vector&lt;Order&gt; {
    <b>let</b> <b>mut</b> tick_level_key = <b>if</b> (option::is_some(&start_tick_level)) {
        option::destroy_some(start_tick_level)
    } <b>else</b> {
        <b>let</b> (key, _) = <b>if</b> (ascending) {
            <a href="../deepbook/critbit.md#deepbook_critbit_min_leaf">critbit::min_leaf</a>(ticks)
        }<b>else</b> {
            <a href="../deepbook/critbit.md#deepbook_critbit_max_leaf">critbit::max_leaf</a>(ticks)
        };
        key
    };
    <b>let</b> <b>mut</b> <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a> = vector[];
    <b>while</b> (tick_level_key != 0 && vector::length(&<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>) &lt; <a href="../deepbook/order_query.md#deepbook_order_query_PAGE_LIMIT">PAGE_LIMIT</a> + 1) {
        <b>let</b> <a href="../deepbook/order_query.md#deepbook_order_query_tick_level">tick_level</a> = <a href="../deepbook/critbit.md#deepbook_critbit_borrow_leaf_by_key">critbit::borrow_leaf_by_key</a>(ticks, tick_level_key);
        <b>let</b> open_orders = <a href="../deepbook/clob_v2.md#deepbook_clob_v2_open_orders">clob_v2::open_orders</a>(<a href="../deepbook/order_query.md#deepbook_order_query_tick_level">tick_level</a>);
        <b>let</b> <b>mut</b> next_order_key = <b>if</b> (option::is_some(&start_order_id)) {
            <b>let</b> key = option::destroy_some(start_order_id);
            <b>if</b> (!linked_table::contains(open_orders, key)) {
                <b>let</b> (next_leaf, _) = <b>if</b> (ascending) {
                    <a href="../deepbook/critbit.md#deepbook_critbit_next_leaf">critbit::next_leaf</a>(ticks, tick_level_key)
                }<b>else</b> {
                    <a href="../deepbook/critbit.md#deepbook_critbit_previous_leaf">critbit::previous_leaf</a>(ticks, tick_level_key)
                };
                tick_level_key = next_leaf;
                <b>continue</b>
            };
            start_order_id = option::none();
            some(key)
        }<b>else</b> {
            *linked_table::front(open_orders)
        };
        <b>while</b> (option::is_some(&next_order_key) && vector::length(&<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>) &lt; <a href="../deepbook/order_query.md#deepbook_order_query_PAGE_LIMIT">PAGE_LIMIT</a> + 1) {
            <b>let</b> key = option::destroy_some(next_order_key);
            <b>let</b> order = linked_table::borrow(open_orders, key);
            // <b>if</b> the order id is greater than max_id, we end the iteration <b>for</b> this tick level.
            <b>if</b> (option::is_some(&max_id) && key &gt; option::destroy_some(max_id)) {
                <b>break</b>
            };
            next_order_key = *linked_table::next(open_orders, key);
            // <b>if</b> expire timestamp is set, and <b>if</b> the order is expired, we skip it.
            <b>if</b> (option::is_none(&min_expire_timestamp) ||
                <a href="../deepbook/clob_v2.md#deepbook_clob_v2_expire_timestamp">clob_v2::expire_timestamp</a>(order) &gt; option::destroy_some(min_expire_timestamp)) {
                vector::push_back(&<b>mut</b> <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>, <a href="../deepbook/clob_v2.md#deepbook_clob_v2_clone_order">clob_v2::clone_order</a>(order));
            };
        };
        <b>let</b> (next_leaf, _) = <b>if</b> (ascending) {
            <a href="../deepbook/critbit.md#deepbook_critbit_next_leaf">critbit::next_leaf</a>(ticks, tick_level_key)
        }<b>else</b> {
            <a href="../deepbook/critbit.md#deepbook_critbit_previous_leaf">critbit::previous_leaf</a>(ticks, tick_level_key)
        };
        tick_level_key = next_leaf;
    };
    <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>
}
</code></pre>



</details>

<a name="deepbook_order_query_orders"></a>

## Function `orders`



<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>(page: &<a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">deepbook::order_query::OrderPage</a>): &vector&lt;<a href="../deepbook/clob_v2.md#deepbook_clob_v2_Order">deepbook::clob_v2::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>(page: &<a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">OrderPage</a>): &vector&lt;Order&gt; {
    &page.<a href="../deepbook/order_query.md#deepbook_order_query_orders">orders</a>
}
</code></pre>



</details>

<a name="deepbook_order_query_has_next_page"></a>

## Function `has_next_page`



<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_has_next_page">has_next_page</a>(page: &<a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">deepbook::order_query::OrderPage</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_has_next_page">has_next_page</a>(page: &<a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">OrderPage</a>): bool {
    page.<a href="../deepbook/order_query.md#deepbook_order_query_has_next_page">has_next_page</a>
}
</code></pre>



</details>

<a name="deepbook_order_query_next_tick_level"></a>

## Function `next_tick_level`



<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_next_tick_level">next_tick_level</a>(page: &<a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">deepbook::order_query::OrderPage</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_next_tick_level">next_tick_level</a>(page: &<a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">OrderPage</a>): Option&lt;u64&gt; {
    page.<a href="../deepbook/order_query.md#deepbook_order_query_next_tick_level">next_tick_level</a>
}
</code></pre>



</details>

<a name="deepbook_order_query_next_order_id"></a>

## Function `next_order_id`



<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_next_order_id">next_order_id</a>(page: &<a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">deepbook::order_query::OrderPage</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_next_order_id">next_order_id</a>(page: &<a href="../deepbook/order_query.md#deepbook_order_query_OrderPage">OrderPage</a>): Option&lt;u64&gt; {
    page.<a href="../deepbook/order_query.md#deepbook_order_query_next_order_id">next_order_id</a>
}
</code></pre>



</details>

<a name="deepbook_order_query_order_id"></a>

## Function `order_id`



<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_order_id">order_id</a>(order: &<a href="../deepbook/clob_v2.md#deepbook_clob_v2_Order">deepbook::clob_v2::Order</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_order_id">order_id</a>(order: &Order): u64 {
    <a href="../deepbook/clob_v2.md#deepbook_clob_v2_order_id">clob_v2::order_id</a>(order)
}
</code></pre>



</details>

<a name="deepbook_order_query_tick_level"></a>

## Function `tick_level`



<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_tick_level">tick_level</a>(order: &<a href="../deepbook/clob_v2.md#deepbook_clob_v2_Order">deepbook::clob_v2::Order</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../deepbook/order_query.md#deepbook_order_query_tick_level">tick_level</a>(order: &Order): u64 {
    <a href="../deepbook/clob_v2.md#deepbook_clob_v2_tick_level">clob_v2::tick_level</a>(order)
}
</code></pre>



</details>
