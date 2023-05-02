---
title: Run a Haneul Full Node
---

**Note:** These instructions are for advanced users. If you just need a local development environment, you should instead follow the instructions in [Create a Local Haneul Network](haneul-local-network.md) to create a local Full node, validators, and faucet.

Haneul Full nodes validate blockchain activities, including transactions, checkpoints, and epoch changes. Each Full node stores and services the queries for the blockchain state and history.

This role enables [validators](../learn/architecture/validators.md) to focus on servicing and processing transactions. When a validator commits a new set of transactions (or a block of transactions), the validator pushes that block to all connected Full nodes that then service the queries from clients.

## Features

Haneul Full nodes:

- Track and verify the state of the blockchain, independently and locally.
- Serve read requests from clients.

## State synchronization

Haneul Full nodes sync with validators to receive new transactions on the network.

A transaction requires a few round trips to 2f+1 validators to form a transaction certificate (TxCert).

This synchronization process includes:

1.  Following 2f+1 validators and listening for newly committed transactions.
1.  Making sure that 2f+1 validators recognize the transaction and that it reaches finality.
1.  Executing the transaction locally and updating the local DB.

This synchronization process requires listening to at a minimum 2f+1 validators to ensure that a Full node has properly processed all new transactions. Haneul will improve the synchronization process with the introduction of checkpoints and the ability to synchronize with other Full nodes.

## Architecture

A Haneul Full node is essentially a read-only view of the network state. Unlike validator nodes, Full nodes cannot sign transactions, although they can validate the integrity of the chain by re-executing transactions that a quorum of validators previously committed.

Today, a Haneul Full node maintains the full history of the chain.

Validator nodes store only the latest transactions on the _frontier_ of the object graph (for example, transactions with >0 unspent output objects).

## Full node setup

Follow the instructions here to run your own Haneul Full.

### Hardware requirements

Suggested minimum hardware to run a Haneul Full node:

- CPUs: 8 physical cores / 16 vCPUs
- RAM: 128 GB
- Storage (SSD): 2 TB NVMe drive

### Software requirements

Haneul recommends running Haneul Full nodes on Linux. Haneul supports the Ubuntu and
Debian distributions. You can also run a Haneul Full node on macOS.

Make sure to update [Rust](../build/install.md#rust).

Use the following command to install additional Linux dependencies.

```shell
sudo apt-get update \
&& sudo apt-get install -y --no-install-recommends \
tzdata \
libprotobuf-dev \
ca-certificates \
build-essential \
libssl-dev \
libclang-dev \
pkg-config \
openssl \
protobuf-compiler \
git \
clang \
cmake
```

## Configure a Full node

You can configure a Haneul Full node either using Docker or by building from
source.

### Using Docker Compose

Follow the instructions in the [Full node Docker Readme](https://github.com/GeunhwaJeong/haneul/tree/main/docker/fullnode#readme) to run a Haneul Full node using Docker, including [resetting the environment](https://github.com/GeunhwaJeong/haneul/tree/main/docker/fullnode#reset-the-environment).

### Setting up a local Haneul repository

You must get the latest source files from the Haneul GitHub repository.

1. Set up your fork of the Haneul repository:
   1. Go to the [Haneul repository](https://github.com/GeunhwaJeong/haneul) on GitHub and click the **Fork** button in the top right-hand corner of the screen.
   1. Clone your personal fork of the Haneul repository to your local machine
      (ensure that you insert your GitHub username into the URL):
      ```shell
      git clone https://github.com/<YOUR-GITHUB-USERNAME>/haneul.git
      ```
1. `cd` into your `haneul` repository:
   ```shell
   cd haneul
   ```
1. Set up the Haneul repository as a git remote:
   ```shell
   git remote add upstream https://github.com/GeunhwaJeong/haneul
   ```
1. Sync your fork:
   ```shell
   git fetch upstream
   ```
1. Check out the branch associated with the network version you want to run (for example, `devnet` to run a Devnet Full node):
   ```shell
   git checkout --track upstream/<BRANCH-NAME>
   ```

### Setting up a Full node from source

Open a Terminal or Console to the `haneul` directory you downloaded in the previous steps to complete the following:

1.  Install the required [Prerequisites](../build/install.md#prerequisites).
1.  Make a copy of the [Full node YAML template](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-config/data/fullnode-template.yaml):
    ```shell
    cp crates/haneul-config/data/fullnode-template.yaml fullnode.yaml
    ```
1.  Download the genesis blob for the network to use:
    - [Devnet genesis blob](https://github.com/GeunhwaJeong/haneul-genesis/raw/main/devnet/genesis.blob):
      ```shell
      curl -fLJO https://github.com/GeunhwaJeong/haneul-genesis/raw/main/devnet/genesis.blob
      ```
    - [Testnet genesis blob](https://github.com/GeunhwaJeong/haneul-genesis/raw/main/testnet/genesis.blob):
      ```shell
      curl -fLJO https://github.com/GeunhwaJeong/haneul-genesis/raw/main/testnet/genesis.blob
      ```
    - [Mainnet genesis blob](https://github.com/GeunhwaJeong/haneul-genesis/raw/main/mainnet/genesis.blob)
      ```shell
      curl -fLJO https://github.com/GeunhwaJeong/haneul-genesis/raw/main/mainnet/genesis.blob
      ```
1.  Testnet Full nodes only: Edit the `fullnode.yaml` file to include peer nodes for state synchronization. Append the following to the end of the current configuration:
      ```shell
      p2p-config:
        seed-peers:
          - address: /dns/ewr-tnt-ssfn-00.testnet.haneul.io/udp/8084
            peer-id: df8a8d128051c249e224f95fcc463f518a0ebed8986bbdcc11ed751181fecd38
          - address: /dns/lax-tnt-ssfn-00.testnet.haneul.io/udp/8084
            peer-id: f9a72a0a6c17eed09c27898eab389add704777c03e135846da2428f516a0c11d
          - address: /dns/lhr-tnt-ssfn-00.testnet.haneul.io/udp/8084
            peer-id: 9393d6056bb9c9d8475a3cf3525c747257f17c6a698a7062cbbd1875bc6ef71e
          - address: /dns/mel-tnt-ssfn-00.testnet.haneul.io/udp/8084
            peer-id: c88742f46e66a11cb8c84aca488065661401ef66f726cb9afeb8a5786d83456e
      ```
1.  Optional: Skip this step to accept the default paths to resources. Edit the `fullnode.yaml` file to use custom paths.

- Update the `db-path` field with the path to the Full node database.
  ```yaml
  db-path: "/db-files/haneul-fullnode"
  ```
- Update the `genesis-file-location` with the path to `genesis.blob`.
  ```yaml
  genesis:
    genesis-file-location: "/haneul-fullnode/genesis.blob"
  ```

### Starting services

At this point, your Haneul Full node is ready to connect to the Haneul network.

1.  Open a Terminal or Console to the `haneul` directory.
1.  Start the Haneul Full node:
    ```shell
    cargo run --release --bin haneul-node -- --config-path fullnode.yaml
    ```
1.  Optional: [Publish/subscribe](event_api.md#subscribe-to-haneul-events) to notifications using JSON-RPC via websocket.

If your setup is successful, your Haneul Full node is now connected to the appropriate network.

Your Full node serves the read endpoints of the [Haneul JSON-RPC API](../build/json-rpc.md#haneul-json-rpc-api) at: `http://127.0.0.1:9000`.

### Troubleshooting

If you receive a `cannot find -lpq` error, you are missing the `libpq` library. Use `sudo apt-get install libpq-dev` to install on Linux, or `brew install libpq` on MacOS. After you install on MacOS, create a Homebrew link using `brew link --force libpq`. For further context, reference the [issue on Stack Overflow](https://stackoverflow.com/questions/70313347/ld-library-not-found-for-lpq-when-build-rust-in-macos?rq=1).

If you receive the following error:

```
panicked at 'error binding to 0.0.0.0:9184: error creating server listener: Address already in use (os error 98)
```

Then update the metrics address in your fullnode.yaml file to use port `9180`.

```
metrics-address: "0.0.0.0:9180"
```

## Haneul Explorer with your Full node

[Haneul Explorer](https://haneulexplorer.com/) supports connections to custom RPC URLS and local networks. You can point the Explorer to your local Full node and see the transactions it syncs from the network.

1.  Open a browser and go to: https://haneulexplorer.com/
1.  Click **Mainnet** in the network drop-down at the top right-hand corner (or three bars on smaller screens) and select **Local** to connect to a local network, or select **Custom RPC URL** and then enter the URL.

Haneul Explorer displays information about the selected network.

## Monitoring

Monitor your Full node using the instructions at [Logging, Tracing, Metrics, and Observability](../contribute/observability.md).

The default metrics port is `9184`. To change the port, edit your `fullnode.yaml` file.

## Update your Full node

Whenever Haneul releases a new version, you must update your Full node with the release to ensure compatibility with the network it connects to. For example, if you use Haneul Testnet you should install the version of Haneul running on Haneul Testnet. 

### Update with Docker Compose

Follow the instructions to [reset the environment](https://github.com/GeunhwaJeong/haneul/tree/main/docker/fullnode#reset-the-environment),
namely by running the command:

```shell
docker-compose down --volumes
```

### Update from source

If you followed the instructions for [Building from Source](#building-from-source), use the following steps to update your Full node:

1.  Shut down your running Full node.
1.  `cd` into your local Haneul repository:
    ```shell
    cd haneul
    ```
1.  Remove the database and 'genesis.blob' file:
    ```shell
    rm -r haneuldb genesis.blob
    ```
1.  Fetch the source from the latest release:
    ```shell
    git fetch upstream
    ```
1.  Reset your branch:
    ```shell
    git checkout -B <BRANCH-NAME> --track upstream/<BRANCH-NAME>
    ```
1.  Download the latest genesis blob:
    - [Devnet genesis blob](https://github.com/GeunhwaJeong/haneul-genesis/raw/main/devnet/genesis.blob):
      ```shell
      curl -fLJO https://github.com/GeunhwaJeong/haneul-genesis/raw/main/devnet/genesis.blob
      ```
    - [Testnet genesis blob](https://github.com/GeunhwaJeong/haneul-genesis/raw/main/testnet/genesis.blob) - supported only when there is an active public Testnet network
      ```shell
      curl -fLJO https://github.com/GeunhwaJeong/haneul-genesis/raw/main/testnet/genesis.blob
      ```
1.  Update your `fullnode.yaml` configuration file if needed.
1.  Restart your Haneul Full node:
    ```shell
    cargo run --release --bin haneul-node -- --config-path fullnode.yaml
    ```

Your Full node starts on: `http://127.0.0.1:9000`.
