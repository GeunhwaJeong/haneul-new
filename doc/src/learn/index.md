---
title: Learning Haneul
---

*Haneul: pronounced "sweet" without the "T" - with Transactions (loads of them), things are SWEET indeed. :-)*

Welcome to the documentation for the Haneul platform. Haneul is built on the core [Move](https://github.com/GeunhwaJeong/awesome-move) programming language. This documentation assumes that you have a basic working knowledge of Move. To learn more about the differences between core Move and Haneul Move, see [How Haneul Move differs from core Move](../learn/haneul-move-diffs.md).

For a deep dive into Haneul technology, see the [Haneul Smart Contracts Platform](https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf) white paper. Find answers to common questions about our [roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md) and more in our [FAQ](../contribute/faq.md).

> **Important:** This site is available in two versions in the menu at top left: the default and stable [Devnet](https://docs.haneul.io/devnet/learn) branch and the [Latest build](https://docs.haneul.io/learn) upstream `main` branch. Use the `devnet` version for app development on top of Haneul. Use the Latest build `main` branch for [contributing to the Haneul blockchain](../contribute/index.md) itself. Always check and submit fixes to the `main` branch.

## See what's new

### Doc updates

The following list includes the recent updates to Haneul and the documentation:

* Interact with the Haneul network using our new [Rust SDK](../build/rust-sdk.md), a collection of Rust language [JSON-RPC wrapper and crypto utilities](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-sdk).
* Haneul now supports development using [Microsoft Windows 11, macOS, and Linux](../build/install.md#supported-oses). See [install Haneul](../build/install.md#prerequisites) for the prerequisites of each operating system.
* This site is now available in two versions in the menu at top left: the default and stable [Devnet](https://docs.haneul.io/devnet/learn) branch and the [Latest build](https://docs.haneul.io/learn) upstream `main` branch. Use the `devnet` version for app development on top of Haneul. Use the Latest build `main` branch for  [contributing to the Haneul blockchain](../contribute/index.md) itself. Always check and submit fixes to the `main` branch.
* `haneul::id` is now `haneul::object` and `VersionedID` is now `Info`. Use the [Object module](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/sources/object.move) that has [replaced the former ID.move.](https://github.com/GeunhwaJeong/haneul/pull/3241)
* Find a list of [single-writer apps](../learn/single-writer-apps.md) that would benefit from Haneul's advantages in handling [simple transactions](../learn/how-haneul-works.md#simple-transactions).
* Install the [Haneul Wallet Browser Extension](../explore/wallet-browser.md) to create NFTs, transfer coins, and carry out common transactions in a Chrome tab.

See the Haneul `doc/src` [history](https://github.com/GeunhwaJeong/haneul/commits/main/doc/src) for a complete changelog of updates to this site. 

### Code changes

Learn about the latest releases in the [#release-notes](https://discord.com/channels/916379725201563759/974444055259910174) channel on Discord.

For a complete view of all changes in the Haneul `devnet` branch, see:
https://github.com/GeunhwaJeong/haneul/commits/devnet

For upstream updates in the `main` branch, see:
https://github.com/GeunhwaJeong/haneul/commits/main

## Kickstart development
The links in the section point to information to help you start working with Haneul. 

### Write Smart Contracts with Move
Go to the [Move Quick Start](../build/move/index.md) for information about installation, defining custom objects, object operations (create/destroy/update/transfer/freeze), publishing, and invoking your published code.

### Start the Haneul network with Haneul CLI client
See the [Haneul CLI client Quick Start](../contribute/cli-client.md) for information about installation, querying the chain, client setup, sending transfer transactions, and viewing the effects.

### Take the end-to-end tutorial
Proceed to the [Haneul Tutorial](../explore/tutorials.md) for a summary view of setting up your environment, starting a Haneul network, gathering accounts and gas, and publishing and playing a game in Haneul.

### Program with Objects
Finish with the detailed [Programming with objects](../build/programming-with-objects/index.md) tutorial series offering detailed guidance on manipulating Haneul objects, from creation and storage through wrapping and using child objects.

## Navigate this site
Navigate and search this site however you see fit. If you're new to Haneul, we recommend that you review the following content in this order:

**Learn** - includes information to help you learn:
* [About Haneul](../learn/about-haneul.md)
* [How Haneul works](../learn/how-haneul-works.md)
* [Haneul compared to other blockchains](../learn/haneul-compared.md)

**Build** - includes information about how to:
* [Install Haneul](../build/install.md)
* [Create smart contracts with Move](../build/move/index.md)
* [Set up and configure a local Haneul network](../contribute/cli-client.md)
* [Start a local JSON-RPC Gateway server](../build/json-rpc.md#start-local-rpc-server)

**Explore** - includes more in-depth information about:
* [Haneul Wallet](../explore/wallet-browser.md)
* [Devnet](../build/devnet.md)
* [Haneul tutorials](../explore/tutorials.md)
* [Haneul prototypes](../explore/prototypes.md)
* [Haneul examples](../explore/examples.md)  

**Contribute** - includes the following:
* [Frequently Asked Questions](../contribute/faq.md)
* [Logging, Tracing, Metrics, and Observability](../contribute/observability.md)
* [Research Papers](../contribute/research-papers.md)
* [Haneul Code of Conduct](../contribute/code-of-conduct.md)
   
**Additional resources** - lets you:
* Employ the [Haneul API Reference](https://docs.haneul.io/haneul-jsonrpc) files for the [Haneul JSON-RPC API](../build/json-rpc.md).
* View the [Haneul Labs](https://www.youtube.com/channel/UCI7pCUVxSLcndVhPpZOwZgg) YouTube channel for introductory videos on technology and partners.
