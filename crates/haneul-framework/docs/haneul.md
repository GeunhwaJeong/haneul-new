
<a name="0x2_haneul"></a>

# Module `0x2::haneul`

Coin<HANEUL> is the token used to pay for gas in Haneul.
It has 9 decimals, and the smallest unit (10^-9) is called "geunhwa".


-  [Struct `HANEUL`](#0x2_haneul_HANEUL)
-  [Function `new`](#0x2_haneul_new)
-  [Function `transfer`](#0x2_haneul_transfer)


<pre><code><b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0x2_haneul_HANEUL"></a>

## Struct `HANEUL`

Name of the coin


<pre><code><b>struct</b> <a href="haneul.md#0x2_haneul_HANEUL">HANEUL</a> <b>has</b> drop
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

<a name="0x2_haneul_new"></a>

## Function `new`

Register the <code><a href="haneul.md#0x2_haneul_HANEUL">HANEUL</a></code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="haneul.md#0x2_haneul_new">new</a>(ctx: &<b>mut</b> <a href="tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="haneul.md#0x2_haneul_new">new</a>(ctx: &<b>mut</b> TxContext): Supply&lt;<a href="haneul.md#0x2_haneul_HANEUL">HANEUL</a>&gt; {
    <b>let</b> (treasury, metadata) = <a href="coin.md#0x2_coin_create_currency">coin::create_currency</a>(
        <a href="haneul.md#0x2_haneul_HANEUL">HANEUL</a> {},
        9,
        b"<a href="haneul.md#0x2_haneul_HANEUL">HANEUL</a>",
        b"Haneul",
        // TODO: add appropriate description and logo <a href="url.md#0x2_url">url</a>
        b"",
        <a href="_none">option::none</a>(),
        ctx
    );
    <a href="transfer.md#0x2_transfer_freeze_object">transfer::freeze_object</a>(metadata);
    <a href="coin.md#0x2_coin_treasury_into_supply">coin::treasury_into_supply</a>(treasury)
}
</code></pre>



</details>

<a name="0x2_haneul_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="transfer.md#0x2_transfer">transfer</a>(c: <a href="coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="haneul.md#0x2_haneul_HANEUL">haneul::HANEUL</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="transfer.md#0x2_transfer">transfer</a>(c: <a href="coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="haneul.md#0x2_haneul_HANEUL">HANEUL</a>&gt;, recipient: <b>address</b>) {
    <a href="transfer.md#0x2_transfer_transfer">transfer::transfer</a>(c, recipient)
}
</code></pre>



</details>
