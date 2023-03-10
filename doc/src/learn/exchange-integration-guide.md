---
title: Haneul Exchange Integration Guide
---

This topic describes how to integrate HANEUL, the token native to the Haneul network, into a cryptocurrency exchange. The specific requirements and processes to implement an integration vary between exchanges. Rather than provide a step-by-step guide, this topic provides information about the primary tasks necessary to complete an integration. After the guidance about how to configure an integration, you can also find information and code samples related to staking on the Haneul network.

## Requirements to configure a HANEUL integration

The requirements to configure a HANEUL integration include:
 * A Haneul Full node. You can operate your own Haneul Full node or use a Full node from a node operator.
 * Suggested hardware requirements to run a Haneul Full node:
    * CPU: 10 core
    * RAM: 32 GB
    * Storage: 1 TB SSD

We recommend running Haneul Full nodes on Linux. Haneul supports the Ubuntu and Debian distributions.

## Configure a Haneul Full node

You can set up and configure a Haneul Full node using Docker or directly from source code in the Haneul GitHub repository.

### Install a Haneul Full node using Docker

Run the command in this section using the same branch of the repository for each. Replace `branch-name` with the branch you use. For example, use `devnet` to use the Haneul Devnet network, or use `testnet` to use the Haneul Testnet network. You must download all files to, and run all commands from, the same folder location.

 1. Install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/). Docker Desktop version installs Docker Compose.
 1. Install dependencies for Linux:
    ```bash
    apt update \
    && apt install -y --no-install-recommends \
    tzdata \
    ca-certificates \
    build-essential \
    pkg-config \
    cmake
    ```
 1. Download the docker-compose.yaml file:
    ```bash
    wget https://github.com/GeunhwaJeong/haneul/blob/branch-name/docker/fullnode/docker-compose.yaml
    ```
 1. Download the fullnode-template.yaml file:
    ```bash
    wget https://github.com/GeunhwaJeong/haneul/raw/branch-name/crates/haneul-config/data/fullnode-template.yaml
    ```
 1. Download the genesis.blob file:
    ```bash
    wget https://github.com/GeunhwaJeong/haneul-genesis/raw/main/branch-name/genesis.blob
    ```
 1. Start the Full node. The -d switch starts it in the background (detached mode).
    ```bash
    docker-compose up -d
    ```

## Install a Haneul Full node from source

Use the steps in this section to install and configure a Haneul Full node directly from the Haneul GitHub repository. These steps use [Cargo](https://doc.rust-lang.org/cargo/), the Rust package manager.

 1. Install prerequisites for Haneul.
 1. Clone the Haneul repository:
    ```bash
    git clone https://github.com/GeunhwaJeong/haneul.git -b branch-name
    ```
    Replace `branch-name` with the branch to use. You should use the same branch for all commands.
 1. Change directories to /haneul:
    ```bash
    cd haneul
    ```
 1. Copy the fullnode.yaml template:
    ```bash
    cp crates/haneul-config/data/fullnode-template.yaml fullnode.yaml
    ```
 1. Download the genesis.blob file:
    ```bash
    wget https://github.com/GeunhwaJeong/haneul-genesis/raw/main/branch-name/genesis.blob
    ```
    Change branch-name to the same branch you used for previous commands.
 1. Optionally, if you installed Haneul to a path other than the default, modify the fullnode.yaml file to use the path you used. Update the path to the folder where you installed haneul-fullnode for the `db-path` and `genesis-file-location` as appropriate:
    `db-path: "/db-files/haneul-fullnode-folder"`
    `genesis-file-location: "/haneul-fullnode-folder/genesis.blob"`
 1. Start you Haneul Full node:
    ```bash
    cargo run --release --bin haneul-node -- --config-path fullnode.yaml
    ```
## Set up Haneul addresses

Haneul addresses do not require on-chain initialization, you own an address if you own the key for the address. You can derive a Haneul address by hashing the signature flag byte + public key bytes. The following code sample demonstrates how to derive a Haneul address in Rust:

```rust
let flag = 0x00; // 0x00 = ED25519, 0x01 = Secp256k1, 0x02 = Secp256r1
// Hash the [flag, public key] bytearray using SHA3-256
let mut hasher = Sha3_256::default();
hasher.update([flag]);
hasher.update(pk);
let g_arr = hasher.finalize();


// The first 32 bytes is the Haneul address.
let mut res = [0u8; HANEUL_ADDRESS_LENGTH]; // HANEUL_ADDRESS_LENGTH = 32
res.copy_from_slice(&AsRef::<[u8]>::as_ref(&g_arr)[..HANEUL_ADDRESS_LENGTH]);
let haneul_address_string = hex::encode(res);
```

## Displaying addresses

Haneul supports both addresses with and without a 0x prefix. Haneul recommends that you always include the 0x prefix in API calls and when you display user addresses.

## Track balance changes for an address

You can track balance changes by calling `haneul_getBalance` at predefined intervals. This call returns the total balance for an address. The total includes any coin or token type, but this document focuses on HANEUL. You can track changes in the total balance for an address between subsequent `haneul_getBalance` requests.

The following bash example demonstrates how to use `haneul_getBalance` for address 0xa38bc2aa63c34e37821f7abb34dbbe97b7ab2ea2. If you use a network other than Devnet, replace the value for `rpc` with the URL to the appropriate Full node.

```bash
rpc="https://fullnode.devnet.haneul.io:443"
address="0xa38bc2aa63c34e37821f7abb34dbbe97b7ab2ea2"
data="{\"jsonrpc\": \"2.0\", \"method\": \"haneul_getBalance\", \"id\": 1, \"params\": [\"$address\"]}"
curl -X POST -H 'Content-type: application/json' --data-raw "$data" $rpc
```

The response is a JSON object that includes the totalBalance for the address:
```json
{
  "jsonrpc":"2.0",
  "result":{
     "coinType":"0x2::haneul::HANEUL",
     "coinObjectCount":40,
     "totalBalance":10000000000,
     "lockedBalance":{

     }
  },
  "id":1
}
```

The following example demonstrates using haneul_getBalance in Rust:
```rust
use std::str::FromStr;
use haneul_sdk::types::base_types::HaneulAddress;
use haneul_sdk::{HaneulClient, HaneulClientBuilder};


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
   let haneul = HaneulClientBuilder::default().build(
      "https://fullnode.devnet.haneul.io:443",
   ).await.unwrap();
   let address = HaneulAddress::from_str("0xa38bc2aa63c34e37821f7abb34dbbe97b7ab2ea2")?;
   let objects = haneul.read_api().get_balance(address).await?;
   println!("{:?}", objects);
   Ok(())
}
```

## Use events to track balance changes for an address

You can also track the balance for an address by subscribing to all of the events emitted from it. Use a filter to include only the events related to HANEUL coins, such as when the address acquires a coin or pays for a gas fee.
The following example demonstrates how to filter events for an address using bash and cURL:

```bash
rpc="https://fullnode.devnet.haneul.io:443"
address="0xa38bc2aa63c34e37821f7abb34dbbe97b7ab2ea2"
data="{\"jsonrpc\": \"2.0\", \"id\":1, \"method\": \"haneul_getEvents\", \"params\": [{\"Recipient\": {\"AddressOwner\": \"0xa38bc2aa63c34e37821f7abb34dbbe97b7ab2ea2\"}}, null, null, true ]}"
curl -X POST -H 'Content-type: application/json' --data-raw "$data" $rpc
```

The response can include a large number of events. Add pagination to the response using the `nextCursor` key in the request. You can determine the corresponding `txDigest` and `eventSeq` from the `id` field of a transaction.

You can add the `txDigest` value instead of the first `null` within the `params`. The second `null` is an integer that defines how many results (up to 1000) to return and the `true` means ascending order. You can use the `nextCursor` so the response starts from a desired point.

The `id` field of any transaction looks like:
```bash
"id": {
         "txDigest": "GZQN9pE3Zr9ZfLzBK1BfVCXtbjx5xKMxPSEKaHDvL3E2",
         "eventSeq": 6019
       }
```

With this data, create a nextCursor as follows:
```bash
nextCursor : {"txDigest": "GZQN9pE3Zr9ZfLzBK1BfVCXtbjx5xKMxPSEKaHDvL3E2","eventSeq": 6019}
```

## Blocks vs Checkpoints

Haneul is a DAG-based blockchain and uses checkpoints for node synchronization and global transaction ordering. Checkpoints differ from blocks in the following ways:
 * Haneul creates checkpoints and adds finalized transactions. Note that transactions are finalized even before they are included in a checkpoint
 * Checkpoints do not fork, roll back, or reorganize.
 * Haneul creates one checkpoint about every 3 seconds.

### Checkpoint API operations

Haneul Checkpoint API operations include:
 * [haneul_getCheckpoint](https://docs.haneul.io/haneul-jsonrpc#haneul_getCheckpoint) - Retrieves the specified checkpoint.
 * [haneul_getLatestCheckpointSequenceNumber](https://docs.haneul.io/haneul-jsonrpc#haneul_getLatestCheckpointSequenceNumber) - Retrieves the sequence number of the most recently executed checkpoint.
 * haneul_getCheckpoints - Retrieves a paginated list of checkpoints that occurred during the specified interval. Pending a future release.

## HANEUL Balance transfer

To transfer a specific amount of HANEUL between addresses, you need a HANEUL token object with that specific value. In Haneul, everything is an object, including HANEUL tokens. The amount of HANEUL in each HANEUL token object varies. For example, an address could own 3 HANEUL tokens with different values: one of 0.1 HANEUL, a second of 1.0 HANEUL, and a third with 0.005 HANEUL. The total balance for the address equals the sum of the values of the individual HANEUL token objects, in this case, 1.105 HANEUL.

You can merge and split HANEUL token objects to create token objects with specific values. To create a HANEUL token worth .6 HANEUL, split the token worth 1 HANEUL into two token objects worth .6 HANEUL and .4 HANEUL.

To transfer a specific amount of HANEUL, you need a HANEUL token worth that specific amount. To get a HANEUL token with that specific value, you might need to split or merge existing HANEUL tokens. Haneul supports several methods to accomplish this, including some that do not require you to manually split or merge coins.

## Haneul API operations for transfers

Haneul supports the following API operations related to transferring HANEUL between addresses:

 * [haneul_transferObject](https://docs.haneul.io/haneul-jsonrpc#haneul_transferObject)
   Because HANEUL tokens are objects, you can transfer HANEUL tokens just like any other object. This method requires a gas token, and is useful in niche cases only.

 * [haneul_payAllHaneul](https://docs.haneul.io/haneul-jsonrpc#haneul_payAllHaneul)
   This method accepts an array of HANEUL token IDs. It merges all existing tokens into one, deducts the gas fee, then sends the merged token to the recipient address.

   The method is especially useful if you want to transfer all HANEUL from an address. To merge together all coins for an address, set the recipient as the same address. This is a native Haneul method so is not considered a transaction in Haneul.

 * [haneul_payHaneul](https://docs.haneul.io/haneul-jsonrpc#haneul_payHaneul)
   This operation accepts an array of HANEUL token IDs, an array of amounts, and an array of recipient addresses.

   The amounts and recipients array map one to one. Even if you use only one recipient address, you must include it for each amount in the amount array.

   The operation merges all of the tokens provided into one token object and settles the gas fees. It then splits the token according to the amounts in the amounts array and sends the first token to the first recipient, the second token to the second recipient, and so on. Any remaining HANEUL on the token stays in the source address.

   The benefits of this method include: no gas fees for merging or splitting tokens, and the abstracted token merge and split. The `haneul_payHaneul` operation is a native function, so the merge and split operations are not considered Haneul transactions. The gas fees for them match typical transactions on Haneul.You can use this operation to split coins in your own address by setting the recipient as your own address. Note that the total value of the input coins must be greater than the total value of the amounts to send.

 * [haneul_pay](https://docs.haneul.io/haneul-jsonrpc#haneul_pay)
   This method is similar to haneul_payHaneul, but it accepts any kind of coin or token instead of only HANEUL. You must include a gas token, and all of the coins or tokens must be the same type.

 * [haneul_transferHaneul](https://docs.haneul.io/haneul-jsonrpc#haneul_transferHaneul)
    This method accepts only one HANEUL token object and an amount to send to the recipient. It uses the same token for gas fees, so the amount to transfer must be strictly less than the value of the HANEUL token used.

## HANEUL Staking and Delegation

The Haneul blockchain uses a Delegated Proof-of-Stake mechanism (DPoS). This allows HANEUL token holders to stake their HANEUL tokens to any validator of their choice. When someone stakes their HANEUL tokens, it means those tokens are locked for the entire epoch. Users can withdraw their stake at any time, but new staking requests become active only at the start of the next epoch.

HANEUL holders who stake their tokens to validators earn rewards for helping secure the Haneul network. Haneul determines rewards for staking based on stake rewards on the network, and distributes them at the end of each epoch.

The total voting power in the Haneul Network is always 10,000. The voting power of each individual validator is similar to basis points. For example, a voting power of 101 = 1.01%. Haneul's quorum threshold (number of votes needed to confirm a transaction) is 6,667 (which is greater than 2/3). The voting power for a single validator is capped at 1,000 (10%) regardless of how much stake the validator has.

## Staking functions

Haneul supports the following API operations related to staking. You can find the source code in the [haneul_system](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/sources/governance/haneul_system.move) module.

 * `request_add_stake`
 Add delegated stake to a validator's staking pool.

```rust
public entry fun request_add_stake(
   self: &mut HaneulSystemState,
   stake: Coin<HANEUL>,
   validator_address: address,
   ctx: &mut TxContext,
) {
   validator_set::request_add_stake(
       &mut self.validators,
       validator_address,
       coin::into_balance(stake),
       option::none(),
       ctx,
   );
}
```

 * `request_add_stake_mul_coin`
 Add delegated stake to a validator's staking pool using multiple coins.

```rust
public entry fun request_add_stake_mul_coin(
   self: &mut HaneulSystemState,
   delegate_stakes: vector<Coin<HANEUL>>,
   stake_amount: option::Option<u64>,
   validator_address: address,
   ctx: &mut TxContext,
) {
   let balance = extract_coin_balance(delegate_stakes, stake_amount, ctx);
   validator_set::request_add_stake(&mut self.validators, validator_address, balance, option::none(), ctx);
}
```

 * `request_add_stake_with_locked_coin`
 Add delegated stake to a validator's staking pool using a locked HANEUL coin.

```rust
public entry fun request_add_stake_with_locked_coin(
   self: &mut HaneulSystemState,
   stake: LockedCoin<HANEUL>,
   validator_address: address,
   ctx: &mut TxContext,
) {
   let (balance, lock) = locked_coin::into_balance(stake);
   validator_set::request_add_stake(&mut self.validators, validator_address, balance, option::some(lock), ctx);
}
```

 * `request_withdraw_stake`
 Withdraw some portion of a delegation from a validator's staking pool.

```rust
public entry fun request_withdraw_stake(
   self: &mut HaneulSystemState,
   delegation: &mut Delegation,
   staked_haneul: &mut StakedHaneul,
   principal_withdraw_amount: u64,
   ctx: &mut TxContext,
) {
   validator_set::request_withdraw_stake(
       &mut self.validators,
       delegation,
       staked_haneul,
       principal_withdraw_amount,
       ctx,
   );
}
```

## Haneul Exchange Integration FAQs

Get answers to common questions about Haneul.

### How to change the amount of an existing stake?

During the staking period, you can add to or withdraw your stake from a validator. To modify your stake amount you can use the following functions:
 * Use the `request_add_stake` and `request_add_stake_with_locked_coin` methods to add to the staked amount.
 * Use the `request_withdraw_stake` method to withdraw your delegation.

### How is a staking transaction different from a typical transaction regarding construction, signing, and broadcasting?

Staking transactions are Move call transactions that call specific Move functions in the [haneul_system](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/sources/governance/haneul_system.move) module of the Haneul Framework. The staking transaction uses a shared object, and is no different from other shared object transactions.

### Is there a minimum and maximum staking amount (for validation and delegation)?

There will be a minimum amount required, as well as limits on stake changes within an epoch.

 * **Validation:** Requires a high minimum amount of HANEUL delegated with each validator to stay in the validator set.
 * **Delegation:** There will be a relatively low minimum amount for each delegation.

Specific amounts to be determined prior to Haneul Mainnet.

### How to stake and un-stake HANEUL?

Haneul Wallet supports both stake and un-staking. Staking via Move code or the Haneul CLI is also possible – the relevant functions are in the [haneul_system](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/sources/governance/haneul_system.move) module.

### Where are the Haneul Developer Docs?

* Haneul Documentation Portal: [https://docs.haneul.io/](https://docs.haneul.io/)
* Haneul REST API's: [https://docs.haneul.io/haneul-jsonrpc](https://docs.haneul.io/haneul-jsonrpc)

### What is the difference between the devnet branch and the main branch of the Haneul repo?

The main branch contains all the latest changes. The `devnet` branch reflects the binary that is currently running on the Devnet network.

### Can I get contract information through the RPC API?

Yes, contracts are also stored in objects. You can use the haneul_getObject to fetch the object. Example: [https://explorer.haneul.io/objects/0xe70628039d00d9779829bb79d6397ea4ecff5686?p=31](https://explorer.haneul.io/objects/0xe70628039d00d9779829bb79d6397ea4ecff5686?p=31)

**Note:** You can see only the deserialized bytecode (as opposed to Source code).

### Can I get the information in the contract, such as the total amount of the currency issued and the number of decimal places?

There's no contract-level storage in Haneul. In general, this contract-level information is usually stored in an object or event. For example, we store decimals in this object [https://github.com/GeunhwaJeong/haneul/blob/1aca0465275496e40f02a674938def962126412b/crates/haneul-framework/sources/coin.move#L36](https://github.com/GeunhwaJeong/haneul/blob/1aca0465275496e40f02a674938def962126412b/crates/haneul-framework/sources/coin.move#L36). And in this case we provide an [RPC endpoint](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-json-rpc/src/api/).

### Is the gas price dynamic? Is it available through JSON-RPC?

Yes, the gas price is dynamic and exposed via the [haneul_getReferenceGasPrice](https://docs.haneul.io/haneul-jsonrpc#haneul_getReferenceGasPrice) endpoint.

### How can I delete an object within Haneul?

You can delete objects (in most cases) only if the Move module that defines the object type includes a Move function that can delete the object, such as when a Move contract writer explicitly wants the object to be deletable).[https://docs.haneul.io/devnet/build/programming-with-objects/ch2-using-objects#option-1-delete-the-object](https://docs.haneul.io/devnet/build/programming-with-objects/ch2-using-objects#option-1-delete-the-object)

If the delete function is defined in the Move module, you can delete the object by invoking the Move call using CLI or wallet. Here’s an example:

 1. Create an example NFT using the Haneul Client CLI: [https://docs.haneul.io/devnet/build/cli-client#create-an-example-nft](https://docs.haneul.io/devnet/build/cli-client#create-an-example-nft).

 2. Call this Move [function](https://github.com/GeunhwaJeong/haneul/blob/21c26ce6a5d4e3448abd74323e3164286d3deba6/crates/haneul-framework/sources/devnet_nft.move#L69-L72) with the CLI by following [https://docs.haneul.io/devnet/build/cli-client#calling-move-code](https://docs.haneul.io/devnet/build/cli-client#calling-move-code).

### What is the denomination of Haneul？

GEUNHWA is the smallest unit of a HANEUL Coin. 1 HANEUL equals 1 billion GEUNHWA, and 1 GEUNHWA equals 10^-9 of a HANEUL.

## Transactions FAQs

Questions about transaction in Haneul.

### How can we subscribe to transaction events?

There are "Move events" that are emitted by Move code, and "transaction events" such as object transfers, creations, and deletions. See the [Haneul Events](../build/event_api.md) topic for a list of all the events you can subscribe to via the pub/sub API and their structure.

### Can I get the corresponding transaction serial number through TransactionDigest?

As a best practice, don't rely on the transaction serial number because there's no total ordering of transactions on Haneul. The transaction serial numbers differ between different Full nodes.

### Is the paged transaction data obtained by different nodes the same?

No, the ordering will be different on different nodes for now, while we are still working on checkpoints. After checkpoint process is complete, the ordering will be the same on all nodes

### Is there a nonce or timestamp mechanism for transactions?

There are no nonce or timestamps in our transaction data structure at the moment

### What is the transaction expiry window?

Transactions don't expire.

### How many validators will Haneul have at Mainnet genesis?

The number is still under consideration. The validator set is not fixed, but validators must apply and then be approved through our validator application process.

### Is the address used for staking the same as the wallet address that owns the staked coins?

Yes, a user/validator stakes using the address that owns the staked coin. There is no special address derivation

### How is a staking transaction different from a typical transaction regarding construction, signing, and broadcasting?

Staking transactions are Move call transactions that call specific Move function in the [Haneul Framework](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/sources/governance/haneul_system.move). The staking transaction uses a shared object, and is no different from other shared object transactions.

### Does Haneul support staking a partial amount of the HANEUL owned by an address?

Yes, an address can own multiple coins of different amounts. Haneul supports staking coins owned by an address to different validators. The minimum staking amount that can be delegated is 1 GEUNHWA which is equal to .000000001 HANEUL.

### Can I use one account address to stake with multiple validators?

Yes, if an address owns multiple coins, you can stake each coin with a different validator.

### Can I change the amount of an existing stake during the staking period?

Yes, you can add to or withdraw your stake from a validator. Use the following methods to modify the stake amount:

Use the [`request_add_stake`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_add_stake) and [`request_add_stake_with_locked_coin`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_add_stake_with_locked_coin) methods to add to the staked amount.

Use the [`request_withdraw_stake`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_withdraw_stake) method to withdraw all or part of the delegation.

### Does Haneul require a bonding / warm-up period?

Yes, the specifics are still under consideration.

### Does Haneul require an un-bonding / cool-down period?

Yes, the current un-bonding period is under consideration.

### Are staking rewards auto-compounded?

Yes, Haneul uses a staking pool approach inspired by liquidity pools. Rewards are added to the pool and auto-compounded through the appreciation of pool token value relative to HANEUL tokens.

### Do rewards appear as inbound/outbound on-chain transactions?

Yes, rewards are added to the staking pool through a special system transaction at epoch boundaries.

### How long does it take to get the first reward after staking? How frequently are rewards paid out?

Rewards are compounded every epoch, and paid out when you withdraw your stake. You must stake for the entire duration of an epoch to receive rewards for that epoch.

### How does slashing work, and what are the penalties?

There will not be slashing for the principal stake allocated. Instead, validators will get penalized by having fewer future rewards when these get paid out. Rewards that have already been accrued are not at risk.

### Does Haneul support on-chain governance or voting?

On-chain governance is not implemented for Haneul. There is no plan to add it in the near future.

### How can I retrieve the current block height or query a block by height using a Haneul endpoint?

Haneul is [DAG](https://cointelegraph.com/explained/what-is-a-directed-acyclic-graph-in-cryptocurrency-how-does-dag-work)-based, so the block-based view of the transaction history is not always the most direct one. To get the latest transaction, use the Transaction Query API:

    ```json
    {
      "jsonrpc": "2.0",
      "id": 1,
      "method": "haneul_getTransactions",
      "params": [
        "All",
        <last known transaction digest>,
        100,
        "Ascending"
      ]
    }
    ```

### How are transactions proposed by validators if they're not included in blocks? Does a validator propose blocks or just individual transactions?

Validators form a certificate (a quorum of signatures) for each transaction, and then propose checkpoints consisting of certificates since the last checkpoint. You can read more in section 4.3 of the [Haneul Smart Contract Platform](https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf).

### How do I get test Devnet coins?

- You can find our [faucet in Discord](https://discord.com/channels/916379725201563759/971488439931392130). You can also request coins from the [Haneul Faucet](../build/faucet.md) programmatically.

### How can I get in touch and request more information?

- Please visit our [Discord server](https://discord.gg/haneul).

