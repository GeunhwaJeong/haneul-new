
<a name="0x2_haneul"></a>

# Module `0x2::haneul`

Coin<HANEUL> is the token used to pay for gas in Haneul.
It has 9 decimals, and the smallest unit (10^-9) is called "geunhwa".


-  [Struct `HANEUL`](#0x2_haneul_HANEUL)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x2_haneul_new)
-  [Function `transfer`](#0x2_haneul_transfer)


<pre><code><b>use</b> <a href="../../dependencies/move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../../dependencies/haneul-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../dependencies/haneul-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../dependencies/haneul-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../dependencies/haneul-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../dependencies/haneul-framework/url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0x2_haneul_HANEUL"></a>

## Struct `HANEUL`

Name of the coin


<pre><code><b>struct</b> <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_HANEUL">HANEUL</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x2_haneul_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="0x2_haneul_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="0x2_haneul_GEUNHWA_PER_HANEUL"></a>

The amount of Geunhwa per Haneul token based on the the fact that geunhwa is
10^-9 of a Haneul token


<pre><code><b>const</b> <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_GEUNHWA_PER_HANEUL">GEUNHWA_PER_HANEUL</a>: u64 = 1000000000;
</code></pre>



<a name="0x2_haneul_TOTAL_SUPPLY_GEUNHWA"></a>

The total supply of Haneul denominated in Geunhwa (10 Billion * 10^9)


<pre><code><b>const</b> <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_TOTAL_SUPPLY_GEUNHWA">TOTAL_SUPPLY_GEUNHWA</a>: u64 = 10000000000000000000;
</code></pre>



<a name="0x2_haneul_TOTAL_SUPPLY_HANEUL"></a>

The total supply of Haneul denominated in whole Haneul tokens (10 Billion)


<pre><code><b>const</b> <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_TOTAL_SUPPLY_HANEUL">TOTAL_SUPPLY_HANEUL</a>: u64 = 10000000000;
</code></pre>



<a name="0x2_haneul_new"></a>

## Function `new`

Register the <code><a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_HANEUL">HANEUL</a></code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_new">new</a>(ctx: &<b>mut</b> <a href="../../dependencies/haneul-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../dependencies/haneul-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_new">new</a>(ctx: &<b>mut</b> TxContext): Balance&lt;<a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_HANEUL">HANEUL</a>&gt; {
    <b>assert</b>!(<a href="../../dependencies/haneul-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(<a href="../../dependencies/haneul-framework/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) == 0, <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_EAlreadyMinted">EAlreadyMinted</a>);

    <b>let</b> (treasury, metadata) = <a href="../../dependencies/haneul-framework/coin.md#0x2_coin_create_currency">coin::create_currency</a>(
        <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_HANEUL">HANEUL</a> {},
        9,
        b"<a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_HANEUL">HANEUL</a>",
        b"Haneul",
        // TODO: add appropriate description and logo <a href="../../dependencies/haneul-framework/url.md#0x2_url">url</a>
        b"",
        <a href="../../dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>(),
        ctx
    );
    <a href="../../dependencies/haneul-framework/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> supply = <a href="../../dependencies/haneul-framework/coin.md#0x2_coin_treasury_into_supply">coin::treasury_into_supply</a>(treasury);
    <b>let</b> total_haneul = <a href="../../dependencies/haneul-framework/balance.md#0x2_balance_increase_supply">balance::increase_supply</a>(&<b>mut</b> supply, <a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_TOTAL_SUPPLY_GEUNHWA">TOTAL_SUPPLY_GEUNHWA</a>);
    <a href="../../dependencies/haneul-framework/balance.md#0x2_balance_destroy_supply">balance::destroy_supply</a>(supply);
    total_haneul
}
</code></pre>



</details>

<a name="0x2_haneul_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/haneul-framework/transfer.md#0x2_transfer">transfer</a>(c: <a href="../../dependencies/haneul-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/haneul-framework/transfer.md#0x2_transfer">transfer</a>(c: <a href="../../dependencies/haneul-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../dependencies/haneul-framework/haneul.md#0x2_haneul_HANEUL">HANEUL</a>&gt;, recipient: <b>address</b>) {
    <a href="../../dependencies/haneul-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>
