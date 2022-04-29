---
title: Building Haneul
---

Now that you've [learned about Haneul](../learn/index.md), it's time to start building.

## Workflow

Here is our recommended workflow to interact with Haneul:

1. [Install](../build/install.md) all of the *required tools*.
1. [Quickstart](../build/move.md) Move *smart contract*s:
   1. [Write](../build/move.md#writing-a-package) a package.
   1. [Test](../build/move.md#testing-a-package) a package.
   1. [Debug](../build/move.md#debugging-a-package) a package.
   1. [Publish](../build/move.md#publishing-a-package) a package.
1. [Create](../build/wallet.md#genesis) and [Start](../build/wallet.md#starting-the-network) a *local Haneul network*.
1. [Start](../build/json-rpc.md#start-local-rpc-server) a *local JSON-RPC Gateway server*.
1. [Connect](../build/wallet.md#rpc-gateway) to the Haneul network Gateway service with the *Haneul Wallet*.
1. Build dApps:
   1. [Use](../build/json-rpc.md) *Haneul RPC Server and JSON-RPC API* to interact with a local Haneul network.
   1. [Employ](../build/haneul-json.md) *HaneulJSON format* to align JSON inputs more closely with Move call arguments.


## Related concepts

And if you haven't already, become familiar with these key Haneul concepts:

* [Validators](../learn/architecture/validators.md) - The Haneul network is operated by a set of independent validators, each running its own instance of the Haneul software on a separate machine (or a sharded cluster of machines operated by the same entity).
* [Objects](../build/objects.md) - Haneul has programmable objects created and managed by Move packages (a.k.a. smart contracts). Move packages themselves are also objects. Thus, Haneul objects can be partitioned into two categories mutable data values and immutable packages.
* [Transactions](../build/transactions.md) - All updates to the Haneul ledger happen via a transaction. This section describes the transaction types supported by Haneul and explains how their execution changes the ledger.

Find answers to common questions about our [roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md) and more in our [FAQ](../contribute/faq.md).
