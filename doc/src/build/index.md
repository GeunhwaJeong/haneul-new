---
title: Building Haneul
---

Now that you've [learned about Haneul](../learn/index.md), it's time to start building.

## Workflow

Here is our recommended workflow to interact with Haneul:

1. [Install](../build/install.md) all of the *required tools*.
1. [Connect](../build/devnet.md) to the Haneul Devnet network.
1. [Create](../build/move/index.md) *smart contracts* with Move:
   1. [Write](../build/move/write-package.md) a package.
   1. [Build and test](../build/move/build-test.md) a package.
   1. [Debug and publish](../build/move/debug-publish.md) a package.
1. [Program objects](../build/programming-with-objects/index.md) in Haneul:
   1. [Learn](../build/programming-with-objects/ch1-object-basics.md) object basics.
   1. [Pass](../build/programming-with-objects/ch2-using-objects.md) Move objects as arguments, mutating objects, deleting objects.
   1. [Freeze](../build//programming-with-objects/ch3-immutable-objects.md) an object, using immutable objects.
   1. [Wrap](../build/programming-with-objects/ch4-object-wrapping.md) objects in another object.
   1. [Enable](../build/programming-with-objects/ch5-child-objects.md) objects to own other objects.
1. [Talk](../build/comms.md) with Haneul using our API and SDKs:
   * [Use](../build/json-rpc.md) the *Haneul RPC Server and JSON-RPC API* to interact with a local Haneul network.
   * [Employ](../build/haneul-json.md) *HaneulJSON format* to align JSON inputs more closely with Move call arguments.
   * [Follow](https://docs.haneul.io/haneul-jsonrpc) the Haneul API Reference.
   * [Make](../build/rust-sdk.md) Rust SDK calls to Haneul from your app.
   * [Write](https://github.com/GeunhwaJeong/haneul/tree/main/sdk/typescript/) TypeScript/JavaScript apps.
   * [Run](../build/fullnode.md) a Haneul Fullnode and [subscribe](../build/pubsub.md) to events.
1. Optionally, [create](../contribute/cli-client.md#genesis) and [start](../contribute/cli-client.md#starting-the-network) a *local Haneul network* to contribute to the blockchain.

Find answers to common questions about our [roadmap](https://github.com/GeunhwaJeong/haneul/blob/main/ROADMAP.md) and more in our [FAQ](../contribute/faq.md).
