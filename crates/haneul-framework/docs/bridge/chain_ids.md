
<a name="0xb_chain_ids"></a>

# Module `0xb::chain_ids`



-  [Struct `BridgeRoute`](#0xb_chain_ids_BridgeRoute)
-  [Constants](#@Constants_0)
-  [Function `haneul_mainnet`](#0xb_chain_ids_haneul_mainnet)
-  [Function `haneul_testnet`](#0xb_chain_ids_haneul_testnet)
-  [Function `haneul_devnet`](#0xb_chain_ids_haneul_devnet)
-  [Function `eth_mainnet`](#0xb_chain_ids_eth_mainnet)
-  [Function `eth_sepolia`](#0xb_chain_ids_eth_sepolia)
-  [Function `valid_routes`](#0xb_chain_ids_valid_routes)
-  [Function `is_valid_route`](#0xb_chain_ids_is_valid_route)


<pre><code><b>use</b> <a href="dependencies/move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a name="0xb_chain_ids_BridgeRoute"></a>

## Struct `BridgeRoute`



<pre><code><b>struct</b> <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>source: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>destination: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xb_chain_ids_EthMainnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_EthMainnet">EthMainnet</a>: u8 = 10;
</code></pre>



<a name="0xb_chain_ids_EthSepolia"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a>: u8 = 11;
</code></pre>



<a name="0xb_chain_ids_HaneulDevnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_HaneulDevnet">HaneulDevnet</a>: u8 = 2;
</code></pre>



<a name="0xb_chain_ids_HaneulMainnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_HaneulMainnet">HaneulMainnet</a>: u8 = 0;
</code></pre>



<a name="0xb_chain_ids_HaneulTestnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_HaneulTestnet">HaneulTestnet</a>: u8 = 1;
</code></pre>



<a name="0xb_chain_ids_haneul_mainnet"></a>

## Function `haneul_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_haneul_mainnet">haneul_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_haneul_mainnet">haneul_mainnet</a>(): u8 {
    <a href="chain_ids.md#0xb_chain_ids_HaneulMainnet">HaneulMainnet</a>
}
</code></pre>



</details>

<a name="0xb_chain_ids_haneul_testnet"></a>

## Function `haneul_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_haneul_testnet">haneul_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_haneul_testnet">haneul_testnet</a>(): u8 {
    <a href="chain_ids.md#0xb_chain_ids_HaneulTestnet">HaneulTestnet</a>
}
</code></pre>



</details>

<a name="0xb_chain_ids_haneul_devnet"></a>

## Function `haneul_devnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_haneul_devnet">haneul_devnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_haneul_devnet">haneul_devnet</a>(): u8 {
    <a href="chain_ids.md#0xb_chain_ids_HaneulDevnet">HaneulDevnet</a>
}
</code></pre>



</details>

<a name="0xb_chain_ids_eth_mainnet"></a>

## Function `eth_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_eth_mainnet">eth_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_eth_mainnet">eth_mainnet</a>(): u8 {
    <a href="chain_ids.md#0xb_chain_ids_EthMainnet">EthMainnet</a>
}
</code></pre>



</details>

<a name="0xb_chain_ids_eth_sepolia"></a>

## Function `eth_sepolia`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_eth_sepolia">eth_sepolia</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_eth_sepolia">eth_sepolia</a>(): u8 {
    <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a>
}
</code></pre>



</details>

<a name="0xb_chain_ids_valid_routes"></a>

## Function `valid_routes`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_valid_routes">valid_routes</a>(): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="chain_ids.md#0xb_chain_ids_BridgeRoute">chain_ids::BridgeRoute</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_valid_routes">valid_routes</a>(): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a>&gt; {
    <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_HaneulMainnet">HaneulMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_EthMainnet">EthMainnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_HaneulDevnet">HaneulDevnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_HaneulTestnet">HaneulTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_EthMainnet">EthMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_HaneulMainnet">HaneulMainnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a>, destination: <a href="chain_ids.md#0xb_chain_ids_HaneulDevnet">HaneulDevnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a>, destination: <a href="chain_ids.md#0xb_chain_ids_HaneulTestnet">HaneulTestnet</a> }]
}
</code></pre>



</details>

<a name="0xb_chain_ids_is_valid_route"></a>

## Function `is_valid_route`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_is_valid_route">is_valid_route</a>(source: u8, destination: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_is_valid_route">is_valid_route</a>(source: u8, destination: u8): bool {
    <b>let</b> route = <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source, destination };
    <b>return</b> <a href="dependencies/move-stdlib/vector.md#0x1_vector_contains">vector::contains</a>(&<a href="chain_ids.md#0xb_chain_ids_valid_routes">valid_routes</a>(), &route)
}
</code></pre>



</details>
