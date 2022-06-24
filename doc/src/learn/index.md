---
title: Learning Haneul
---

*Haneul: pronounced "sweet" without the "T" - with Transactions (loads of them), things are SWEET indeed. :-)*

Welcome to the documentation for the Haneul platform. Since Haneul is built upon the core [Move](https://github.com/GeunhwaJeong/awesome-move)
programming language, you should familiarize yourself with it and use this content to apply the differences. For a summary of these differences, see
[Haneul compared to other blockchains](../learn/haneul-compared.md).

For a deep dive into Haneul technology, see the [Haneul Smart Contracts Platform](https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf) white paper. Find answers to common questions about our [roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md) and more in our [FAQ](../contribute/faq.md).

> **Important:** This site is built from the upstream `main` branch and therefore will contain updates not yet found in `devnet`.

## See what's new

Find the latest updates to these contents in this section:

* [Docker Compose](../build/fullnode.md#using-docker-compose) enables simple creation of Haneul Fullnodes using [Docker](https://github.com/GeunhwaJeong/haneul/tree/main/docker/fullnode#readme).
* [Run a fullnode](../build/fullnode.md) in Haneul to have your own local copy of full blockchain state, contribute to Haneul, and qualify to be a potential validator.
* [Programming with objects](../build/programming-with-objects/index.md) tutorial series offers detailed guidance on manipulating Haneul objects, from creation and storage through wrapping and using child objects.
* Haneul [version 0.3.0](https://github.com/GeunhwaJeong/haneul/releases/tag/devnet-0.3.0-rc) released to DevNet. See [RELEASES](https://github.com/GeunhwaJeong/haneul/blob/main/RELEASES.md) for details on other releases.
* [Haneul tokenomics](../learn/tokenomics/index.md) are now fully explained and cover:
  * [Haneul token](../learn/tokenomics/haneul-token.md).
  * [Gas-pricing mechanism](../learn/tokenomics/gas-pricing.md).
  * [Haneul storage fund](../learn/tokenomics/storage-fund.md).
  * [Delegated proof-of-stake system](../learn/tokenomics/proof-of-stake.md).

For a complete view of all changes in the Haneul `devnet` branch, see:
https://github.com/GeunhwaJeong/haneul/commits/devnet

For upstream updates in the `main` branch, see:
https://github.com/GeunhwaJeong/haneul/commits/main

See the Haneul `doc/src` [history](https://github.com/GeunhwaJeong/haneul/commits/main/doc/src) for a complete changelog of updates to this site. 

## Kickstart development

### Move quick start
Go to the [Move Quick Start](../build/move.md) for installation, defining custom objects, object operations (create/destroy/update/transfer/freeze), publishing, and invoking your published code.

### Wallet quick start
See the [Wallet Quick Start](../build/wallet.md) for installation, querying the chain, client setup, sending transfer transactions, and viewing the effects.

### End-to-end tutorial
Proceed to the [Haneul Tutorial](../explore/tutorials.md) for a summary view of setting up your environment, starting a Haneul network, gathering accounts and gas, and publishing and playing a game in Haneul.

### Programming with Objects series
Finish with the detailed [Programming with objects](../build/programming-with-objects/index.md) tutorial series offering detailed guidance on manipulating Haneul objects, from creation and storage through wrapping and using child objects.

## Navigate this site

Navigate and search this site however you see fit. Here is the order we recommend if you are new to Haneul:

1. Learn [about Haneul](../learn/about-haneul.md), how [Haneul differs from Move](../learn/why-move.md), and [how Haneul works](../learn/how-haneul-works.md) starting in this very section.
1. [Build](../build/index.md) smart contracts, wallets, validators, transactions, and more.
1. [Explore](../explore/index.md) prototypes and examples.
1. [Contribute](../contribute/index.md) to Haneul by joining the community, making enhancements, and learning about Haneul Labs.
1. Employ the [Haneul API Reference](https://playground.open-rpc.org/?uiSchema%5BappBar%5D%5Bui:splitView%5D=false&schemaUrl=https://raw.githubusercontent.com/HaneulLabs/haneul/main/haneul/open_rpc/spec/openrpc.json&uiSchema%5BappBar%5D%5Bui:input%5D=false) reference files for the [Haneul JSON-RPC API](../build/json-rpc.md).
1. View the [Haneul Labs](https://www.youtube.com/channel/UCI7pCUVxSLcndVhPpZOwZgg) YouTube channel for introductory videos on technology and partners.
