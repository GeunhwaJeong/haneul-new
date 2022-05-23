---
title: Learning Haneul
---

*Haneul: pronounced "sweet" without the "T" - with Transactions (loads of them), things are SWEET indeed. :-)*

Welcome to the documentation for the Haneul platform. Since Haneul is built upon the core [Move](https://github.com/GeunhwaJeong/awesome-move)
programming language, you should familiarize yourself with it and use this content to apply the differences. For a summary of these differences, see
[Haneul compared to other blockchains](../learn/haneul-compared.md).

For a deep dive into Haneul technology, see the [Haneul Smart Contracts Platform](https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf) white paper. Find answers to common questions about our [roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md) and more in our [FAQ](../contribute/faq.md).

## See what's new

Find the latest updates to these contents in this section:

* [Haneul tokenomics](../learn/tokenomics/index.md) are now fully explained and cover:
  * [Haneul token](../learn/tokenomics/haneul-token.md).
  * [Gas-pricing mechanism](../learn/tokenomics/gas-pricing.md).
  * [Haneul storage fund](../learn/tokenomics/storage-fund.md).
  * [Delegated proof-of-stake system](../learn/tokenomics/proof-of-stake.md).
* New [`haneul-setup.sh`](https://github.com/GeunhwaJeong/haneul/blob/main/doc/utils/haneul-setup.sh) script enables full environment setup in a [single set of commands](../build/install.md).
* Haneul version 0.2.0 released to DevNet:
  * DevNet data will be wiped along with this release. If you have requested test HANEUL tokens via faucet, please do so again via the [#devnet-faucet](https://discord.com/channels/916379725201563759/971488439931392130) channel on Discord.
  * Added rustdoc output for [haneul](https://haneullabs.github.io/haneul/), [narwhal](https://haneullabs.github.io/narwhal/), and [haneullabs-infra](https://haneullabs.github.io/haneullabs-infra/) projects available from both [Install Haneul](../build/install.md#source-code) and [Contribute to Haneul](../contribute/index.md#download-haneul).
  * Added persistent storage across releases. This will greatly reduce the frequency to wipe data during upgrades. 
  * Internal network interfaces are now described using the MultiAddr format.
  * Internal gRPC network interfaces now use a bincode codec instead of protobuf.
  * And many Narwhal updates relevant to Haneul.

For a complete view of all changes in Haneul 0.2.0, see:
https://github.com/GeunhwaJeong/haneul/commits/devnet

See the Haneul `doc/src` [history](https://github.com/GeunhwaJeong/haneul/commits/main/doc/src) for a complete changelog of updates to this site. 

## Kickstart development

### Move quick start
Go to the [Move Quick Start](../build/move.md) for installation, defining custom objects, object operations (create/destroy/update/transfer/freeze), publishing, and invoking your published code.

### Wallet quick start
See the [Wallet Quick Start](../build/wallet.md) for installation, querying the chain, client setup, sending transfer transactions, and viewing the effects.

### End-to-end tutorial
Finish with the [Haneul Tutorial](../explore/tutorials.md) for a summary view of setting up your environment, starting a Haneul network, gathering accounts and gas, and publishing and playing a game in Haneul.

## Navigate this site

Navigate and search this site however you see fit. Here is the order we recommend if you are new to Haneul:

1. Learn [about Haneul](../learn/about-haneul.md), how [Haneul differs from Move](../learn/why-move.md), and [how Haneul works](../learn/how-haneul-works.md) starting in this very section.
1. [Build](../build/index.md) smart contracts, wallets, validators, transactions, and more.
1. [Explore](../explore/index.md) prototypes and examples.
1. [Contribute](../contribute/index.md) to Haneul by joining the community, making enhancements, and learning about Haneul Labs.

## Use supporting sites

Take note of these related repositories of information to make best use of the knowledge here:

* [Move & Haneul podcast](https://zeroknowledge.fm/228-2/) on Zero Knowledge where programmable objects are described in detail.
* Original [Move Book](https://move-book.com/index.html) written by a member of the Haneul team.
* [Core Move](https://github.com/move-language/move/tree/main/language/documentation) documentation, including:
  * [Tutorial](https://github.com/move-language/move/blob/main/language/documentation/tutorial/README.md) - A step-by-step guide through writing a Move module.
  * [Book](https://github.com/move-language/move/blob/main/language/documentation/book/src/introduction.md) - A summary with pages on [various topics](https://github.com/move-language/move/tree/main/language/documentation/book/src).
  * [Examples](https://github.com/move-language/move/tree/main/language/documentation/examples/experimental) - A set of samples, such as for [defining a coin](https://github.com/move-language/move/tree/main/language/documentation/examples/experimental/basic-coin) and [swapping it](https://github.com/move-language/move/tree/main/language/documentation/examples/experimental/coin-swap).
* [Awesome Move](https://github.com/GeunhwaJeong/awesome-move/blob/main/README.md) - A summary of resources related to Move, from blockchains through code samples.
* [Haneul API Reference](https://playground.open-rpc.org/?uiSchema%5BappBar%5D%5Bui:splitView%5D=false&schemaUrl=https://raw.githubusercontent.com/HaneulLabs/haneul/main/haneul/open_rpc/spec/openrpc.json&uiSchema%5BappBar%5D%5Bui:input%5D=false) - The reference files for the [Haneul JSON-RPC API](../build/json-rpc.md).
