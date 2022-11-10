---
title: Run a Haneul Full node
---

Haneul Full nodes validate blockchain activities, including transactions, checkpoints, and epoch changes. Each Full node stores and services the queries for the blockchain state and history.

This role enables [validators](../learn/architecture/validators.md) to focus on servicing and processing transactions. When a validator commits a new set of transactions (or a block of transactions), the validator pushes that block to all connected Full nodes that then service the queries from clients.

## Features

Haneul Full nodes:

* Track and verify the state of the blockchain, independently and locally.
* Serve read requests from clients.

## State synchronization

Haneul Full nodes sync with validators to receive new transactions on the network.

A transaction requires a few round trips to 2f+1 validators to form a transaction certificate (TxCert).

This synchronization process includes:

1. Following 2f+1 validators and listening for newly committed transactions.
1. Making sure that 2f+1 validators recognize the transaction and that it reaches finality.
1. Executing the transaction locally and updating the local DB.

This synchronization process requires listening to at a minimum 2f+1 validators to ensure that a Full node has properly processed all new transactions. Haneul will improve the synchronization process with the introduction of checkpoints and the ability to synchronize with other Full nodes.

## Architecture

A Haneul Full node is essentially a read-only view of the network state. Unlike
validator nodes, full nodes cannot sign transactions, although they can validate
the integrity of the chain by re-executing transactions that were previously
committed by a quorum of validators.

Today, a Haneul Full node maintains the full history of the chain.

Validator nodes store only the latest transactions on the *frontier* of the object graph (for example, transactions with >0 unspent output objects).

## Full node setup

Follow the instructions here to run your own Haneul Full node.

### Hardware requirements

Minimum hardware requirements for running a Haneul Full node:

* CPUs: 10 core
* RAM: 32 GB
* Storage: 1 TB

### Software requirements

We recommend running Haneul Full nodes on Linux. Haneul supports the Ubuntu and
Debian distributions. You can also run a Haneul Full node on macOS.

Make sure to update [Rust](../build/install.md#rust).

Use the following command to install additional Linux dependencies.
```shell
    $ apt-get update \
    && apt-get install -y --no-install-recommends \
    tzdata \
    ca-certificates \
    build-essential \
    pkg-config \
    cmake
```

## Configure a Full node

You can configure a Haneul Full node either using Docker or by building from
source.

### Using Docker Compose

Follow the instructions in the [Full node Docker README](https://github.com/GeunhwaJeong/haneul/tree/main/docker/fullnode#readme) to run a Haneul Full node using Docker, including [resetting the environment](https://github.com/GeunhwaJeong/haneul/tree/main/docker/fullnode#reset-the-environment).

### Building from source

1. Install the required [Prerequisites](../build/install.md#prerequisites).
1. Set up your fork of the Haneul repository:
    1. Go to the [Haneul repository](https://github.com/GeunhwaJeong/haneul) on GitHub
       and click the *Fork* button in the top right-hand corner of the screen.
    1. Clone your personal fork of the Haneul repository to your local machine
       (ensure that you insert your GitHub username into the URL):
       ```shell
       $ git clone https://github.com/<YOUR-GITHUB-USERNAME>/haneul.git
       ```
1. `cd` into your `haneul` repository:
    ```shell
    $ cd haneul
    ```
1. Set up the Haneul repository as a git remote:
    ```shell
    $ git remote add upstream https://github.com/GeunhwaJeong/haneul
    ```
1. Sync your fork:
    ```shell
    $ git fetch upstream
    ```
1. Check out the `devnet` branch:
    ```shell
    $ git checkout --track upstream/devnet
    ```
1. Make a copy of the [Full node YAML template](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-config/data/fullnode-template.yaml):
   ```shell
   $ cp crates/haneul-config/data/fullnode-template.yaml fullnode.yaml
   ```
1. Download the [`genesis`](https://github.com/GeunhwaJeong/haneul-genesis/raw/main/devnet/genesis.blob) state for Devnet:
    ```shell
    $ curl -fLJO https://github.com/GeunhwaJeong/haneul-genesis/raw/main/devnet/genesis.blob
    ```
1. Optional: Skip this step to accept the default paths to resources. Edit the `fullnode.yaml` file to use custom paths.
   * Update the `db-path` field with the path to the Full node database.
       ```yaml
       db-path: "/db-files/haneul-fullnode"
       ```
   * Update the `genesis-file-location` with the path to `genesis.blob`.
       ```yaml
       genesis:
       genesis-file-location: "/haneul-fullnode/genesis.blob"
       ```
1. Start your Haneul Full node:
    ```shell
    $ cargo run --release --bin haneul-node -- --config-path fullnode.yaml
    ```
1. Optional: [Publish / subscribe](event_api.md#subscribe-to-haneul-events) to notifications using JSON-RPC via websocket.

Your Full node will now be serving the read endpoints of the [Haneul JSON-RPC
API](../build/json-rpc.md#haneul-json-rpc-api) at:
`http://127.0.0.1:9000`

## Haneul Explorer with your Full node

[Haneul Explorer](https://explorer.devnet.haneul.io/) supports connections to custom RPC URLS and local networks. You can point the Explorer to your local Full node and see the
transactions it syncs from the network. To make this change:

1. Open a browser and go to: https://explorer.devnet.haneul.io/
1. Click the **Devnet** button in the top right-hand corner of Haneul Explorer and select
   **Local** from the drop-down menu.
1. Close the **Choose a Network** menu to see the latest transactions. 

Haneul Explorer now uses your local Full node to explore the state of the chain.

## Monitoring

Monitor your Full node using the instructions at [Logging, Tracing, Metrics, and
Observability](../contribute/observability.md).

Note the default metrics port is 9184. To change the port, edit your `fullnode.yaml` file.

## Update your Full node

Whenever Haneul releases a new version, Devnet restarts as a new network with no data. You must update your Full node with each Haneul release to ensure compatibility with the network.

### Update with Docker Compose

Follow the instructions to [reset the environment](https://github.com/GeunhwaJeong/haneul/tree/main/docker/fullnode#reset-the-environment),
namely by running the command:
```shell
$ docker-compose down --volumes
```

### Update from source

If you followed the instructions for [Building from
Source](#building-from-source), update your Full node as follows:

1. Shut down your currently running Full node.
1. `cd` into your local Haneul repository:
    ```shell
    $ cd haneul
    ```
1. Remove the old on-disk database and 'genesis.blob' file:
    ```shell
    $ rm -r haneuldb genesis.blob
    ```
1. Fetch the source from the latest release:
    ```shell
    $ git fetch upstream
    ```
1. Reset your branch:
    ```shell
    $ git checkout -B devnet --track upstream/devnet
    ```
1. Download the latest [`genesis`](https://github.com/GeunhwaJeong/haneul-genesis/raw/main/devnet/genesis.blob) state for Devnet as described above.
1. Update your `fullnode.yaml` configuration file if needed.
1. Restart your Haneul Full node:
    ```shell
    $ cargo run --release --bin haneul-node -- --config-path fullnode.yaml
    ```
Your Full node starts on:
`http://127.0.0.1:9000`

## Future plans

Today, a Full node relies only on synchronizing with 2f+1 validators in order to
ensure it has seen all committed transactions. In the future, we expect
Full nodes to fully participate in a peer-to-peer (p2p) environment where the
load of disseminating new transactions can be shared with the whole network and
not place the burden solely on the validators. We also expect future
features, such as checkpoints, to enable improved performance of synchronizing the
state of the chain from genesis.

Please see our [privacy policy](https://haneul.io/policy/) to learn how we handle
information about our nodes.
