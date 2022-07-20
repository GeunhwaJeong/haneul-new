---
title: How Haneul Works
---

This document has two main goals: It first provides a high-level overview of Haneul, presenting its main functionalities and design choices. It then compares Haneul with existing blockchains allowing potential adopters to decide whether Haneul fits their use cases.

This document is written for engineers, developers, and technical readers knowledgeable about the crypto space. It does not assume deep programming language or distributed systems expertise. See the [Haneul white paper](https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf) for a much deeper explanation of how Haneul works. See [How Haneul Differs from Other Blockchains](haneul-compared.md) for a high-level overview of the differences in approach between Haneul and other blockchain systems.

## tl;dr

The Haneul blockchain operates at a speed and scale previously thought unimaginable. Haneul assumes the typical blockchain transaction is a simple transfer and optimizes for that use. Haneul does this by making each request idempotent, holding network connections open longer, and ensuring transactions complete immediately. Haneul optimizes for single-writer objects, allowing a design that forgoes consensus for simple transactions.

Instead of the traditional blockchain’s fire-and-forget broadcast, Haneul ensures a two-way handshake between the requestor and approving validators, with simple transactions having near instant finality. With this low latency, transactions can easily be incorporated into games and other settings that need completion in real time. Furthermore, Haneul supports smart contracts written in Move, a language designed for blockchains with strong inherent security and a more understandable programming model.

In a world where the cost of bandwidth is diminishing steadily, we are creating an ecosystem of services that will find it easy, fun, and perhaps profitable to ensure transaction voting on behalf of users.

## Components

Become familiar with these key Haneul concepts:

* [Objects](../build/objects.md) - Haneul has programmable objects created and managed by Move packages (a.k.a. smart contracts). Move packages themselves are also objects. Thus, Haneul objects can be partitioned into two categories: mutable data values and immutable packages.
* [Transactions](../build/transactions.md) - All updates to the Haneul ledger happen via a transaction. This section describes the transaction types supported by Haneul and explains how their execution changes the ledger.
* [Validators](../learn/architecture/validators.md) - The Haneul network is operated by a set of independent validators, each running its own instance of the Haneul software on a separate machine (or a sharded cluster of machines operated by the same entity).

## Architecture
Haneul is a distributed ledger that stores a collection of programmable *[objects](../build/objects.md)*, each with a globally unique ID. Every object is owned by a single *address*, and each address can own an arbitrary number of objects.

The ledger is updated via a *[transaction](../build/transactions.md)* sent by a particular address. A transaction can create, destroy, and write objects, as well as transfer them to other addresses.

Structurally, a transaction contains a set of input object references and a pointer to a Move code object that already exists in the ledger. Executing a transaction produces updates to the input objects and (if applicable) a set of freshly created objects along with their owners. A transaction whose sender is address *A* can accept objects owned by *A*, shared objects, and objects owned by other objects in the first two groups as input.

```mermaid
flowchart LR
    CC(CLI Client) --> ClientService
    RC(Rest Client) --> ClientService
    RPCC(RPC Client) --> ClientService
    ClientService --> AuthorityAggregator
    AuthorityAggregator --> AC1[AuthorityClient] & AC2[AuthorityClient]
    subgraph Authority1
      AS[AuthorityState]
    end
    subgraph Authority2
      AS2[AuthorityState]
    end
    AC1 <==>|Network TCP| Authority1
    AC2 <==>|Network TCP| Authority2
```

Haneul validators agree on and execute transactions in parallel with high throughput using [Byzantine Consistent Broadcast](https://en.wikipedia.org/wiki/Byzantine_fault).

## System overview

This section is written for a technical audience wishing to gain more insight about how Haneul achieves its main performance and security objectives.

Haneul assumes the typical blockchain transaction is a user-to-user transfer or asset manipulation and optimizes for that scenario. As a result, Haneul distinguishes between two types of assets (i) owned objects that can only be modified by its specific owner, and (ii) shared objects that have no specific owners and can be modified by more than one user. This distinction allows a design that forgoes consensus for simple transactions involving only owned objects to achieve very low latency.

Haneul mitigates a major hindrance to blockchain growth: [head-of-line blocking](https://en.wikipedia.org/wiki/Head-of-line_blocking). Blockchain nodes maintain an accumulator that represents the state of the entire blockchain, such as the latest certified transactions. Nodes participate in a consensus protocol to add an update to that state reflecting the transaction’s modification to blocks (add, remove, mutate). That consensus protocol leads to an agreement on the state of the blockchain before the increment, the validity and haneultability of the state update itself, and the state of the blockchain after the increment. On a periodic basis, these increments are collected in the accumulator.

In Haneul, this consensus protocol is required only when the transaction involves shared objects. For this, Haneul offers the [Narwhal and Tusk](https://github.com/GeunhwaJeong/narwhal) DAG-based mempool and efficient Byzantine Fault Tolerant (BFT) consensus. When shared objects are involved, the Haneul validators play the role of more active validators in other blockchains to totally order the transaction with respect to other transactions accessing shared objects.

Because Haneul focuses on managing specific objects rather than a single aggregation of state, it also reports on them in a unique way: (i) every object in Haneul has a unique version number, and (ii) every new version is created from a transaction that may involve several dependencies, themselves versioned objects.

As a consequence, a Haneul validator – or any other entity with a copy of the state – can exhibit a causal history of an object, showing its history since genesis. Haneul explicitly makes the bet that in many cases, the ordering of that causal history with the causal history of another object is irrelevant; and in the few cases where this information is relevant, Haneul makes this relationship explicit in the data.

Haneul guarantees transaction processing obeys *[eventual consistency](https://en.wikipedia.org/wiki/Eventual_consistency)* in the [classical sense](https://hal.inria.fr/inria-00609399/document). This breaks down in two parts:

* Eventual delivery - if one honest validator processes a transaction, all other honest validators will eventually do the same.
* Convergence - two validators that have seen the same set of transactions share the same view of the system (reach the same state).

But contrary to a blockchain, Haneul does not stop the flow of transactions in order to witness the convergence.

## Simple transactions

[Many transactions](https://eprint.iacr.org/2019/611.pdf) do not have complex interdependencies with other, arbitrary parts of the blockchain state. Often financial users just want to send an asset to a recipient, and the only data required to gauge whether this simple transaction is admissible is a fresh view of the sender's account. This observation allows Haneul to forgo [consensus](https://pmg.csail.mit.edu/papers/osdi99.pdf) and instead use simpler algorithms based on [Byzantine Consistent Broadcast](https://link.springer.com/book/10.1007/978-3-642-15260-3). See our list of potential [single-writer apps](single-writer-apps.md) for examples of real-world simple transactions.

These protocols are based on the [FastPay](https://arxiv.org/abs/2003.11506) design that comes with peer-reviewed security guarantees. In a nutshell, Haneul takes the approach of taking a lock (or "stopping the world") only for the relevant piece of data rather than the whole chain. In this case, the only information needed is the sender account, which can then send only one transaction at a time.

Haneul further expands this approach to more involved transactions that may explicitly depend on multiple elements under their sender's control, using Move’s object model and leveraging Move's strong ownership model. By requiring that dependencies be explicit, Haneul applies a _multi-lane_ approach to transaction validation, making sure those independent transaction flows can progress without impediment from the others.

Haneul validates transactions individually, rather than batching them into traditional blocks. The key advantage of this approach is low latency; each successful transaction quickly obtains a certificate of finality that proves to anyone the transaction will be processed by the Haneul network.

The process of submitting a Haneul transaction is thus a bit more involved than in traditional blockchains. Whereas a usual blockchain can accept a bunch of transactions from the same author in a fire-and-forget mode, Haneul transaction submission follows these steps:

1. The sender broadcasts a transaction to all Haneul validators.
2. Each Haneul validator replies with an individual vote for this transaction. Each vote has a certain weight based on the stake owned by the validator.
3. The sender collects a Byzantine-resistant-majority of these votes into a _certificate_ and broadcasts that back to all Haneul validators. This settles the transaction, ensuring _finality_ that the transaction will not be dropped (revoked).
4. Optionally, the sender collects a certificate detailing the effects of the transaction.

While those steps demand more of the sender, performing them efficiently can still yield a cryptographic proof of finality with minimum latency. Aside from crafting the original transaction itself, the session management for a transaction does not require access to any private keys and can be delegated to a third party. Haneul takes advantage of this observation to provide [Haneul Gateway services](#haneul-gateway-services).


## Complex contracts

Complex smart contracts may benefit from shared objects where more than one user can mutate those objects (following smart contract specific rules). In this case, Haneul totally orders all transactions involving shared objects using a consensus protocol. Haneul uses a novel peer-reviewed consensus protocol based on [Narwhal](https://arxiv.org/abs/2105.11827). This is state-of-the-art in terms of both performance and robustness.

Transactions involving shared objects also contain at least one owned object to pay for gas fees. It is thus essential to carefully compose the protocol dealing with owned objects with the protocol sequencing the transaction to guarantee Haneul’s security properties. When shared objects are involved, transaction submission follows these steps:

1. The sender broadcasts a transaction to all Haneul validators.
2. Each Haneul validator replies with an individual vote for this transaction. Each vote has a certain weight based on the stake owned by the validator.
3. The sender collects a Byzantine-resistant-majority of these votes into a certificate and broadcasts it back to all Haneul validators. _This time however, the certificate is sequenced through Byzantine Agreement._
4. Once the transaction has been successfully sequenced, the user broadcasts again the certificate to the validators to settle the transaction.

## Scalability

As mentioned, Haneul does not impose a total order on the transactions containing only owned objects. Instead, transactions are [causally ordered](haneul-compared.md#causal-order-vs-total-order). If a transaction `T1` produces an output object `O1` used as input objects in a transaction `T2`, a validator must execute `T1` before it executes `T2`. Note that `T2` does not need to use these objects directly for a causal relationship to exist, e.g., `T1` might produce output objects which are then used by `T3`, and `T2` might use `T3`'s output objects. However, transactions with no causal relationship can be processed by Haneul validators in any order. This insight allows Haneul to massively parallelize execution, and shard it across multiple machines.

Haneul employs the [state-of-the-art Narwhal consensus protocol](https://arxiv.org/abs/2105.11827) to totally order transactions involving shared objects. The consensus sub-system also scales in the sense that it can sequence more transactions by adding more machines per validator.

## Haneul Gateway services

The Haneul model encourages third parties to assist with transaction submissions. For example, if an app developer (e.g., a game developer) has many users, they can manage votes aggregation and certificate submission on behalf of their users. The app developer may use their own servers (e.g., where they store the state of the game) to run a _Haneul Gateway service_. We provide a reference implementation of such a service.

Instead of the app users attempting to send transactions to multiple validators from their mobile device, which may degrade user experience, users may submit their transactions to the app, which forwards it to the Haneul Gateway service run by the app developer. The Haneul Gateway service conducts the entire transaction session and returns the results to the users. Security is assured since the app doesn’t need to know the users’ private keys; the app owner merely provides the bandwidth.

More specifically, this service plays the role of an accumulator and makes sure the transaction is received by a quorum of validators, collects a quorum of votes, submits the certificate to the validators, and replies to the client. The Haneul Gateway is trusted for availability only and not safety.

In a world where the cost of bandwidth is diminishing steadily, Haneul fosters an ecosystem of services that will find it easy, fun, and perhaps profitable to ensure transaction voting and certificates broadcast on behalf of end-users.

## Smart contract programming

Haneul smart contracts are written in the [Move language](https://github.com/GeunhwaJeong/awesome-move/blob/main/README.md). Move is safe and expressive, and its type system and data model naturally support the parallel agreement/execution strategies that make Haneul scalable. Move is an open source programming language for building smart contracts originally developed at [Meta](http://meta.com) for the [Diem blockchain](https://www.diem.com). The language is platform-agnostic, and in addition to being adopted by Haneul, it has been gaining popularity on other platforms (e.g., [0L](https://0l.network), [StarCoin](https://starcoin.org/en/)).

Find a more thorough explanation of Move’s features in:

* the [Move Programming Language book](https://github.com/move-language/move/blob/main/language/documentation/book/src/introduction.md)
* Haneul-specific [Move instructions](../build/move.md) and [differences](haneul-move-diffs.md) on this site
* the [Haneul whitepaper](https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf) and its formal description of Move in the context of Haneul
