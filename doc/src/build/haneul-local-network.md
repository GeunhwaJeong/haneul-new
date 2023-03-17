---
title: Create a local Haneul network
---

To test your dApps against the latest changes or to prepare for new features ahead of the next Devnet or Testnet release, you can test on a local network using the `haneul-test-validator` binary. This binary starts a single-node cluster with Full node and faucet capabilities.

## Prerequisite

[Install](../build/install.md) the required libraries if not already installed.

## Install Haneul

You can install Haneul from your local repository or from the remote repository. If you build from your local source, you have the benefit of being able to run a local Haneul Explorer and Haneul Wallet.

To run from your local source, clone the repository locally (or get latest, if already cloned). Then, run `cargo build` from the `haneul` directory:

```bash
# Clone the repository
git clone https://github.com/GeunhwaJeong/haneul.git
# Make haneul the working directory
cd haneul
# Build Haneul
cargo build --bin haneul-test-validator --bin haneul
```

To use remote code, `cargo install` Haneul directly from the remote repository. The following example uses the `main` branch, but you can set other branches as needed (e.g., `--branch devnet`, `--branch testnet`, and so on) to target different network versions.

```bash
cargo install --locked --git https://github.com/GeunhwaJeong/haneul.git --branch main haneul-test-validator haneul
```

## Running local network

To run a local network with validators and a faucet, open a Terminal or Console window at the `haneul` root directory. Use the following command to run `haneul-test-validator`, setting `RUST_LOG` to `consensus=off`:

```bash
RUST_LOG="consensus=off" cargo run --bin haneul-test-validator
```

**Note** The state for `haneul-test-validator` is currently not persistent, i.e., it will always start from a fresh state upon restart.

You can customize your local Haneul network by passing values to the following flags for the `haneul-test-validator` command:

```bash
OPTIONS:
        --epoch-duration-ms <EPOCH_DURATION_MS>
            The duration for epochs (defaults to one minute) [default: 60000]

        --faucet-port <FAUCET_PORT>
            Port to start the Haneul faucet on [default: 9123]

        --fullnode-rpc-port <FULLNODE_RPC_PORT>
            Port to start the Fullnode RPC server on [default: 9000]
```

Use `haneul-validator-test --help` to see these options in your console.

### Making faucet requests

To get gas coins for an address, open a new Terminal or Conaole window or tab. Make a cURL request with the address you want to receive the coins. Use the `haneul client active-address` command to get the current active address, if needed.

```bash
curl --location --request POST 'http://127.0.0.1:9123/gas' \
--header 'Content-Type: application/json' \
--data-raw '{
    "FixedAmountRequest": {
        "recipient": "0x<ADDRESS>"
    }
}'
```

If successful, the response resembles the following:

```bash
{
    "transferredGasObjects": [
        {
            "amount": 200000000,
            "id": "0x192ce62506ed8705b76e8423be1f6e011064a3f887ba924605f27a8c83c8c970",
            "transferTxDigest": "7sp4fFPH2WaUgvN43kjDzCpEhKfifqjx5RTki74y8T3E"
        },
        {
            "amount": 200000000,
            "id": "0x31d003ade00675d1ab82b225bfcceaa60bb993f5d90e9d0aa88f81dc24ec14d6",
            "transferTxDigest": "7sp4fFPH2WaUgvN43kjDzCpEhKfifqjx5RTki74y8T3E"
        },
        {
            "amount": 200000000,
            "id": "0x98cbdc93ae672110f91bc0c39c0c87bc66f36984c79218bb2c0bac967260970c",
            "transferTxDigest": "7sp4fFPH2WaUgvN43kjDzCpEhKfifqjx5RTki74y8T3E"
        },
        {
            "amount": 200000000,
            "id": "0xba66aee6289cc6d0203c451bea442ad30d4cfe699e50b36fed0ff3e99ba51529",
            "transferTxDigest": "7sp4fFPH2WaUgvN43kjDzCpEhKfifqjx5RTki74y8T3E"
        },
        {
            "amount": 200000000,
            "id": "0xd9f0b521443d66227eddc2aac2e16f667ca9caeef9f1b7afb4a6c2fc7dcb58d8",
            "transferTxDigest": "7sp4fFPH2WaUgvN43kjDzCpEhKfifqjx5RTki74y8T3E"
        }
    ],
    "error": null
}
```

### Accessing Full node

You can access your Full node using cURL:

```bash
curl --location --request POST 'http://127.0.0.1:9000' \
--header 'Content-Type: application/json' \
--data-raw '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "haneul_getTotalTransactionNumber",
  "params": []
}'
```

If successful, the return resembles the following:

```bash
{
    "jsonrpc": "2.0",
    "result": 168,
    "id": 1
}
```

## Setup local Haneul Explorer

While [https://explorer.haneul.io/?network=local](https://explorer.haneul.io/?network=local) is compatible with the local network, it might not have all the latest features that are available in the `main` branch of the Haneul repository. To run `explorer` locally, open a Terminal or Console window in the `haneul` directory (install [pnpm](https://pnpm.io/installation) first if you don't already have it):

```bash
pnpm explorer dev
```

After running the command, you can open a browser to [http://localhost:3000/](http://localhost:3000/) to access your local version of Haneul Explorer.

For more details, see [https://github.com/GeunhwaJeong/haneul/tree/main/apps/explorer](https://github.com/GeunhwaJeong/haneul/tree/main/apps/explorer).

## Set up local Haneul Wallet

Similar to local Haneul Explorer, you can also setup a local Haneul Wallet. Open a Terminal or Console window or tab at the `haneul` root directory and use the `wallet start` command (install [pnpm](https://pnpm.io/installation) first if you don't already have it):

```bash
pnpm wallet start
```

**Tips** You can set the default environment to be local network with https://github.com/GeunhwaJeong/haneul/tree/main/apps/wallet#environment-variables so that you don't have to switch network manually.

For more details, reference [https://github.com/GeunhwaJeong/haneul/tree/main/apps/wallet](https://github.com/GeunhwaJeong/haneul/tree/main/apps/wallet).

## Generating example data

Open a Terminal or Console window or tab at the `haneul` root directory. From there, run the TypeScript SDK end to end test against the local network to generate example data to the network (install [pnpm](https://pnpm.io/installation) first if you don't already have it):

```bash
pnpm sdk test:e2e
```

For more details, refer to [https://github.com/GeunhwaJeong/haneul/tree/main/sdk/typescript#testing](https://github.com/GeunhwaJeong/haneul/tree/main/sdk/typescript#testing).
