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

* [Bullshark](https://arxiv.org/abs/2201.05677) replaced Tusk as the default consensus component of the [Narwhal](https://github.com/GeunhwaJeong/narwhal)-based [Haneul consensus engine](../learn/architecture/consensus.md) for reduced latency and support for fairness with slower validators. Note, Tusk may still be used.
* You must now specify the key scheme type as an argument (`secp256k1` or `ed25519`)  when running either the `haneul keytool generate` or `haneul client new-address` commands, as shown in [adding accounts to the client](../build/cli-client.md#adding-accounts-to-the-client).
* [Haneul version 0.8.0](https://github.com/GeunhwaJeong/haneul/releases/tag/devnet-0.8.0) is now live in Devnet with numerous fixes and enhancements, including new designs for [Haneul Explorer](https://explorer.devnet.haneul.io/) and [event query support](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-json-rpc/src/event_api.rs#L122-L210) in fullnode.
* Follow the [Cryptography (math)](https://github.com/GeunhwaJeong/haneul/tree/main/haneul_programmability/examples/math) example for a simple contract that hashes a piece of data using keccak256, recovers a [Secp256k1](https://crates.io/crates/secp256k1/) signature to its public key, and verifies a Secp256k1 signature, producing an event with the results.
* Haneul now supports [shared objects](../build/objects.md#shared) that anyone can read or write to. For an example of creating and accessing a shared object, see [Shared Object](https://examples.haneul.io/basics/shared-object.html#shared-object) on https://examples.haneul.io/.
* Interact with the Haneul network using our new [Rust SDK](../build/rust-sdk.md), a collection of Rust language [JSON-RPC wrapper and crypto utilities](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-sdk).
* Haneul now supports development using [Microsoft Windows 11, macOS, and Linux](../build/install.md#supported-oses). See [install Haneul](../build/install.md#prerequisites) for the prerequisites of each operating system.

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
See the [Haneul CLI client Quick Start](../build/cli-client.md) for information about installation, querying the chain, client setup, sending transfer transactions, and viewing the effects.

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
* [Set up and configure a local Haneul network](../build/cli-client.md)
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
