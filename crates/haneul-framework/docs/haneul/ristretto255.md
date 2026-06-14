---
title: Module `haneul::ristretto255`
---

Group operations of BLS12-381.
Only available in devnet.


-  [Struct `Scalar`](#haneul_ristretto255_Scalar)
-  [Struct `G`](#haneul_ristretto255_G)
-  [Constants](#@Constants_0)
-  [Function `scalar_from_bytes`](#haneul_ristretto255_scalar_from_bytes)
-  [Function `scalar_from_u64`](#haneul_ristretto255_scalar_from_u64)
-  [Function `scalar_zero`](#haneul_ristretto255_scalar_zero)
-  [Function `scalar_one`](#haneul_ristretto255_scalar_one)
-  [Function `scalar_add`](#haneul_ristretto255_scalar_add)
-  [Function `scalar_sub`](#haneul_ristretto255_scalar_sub)
-  [Function `scalar_mul`](#haneul_ristretto255_scalar_mul)
-  [Function `scalar_div`](#haneul_ristretto255_scalar_div)
-  [Function `scalar_neg`](#haneul_ristretto255_scalar_neg)
-  [Function `scalar_inv`](#haneul_ristretto255_scalar_inv)
-  [Function `g_from_bytes`](#haneul_ristretto255_g_from_bytes)
-  [Function `g_identity`](#haneul_ristretto255_g_identity)
-  [Function `g_generator`](#haneul_ristretto255_g_generator)
-  [Function `g_add`](#haneul_ristretto255_g_add)
-  [Function `g_sub`](#haneul_ristretto255_g_sub)
-  [Function `g_mul`](#haneul_ristretto255_g_mul)
-  [Function `g_div`](#haneul_ristretto255_g_div)
-  [Function `g_neg`](#haneul_ristretto255_g_neg)


<pre><code><b>use</b> <a href="../haneul/address.md#haneul_address">haneul::address</a>;
<b>use</b> <a href="../haneul/bcs.md#haneul_bcs">haneul::bcs</a>;
<b>use</b> <a href="../haneul/group_ops.md#haneul_group_ops">haneul::group_ops</a>;
<b>use</b> <a href="../haneul/hex.md#haneul_hex">haneul::hex</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="haneul_ristretto255_Scalar"></a>

## Struct `Scalar`



<pre><code><b>public</b> <b>struct</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="haneul_ristretto255_G"></a>

## Struct `G`



<pre><code><b>public</b> <b>struct</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="haneul_ristretto255_SCALAR_ZERO_BYTES"></a>



<pre><code><b>const</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_ZERO_BYTES">SCALAR_ZERO_BYTES</a>: vector&lt;u8&gt; = vector[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
</code></pre>



<a name="haneul_ristretto255_SCALAR_ONE_BYTES"></a>



<pre><code><b>const</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_ONE_BYTES">SCALAR_ONE_BYTES</a>: vector&lt;u8&gt; = vector[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
</code></pre>



<a name="haneul_ristretto255_IDENTITY_BYTES"></a>



<pre><code><b>const</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_IDENTITY_BYTES">IDENTITY_BYTES</a>: vector&lt;u8&gt; = vector[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
</code></pre>



<a name="haneul_ristretto255_GENERATOR_BYTES"></a>



<pre><code><b>const</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_GENERATOR_BYTES">GENERATOR_BYTES</a>: vector&lt;u8&gt; = vector[226, 242, 174, 10, 106, 188, 78, 113, 168, 132, 169, 97, 197, 0, 81, 95, 88, 227, 11, 106, 165, 130, 221, 141, 182, 166, 89, 69, 224, 141, 45, 118];
</code></pre>



<a name="haneul_ristretto255_SCALAR_TYPE"></a>



<pre><code><b>const</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_TYPE">SCALAR_TYPE</a>: u8 = 5;
</code></pre>



<a name="haneul_ristretto255_G_TYPE"></a>



<pre><code><b>const</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_G_TYPE">G_TYPE</a>: u8 = 6;
</code></pre>



<a name="haneul_ristretto255_scalar_from_bytes"></a>

## Function `scalar_from_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_from_bytes">scalar_from_bytes</a>(bytes: &vector&lt;u8&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_from_bytes">scalar_from_bytes</a>(bytes: &vector&lt;u8&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_from_bytes">group_ops::from_bytes</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_TYPE">SCALAR_TYPE</a>, *bytes, <b>false</b>)
}
</code></pre>



</details>

<a name="haneul_ristretto255_scalar_from_u64"></a>

## Function `scalar_from_u64`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_from_u64">scalar_from_u64</a>(x: u64): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_from_u64">scalar_from_u64</a>(x: u64): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt; {
    <b>let</b> scalar: u256 = x <b>as</b> u256;
    <b>let</b> bytes = <a href="../haneul/bcs.md#haneul_bcs_to_bytes">bcs::to_bytes</a>(&scalar);
    <a href="../haneul/group_ops.md#haneul_group_ops_from_bytes">group_ops::from_bytes</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_TYPE">SCALAR_TYPE</a>, bytes, <b>true</b>)
}
</code></pre>



</details>

<a name="haneul_ristretto255_scalar_zero"></a>

## Function `scalar_zero`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_zero">scalar_zero</a>(): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_zero">scalar_zero</a>(): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_from_bytes">group_ops::from_bytes</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_TYPE">SCALAR_TYPE</a>, <a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_ZERO_BYTES">SCALAR_ZERO_BYTES</a>, <b>true</b>)
}
</code></pre>



</details>

<a name="haneul_ristretto255_scalar_one"></a>

## Function `scalar_one`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_one">scalar_one</a>(): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_one">scalar_one</a>(): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_from_bytes">group_ops::from_bytes</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_TYPE">SCALAR_TYPE</a>, <a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_ONE_BYTES">SCALAR_ONE_BYTES</a>, <b>true</b>)
}
</code></pre>



</details>

<a name="haneul_ristretto255_scalar_add"></a>

## Function `scalar_add`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_add">scalar_add</a>(e1: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;, e2: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_add">scalar_add</a>(e1: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;, e2: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_add">group_ops::add</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_TYPE">SCALAR_TYPE</a>, e1, e2)
}
</code></pre>



</details>

<a name="haneul_ristretto255_scalar_sub"></a>

## Function `scalar_sub`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_sub">scalar_sub</a>(e1: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;, e2: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_sub">scalar_sub</a>(e1: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;, e2: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_sub">group_ops::sub</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_TYPE">SCALAR_TYPE</a>, e1, e2)
}
</code></pre>



</details>

<a name="haneul_ristretto255_scalar_mul"></a>

## Function `scalar_mul`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_mul">scalar_mul</a>(e1: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;, e2: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_mul">scalar_mul</a>(e1: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;, e2: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_mul">group_ops::mul</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_TYPE">SCALAR_TYPE</a>, e1, e2)
}
</code></pre>



</details>

<a name="haneul_ristretto255_scalar_div"></a>

## Function `scalar_div`

Returns e2/e1, fails if a is zero.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_div">scalar_div</a>(e1: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;, e2: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_div">scalar_div</a>(e1: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;, e2: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_div">group_ops::div</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_SCALAR_TYPE">SCALAR_TYPE</a>, e1, e2)
}
</code></pre>



</details>

<a name="haneul_ristretto255_scalar_neg"></a>

## Function `scalar_neg`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_neg">scalar_neg</a>(e: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_neg">scalar_neg</a>(e: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt; {
    <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_sub">scalar_sub</a>(&<a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_zero">scalar_zero</a>(), e)
}
</code></pre>



</details>

<a name="haneul_ristretto255_scalar_inv"></a>

## Function `scalar_inv`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_inv">scalar_inv</a>(e: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_inv">scalar_inv</a>(e: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt; {
    <a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_div">scalar_div</a>(e, &<a href="../haneul/ristretto255.md#haneul_ristretto255_scalar_one">scalar_one</a>())
}
</code></pre>



</details>

<a name="haneul_ristretto255_g_from_bytes"></a>

## Function `g_from_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_from_bytes">g_from_bytes</a>(bytes: &vector&lt;u8&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_from_bytes">g_from_bytes</a>(bytes: &vector&lt;u8&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_from_bytes">group_ops::from_bytes</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_G_TYPE">G_TYPE</a>, *bytes, <b>false</b>)
}
</code></pre>



</details>

<a name="haneul_ristretto255_g_identity"></a>

## Function `g_identity`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_identity">g_identity</a>(): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_identity">g_identity</a>(): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_from_bytes">group_ops::from_bytes</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_G_TYPE">G_TYPE</a>, <a href="../haneul/ristretto255.md#haneul_ristretto255_IDENTITY_BYTES">IDENTITY_BYTES</a>, <b>true</b>)
}
</code></pre>



</details>

<a name="haneul_ristretto255_g_generator"></a>

## Function `g_generator`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_generator">g_generator</a>(): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_generator">g_generator</a>(): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_from_bytes">group_ops::from_bytes</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_G_TYPE">G_TYPE</a>, <a href="../haneul/ristretto255.md#haneul_ristretto255_GENERATOR_BYTES">GENERATOR_BYTES</a>, <b>true</b>)
}
</code></pre>



</details>

<a name="haneul_ristretto255_g_add"></a>

## Function `g_add`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_add">g_add</a>(e1: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;, e2: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_add">g_add</a>(e1: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt;, e2: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_add">group_ops::add</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_G_TYPE">G_TYPE</a>, e1, e2)
}
</code></pre>



</details>

<a name="haneul_ristretto255_g_sub"></a>

## Function `g_sub`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_sub">g_sub</a>(e1: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;, e2: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_sub">g_sub</a>(e1: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt;, e2: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_sub">group_ops::sub</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_G_TYPE">G_TYPE</a>, e1, e2)
}
</code></pre>



</details>

<a name="haneul_ristretto255_g_mul"></a>

## Function `g_mul`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_mul">g_mul</a>(e1: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;, e2: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_mul">g_mul</a>(e1: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;, e2: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_mul">group_ops::mul</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_G_TYPE">G_TYPE</a>, e1, e2)
}
</code></pre>



</details>

<a name="haneul_ristretto255_g_div"></a>

## Function `g_div`

Returns e2 / e1, fails if scalar is zero.


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_div">g_div</a>(e1: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">haneul::ristretto255::Scalar</a>&gt;, e2: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_div">g_div</a>(e1: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_Scalar">Scalar</a>&gt;, e2: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt; {
    <a href="../haneul/group_ops.md#haneul_group_ops_div">group_ops::div</a>(<a href="../haneul/ristretto255.md#haneul_ristretto255_G_TYPE">G_TYPE</a>, e1, e2)
}
</code></pre>



</details>

<a name="haneul_ristretto255_g_neg"></a>

## Function `g_neg`



<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_neg">g_neg</a>(e: &<a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;): <a href="../haneul/group_ops.md#haneul_group_ops_Element">haneul::group_ops::Element</a>&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">haneul::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../haneul/ristretto255.md#haneul_ristretto255_g_neg">g_neg</a>(e: &Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt;): Element&lt;<a href="../haneul/ristretto255.md#haneul_ristretto255_G">G</a>&gt; {
    <a href="../haneul/ristretto255.md#haneul_ristretto255_g_sub">g_sub</a>(&<a href="../haneul/ristretto255.md#haneul_ristretto255_g_identity">g_identity</a>(), e)
}
</code></pre>



</details>
