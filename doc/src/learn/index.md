---
title: Learning Haneul
---

*Haneul: pronounced "sweet" without the "T" - with Transactions (loads of them), things are SWEET indeed. :-)*

Welcome to the documentation for the Haneul platform. Since Haneul is built upon the core [Move](https://github.com/GeunhwaJeong/awesome-move)
programming language, you should familiarize yourself with it and use this content to apply the differences. For a summary of these differences, see
[Haneul compared to other blockchains](../learn/haneul-compared.md).

For a deep dive into Haneul technology, see the [Haneul Smart Contracts Platform](../../../paper/haneul.pdf) white paper. Find answers to common questions about our [roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md) and more in our [FAQ](../contribute/faq.md).

## See what's new

Find the latest updates to these contents in this section:

* [Haneul DevNet](../explore/devnet.md) - Experiment with Haneul DevNet: request gas tokens, mint/customize example NFTs, publish a Move module, and make a Move call.
* [JSON-RPC API](../build/json-rpc.md) - Set up your own local Haneul RPC Server and use the Haneul JSON-RPC API to interact with a local Haneul network.
* [Narwhal and Tusk for consensus](../learn/architecture/consensus.md) - Learn about Narwhal and Tusk, Haneul's high-throughput mempool and consensus engine.
* [RPC API publishing](../build/json-rpc.md#haneul_publish) - Follow instructions for publishing Move modules via the Publish endpoint.
* [Wallet improvements](../build/wallet.md#active-address)- Employ an active (default) addresses and [use gas objects for transactions](../build/wallet.md#paying-for-transactions-with-gas-objects).

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
* [Core Move](https://github.com/diem/move/tree/main/language/documentation) documentation, including:
  * [Tutorial](https://github.com/diem/move/blob/main/language/documentation/tutorial/README.md) - A step-by-step guide through writing a Move module.
  * [Book](https://github.com/diem/move/blob/main/language/documentation/book/src/introduction.md) - A summary with pages on [various topics](https://github.com/diem/move/tree/main/language/documentation/book/src).
  * [Examples](https://github.com/diem/move/tree/main/language/documentation/examples/experimental) - A set of samples, such as for [defining a coin](https://github.com/diem/move/tree/main/language/documentation/examples/experimental/basic-coin) and [swapping it](https://github.com/diem/move/tree/main/language/documentation/examples/experimental/coin-swap).
* [Awesome Move](https://github.com/GeunhwaJeong/awesome-move/blob/main/README.md) - A summary of resources related to Move, from blockchains through code samples.
* [Haneul API Reference](https://playground.open-rpc.org/?uiSchema%5BappBar%5D%5Bui:splitView%5D=false&schemaUrl=https://raw.githubusercontent.com/HaneulLabs/haneul/main/haneul/open_rpc/spec/openrpc.json&uiSchema%5BappBar%5D%5Bui:input%5D=false) - The reference files for the [Haneul JSON-RPC API](../build/json-rpc.md).
