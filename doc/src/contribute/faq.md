---
title: Haneul Frequently Asked Questions
---

This page contains answers to frequently asked questions (FAQs) about Haneul and Haneul Labs. 
Ask more in [Discord](https://discord.gg/haneul), see where we are headed in our
[roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md#roadmap), and
find full details in our
[white paper](https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf).


## Why Haneul?


### Where can I find out about Haneul?

Use these online resources:

* Haneul Website: [https://haneul.io/](https://haneul.io/)
* Haneul Developer Portal: [https://docs.haneul.io/](https://docs.haneul.io/)
* Haneul Smart Contract White Paper: [https://haneul.io/whitepaper](https://haneul.io/whitepaper) 
* SDK reference: [Haneul JSON-RPC](https://playground.open-rpc.org/?uiSchema%5BappBar%5D%5Bui:splitView%5D=false&schemaUrl=https://raw.githubusercontent.com/HaneulLabs/haneul/main/haneul/open_rpc/spec/openrpc.json&uiSchema%5BappBar%5D%5Bui:input%5D=false)  


### What does Haneul offer over other blockchains?

Haneul offers ease of development, a developer interface, fast transaction speeds, a sane object model, and better security. Haneul calls the consensus protocol only for transactions affecting objects owned by multiple accounts. This means simple transactions complete almost immediately.

See these resources on the [Haneul Developer Portal](https://docs.haneul.io/) for the complete story on why we built Haneul:



* [Why Move?](../learn/why-move)
* [How Haneul Move differs from Core Move](../learn/haneul-move-diffs.md)
* [How Haneul Works](../learn/how-haneul-works.md)
* [Haneul Compared to Other Blockchains](../learn/haneul-compared.md)


### Is Haneul based on Diem?

There is no technical relationship between Diem and Haneul except that both use Move.

All five co-founders (as well as several Haneullabs employees) worked on the Diem system and are very familiar with both its good qualities and its limitations. Diem was designed to handle light payments traffic between a small number (10s-100s) of custodial wallets. There were eventual visions of evolving it into a more scalable system that is capable of handling more general-purpose smart contracts; however, the original architecture was not designed to support this and has not evolved significantly.

When we started Haneullabs, we had the option to build on top of Diem but chose not to because of these limitations. We believe blockchain tech has evolved a lot since Diem came out in 2019, and we have many ideas about how to design a system that is more scalable and programmer-friendly from the ground up. That is why we built Haneul.


### What is the relationship between Haneul/Haneullabs and Aptos?

There is no relationship between Haneul/Haneullabs and Aptos. The similarity is that they both use Move; but Haneul has a different object model. The research behind the [block STM paper](https://arxiv.org/abs/2203.06871) was all done at Facebook. Subsequently, some of the authors joined Haneullabs and some joined Aptos. The paper carries the current affiliations of the authors.


## Roadmap


### Can I buy the Haneul token?

We will have a public token for the Haneul Mainnet. But it is not available right now and there is no timeline as of yet. Anyone who claims otherwise (offering tokens, whitelists, pre-sale, etc.) is running a scam.

### Will the token name be HANEUL?

Yes, the token name will be HANEUL.


### When is the Haneul Devnet/Testnet/Mainnet launching?

We have launched our [Haneul Devnet](../explore/devnet.md) in May 2022. A Testnet is coming in a few months. 

See our roadmap: \
[https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md#roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md#roadmap) 


### Is there some kind of waitlist for the Testnet?

No. More information is forthcoming.


## Getting involved


### How can I join the Haneul network? How do I participate in the Haneul project?

Join our [Discord](https://discord.gg/haneul) and follow our [Twitter](https://twitter.com/Haneul_Labs) for the latest updates and announcements.

For developers:

1. [Install Haneul](https://docs.haneul.io/build/install).
2. Spin up a [Haneul Fullnode](../build/fullnode.md).
3. Experiment with the [Haneul CLI client](../build/cli-client.md).
4. Join the [Move](https://discord.gg/8prNjUqyFj) and [Haneul](https://discord.gg/CVcnUzKYCB) developer channels in Discord.
5. [Start building](https://docs.haneul.io/build)!

### Are there things we can already try if we are not a developer?

We have a number of examples and demos available for viewing at:
https://docs.haneul.io/explore

### Are you looking for partners?

We are seeking partners that can contribute to the ecosystem primarily in development by building apps with the SDK now so they can be ready to launch when the network goes live. If interested, please apply using the [Haneul Partnerships Form](https://bit.ly/haneulform).


### With whom can I talk about a partnership or strategic investment? How can I discuss Haneul in person?
Start in [Discord](https://discord.gg/haneul).


### Where can I see the investors of the project?

See the [haneul-labs.com](https://haneul-labs.com/) website for company details.


### Do you need moderators in Discord? Can I be the mod for my country?

Haneul Community Mod Program is officially accepting applications. [Apply here](https://bit.ly/haneulmods)


### I'm looking for someone from the Haneullabs Crew to speak at an event - is there someone I can DM/email?

Ask in Discord.


## Development

### Once a Move package is published to the Haneul network, is there any way for devs to update the code?

Not currently. Packages are immutable objects, and this property is relied upon in several places.


### Is there any information on node architecture and running validators on Haneul?

Section four in the [Haneul Smart Contract Platform](https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf) white paper is the best reference for node architecture.

See the instructions to [run a Haneul Fullnode](../build/fullnode.md).


### Can I run a Haneul validator node?

We have a public [Haneul Devnet](../explore/devnet.md) with nodes operated by Haneul Labs. And you can now run a networked [Haneul Fullnode](../build/fullnode.md).

You may also run a local, non-networked Haneul node for development using the [Haneul CLI client](../build/cli-client.md). You can interact with the nodes with either the [Haneul Wallet CLI](../build/cli-client.md#command-line-mode) or over the [Haneul RPC API](../build/json-rpc.md).

Soon we will have a Testnet that allow others to operate Haneul nodes, but both are on our [roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md#roadmap). Right now, 

### What does a local node do and what are the hardware requirements to run it? How is it different from a Testnet node?

A local node allows you to immediately start building software using Move for Haneul, and can be run on commodity hardware. See the [Haneul Fullnode](../build/fullnode.md) documentation for system requirements.

A Testnet node will be part of a live network of validators.


## Technology


### Is Haneul compatible with Ethereum Virtual Machine (EVM)?

No. Haneul heavily leverages the Move's asset-centric data model for its novel parallel execution and commitment scheme. This is simply not possible with the EVM data model. Because assets are represented as entries in dynamically indexable maps, it is not possible to statically determine which assets a transaction will touch.

To be blunt: even if we preferred the EVM/Solidity to Move, we could not use them in Haneul without sacrificing the performance breakthroughs that make Haneul unique. And of course, we think there are many reasons why Move is a safer and more developer-friendly language than the EVM.

See [Why move?](../learn/why-move.md) for more details on this. In addition, see the [Move Problem Statement](https://github.com/GeunhwaJeong/awesome-move/blob/main/docs/problem_statement.md) for why we think that - despite being the most popular smart contract language of today - EVM is holding back the crypto space.

Finally, the EVM developer community is very small--approximately 4,000 programmers according to [this study](https://medium.com/electric-capital/electric-capital-developer-report-2021-f37874efea6d). Compare this to (e.g.) the [>20M registered iOS developers](https://techcrunch.com/2018/06/04/app-store-hits-20m-registered-developers-at-100b-in-revenues-500m-visitors-per-week/#:~:text=Today%20at%20WWDC%2C%20Apple's%20CEO,500%20million%20visitors%20per%20week.). Thus, the practical path to scaling the smart contract dev community is to bring folks in from the broader population, not to pull them from the tiny pool of existing Solidity developers. We think Move is much safer and much more approachable for mainstream programmers than the EVM.

### Is Haneul an L2, or are there plans to support L2s?

Haneul tackles scaling at the base layer rather than via L2s. We think this approach leads to a more user and developer-friendly system than adding additional complexity on top of an already-complex base layer that doesn't scale.


### Does Haneullabs maintain a fork of Move?

No. Move is designed to be a cross-platform language that can be used anywhere you need safe smart contracts. There are some more details on how this works + the chains Move runs in the [Awesome Move](https://github.com/GeunhwaJeong/awesome-move) documentation.
