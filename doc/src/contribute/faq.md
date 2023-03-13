---
title: Haneul Frequently Asked Questions
---

This page contains answers to frequently asked questions (FAQs) about Haneul and Haneul Labs. 
Ask more in the [Haneul Discord](https://discord.gg/haneul) server.

## What does Haneul offer over other blockchains?

Haneul offers ease of development, a developer interface, fast transaction speeds, a sane object model, and better security. Haneul calls the [consensus protocol](../learn/architecture/consensus.md) only for transactions affecting objects owned by multiple addresses. This means simple transactions complete almost immediately.

Additional resources:

* [Why Move?](../learn/why-move)
* [How Haneul Move differs from Core Move](../learn/haneul-move-diffs.md)
* [How Haneul Works](../learn/how-haneul-works.md)
* [Haneul Compared to Other Blockchains](../learn/haneul-compared.md)
* [Narwhal and Bullshark, Haneul's Consensus Engine](../learn/architecture/consensus.md)

## Can I buy Haneul tokens?

We will have a public token, called HANEUL, for the Haneul Mainnet. But it is not available right now and there is no timeline as of yet. Anyone who claims otherwise (offering tokens, whitelists, pre-sale, etc.) is running a scam.

## How can I join the Haneul network? How do I participate in the Haneul project?

Join our [Discord](https://discord.gg/haneul) and follow our [Twitter](https://twitter.com/HaneulNetwork) for the latest updates and announcements.

You can also join the [Move](https://discord.gg/8prNjUqyFj) and [Haneul](https://discord.gg/CVcnUzKYCB) developer channels in Discord.

## Are you looking for partners?

We are seeking partners that can contribute to the ecosystem, primarily in development, by building apps with the SDK now so they can be ready to launch when the network goes live. If interested, please apply using the [Haneul Partnerships Form](https://bit.ly/haneulform).

## After I publish a Move package, how do I update it?

Packages are immutable objects, and this property is relied upon in several places. To update the package you need to publish an updated package.

## Is there any information on node architecture and running validators on Haneul?

See the [Haneul Smart Contract Platform](https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf) for node architecture information.

See the instructions to [run a Haneul Fullnode](../build/fullnode.md).

## Is Haneul compatible with Ethereum Virtual Machine (EVM)?

No. Haneul heavily leverages the Move's asset-centric data model for its novel parallel execution and commitment scheme. This is simply not possible with the EVM data model. Because assets are represented as entries in dynamically indexable maps, it is not possible to statically determine which assets a transaction will touch.

To be blunt: even if we preferred the EVM/Solidity to Move, we could not use them in Haneul without sacrificing the performance breakthroughs that make Haneul unique. And of course, we think there are many reasons why Move is a safer and more developer-friendly language than the EVM.

See [Why move?](../learn/why-move.md) for more details on this. In addition, see the [Move Problem Statement](https://github.com/GeunhwaJeong/awesome-move/blob/main/docs/problem_statement.md) for why we think that - despite being the most popular smart contract language of today - EVM is holding back the crypto space.

Finally, the EVM developer community is very small--approximately 4,000 programmers according to [this study](https://medium.com/electric-capital/electric-capital-developer-report-2021-f37874efea6d). Compare this to (e.g.) the [>20M registered iOS developers](https://techcrunch.com/2018/06/04/app-store-hits-20m-registered-developers-at-100b-in-revenues-500m-visitors-per-week). Thus, the practical path to scaling the smart contract dev community is to bring folks in from the broader population, not to pull them from the tiny pool of existing Solidity developers. We think Move is much safer and much more approachable for mainstream programmers than the EVM.

## Is Haneul an L2, or are there plans to support L2s?

Haneul tackles scaling at the base layer rather than via L2s. We think this approach leads to a more user and developer-friendly system than adding additional complexity on top of an already-complex base layer that doesn't scale.
