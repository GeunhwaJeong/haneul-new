---
title: Haneul Exchange Integration FAQ
---

The Haneul blockchain is still in its development stages, therefore many of the solutions provided in this topic are interim. Please do not hesitate to contact us if you are having issues with the information provided.

## Where are the Haneul Developer Docs?

* Haneul Documentation Portal: [https://docs.haneul.io/](https://docs.haneul.io/)
* Haneul REST API's: [https://docs.haneul.io/haneul-jsonrpc](https://docs.haneul.io/haneul-jsonrpc)
* Run a Full node: [https://docs.haneul.io/devnet/build/fullnode](https://docs.haneul.io/devnet/build/fullnode)

## When will Testnet be live?

Testnet Wave 2 ended on 02/15/23. Information about subsequent Testnet waves will be provided when available.

## General questions about Haneul

This section includes general questions about the Haneul platform.

### What is the difference between the devnet branch and the main branch of the Haneul repo?

The main branch contains all the latest changes. The `devnet` branch reflects the binary that is currently running on the Devnet network.

### Can I get contract information through the RPC API?

Yes, contracts are also stored in objects. You can use the haneul_getObject to fetch the object. Example: [https://explorer.haneul.io/objects/0xe70628039d00d9779829bb79d6397ea4ecff5686?p=31](https://explorer.haneul.io/objects/0xe70628039d00d9779829bb79d6397ea4ecff5686?p=31)

**Note:** You can see only the deserialized bytecode (as opposed to Source code).

### Can I get the information in the contract, such as the total amount of the currency issued and the number of decimal places?
    
There's no contract-level storage in Haneul. In general, this contract-level information is usually stored in an object or event. For example, we store decimals in this object [https://github.com/GeunhwaJeong/haneul/blob/1aca0465275496e40f02a674938def962126412b/crates/haneul-framework/sources/coin.move#L36](https://github.com/GeunhwaJeong/haneul/blob/1aca0465275496e40f02a674938def962126412b/crates/haneul-framework/sources/coin.move#L36). And in this case we provide an convenience [RPC endpoint](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-json-rpc/src/api.rs#L91-L97).

### Is the gas price dynamic? Is it available through JSON-RPC?

Yes, the gas price is dynamic and exposed via the [haneul_getReferenceGasPrice](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-json-rpc/src/api.rs#L337-L339) endpoint.

### How can I delete an object within Haneul?

You can delete objects (in most cases) only if the Move module that defines the object type includes a Move function that can delete the object, such as when a Move contract writer explicitly wants the object to be deletable).[https://docs.haneul.io/devnet/build/programming-with-objects/ch2-using-objects#option-1-delete-the-object](https://docs.haneul.io/devnet/build/programming-with-objects/ch2-using-objects#option-1-delete-the-object)

If the delete function is defined in the Move module, you can delete the object by invoking the Move call using CLI or wallet. Here’s an example:

 1. Create an example NFT using the Haneul Client CLI: [https://docs.haneul.io/devnet/build/cli-client#create-an-example-nft](https://docs.haneul.io/devnet/build/cli-client#create-an-example-nft).

 2. Call this Move [function](https://github.com/GeunhwaJeong/haneul/blob/21c26ce6a5d4e3448abd74323e3164286d3deba6/crates/haneul-framework/sources/devnet_nft.move#L69-L72) with the CLI by following [https://docs.haneul.io/devnet/build/cli-client#calling-move-code](https://docs.haneul.io/devnet/build/cli-client#calling-move-code).

### What is the denomination of Haneul？

GEUNHWA is the smallest unit of a HANEUL Coin. 1 HANEUL equals 1 billion GEUNHWA, and 1 GEUNHWA equals 10^-9 of a HANEUL.

## Transactions

Questions about transaction in Haneul. 

### How can we subscribe to transaction events?

There are "Move events" that are emitted by Move code, and "transaction events" such as object transfers, creations, and deletions. [https://docs.haneul.io/build/pubsub#events](https://docs.haneul.io/build/pubsub#events) has a list of all the events you can subscribe to via the pub/sub API and their structure.

### Can I get the corresponding transaction serial number through TransactionDigest?

As a best practive, don't rely on the transaction serial number because there's no total ordering of transactions on Haneul. The transaction serial numbers differ between different Full nodes.
    
### Is the paged transaction data obtained by different nodes the same?

No, the ordering will be different on different nodes for now, while we are still working on checkpoints. After checkpoint process is complete, the ordering will be the same on all nodes
    
### Is there a nonce or timestamp mechanism for transactions?

There are no nonce or timestamps in our transaction data structure at the moment

### What is the transaction expiry window?

Transactions don't expire.

## Staking and delegation

The following questions are related to staking on the Haneul network.

### How does HANEUL staking work?

Sample staking implementation:

The entry functions for staking are defined in [this module](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/sources/governance/haneul_system.move). Relevant functions include:
 * [`request_add_stake`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_add_stake)
 * [`request_add_stake_with_locked_coin`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_add_stake_with_locked_coin)
 * [`request_withdraw_stake`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_withdraw_stake)
 * [`request_add_delegation`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_add_delegation)
 * [`request_add_delegation_with_locked_coin`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_add_delegation_with_locked_coin)
 * [`request_withdraw_delegation`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_withdraw_delegation)

The first three functions are for validator staking for themselves while the rest are for delegated staking.

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

Use the [`request_add_delegation`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_add_delegation) and [`request_add_delegation_with_locked_coin`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_add_delegation_with_locked_coin) methods to add to the staked amount.

Use the [`request_withdraw_delegation`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_withdraw_delegation) method to withdraw all or part of the delegation.

### Does Haneul require a bonding / warm-up period?

Yes, the specifics are still under consideration.

### Does Haneul require an un-bonding / cool-down period?

Yes, the current un-bonding period under consideration.

### Are staking rewards auto-compounded?

Yes, Haneul uses a staking pool approach inspired by liquidity pools. Rewards are added to the pool and auto-compounded through the appreciation of pool token value relative to HANEUL tokens.

### Do rewards appear as inbound/outbound on-chain transactions?

Yes, rewards are added to the staking pool through a special system transaction at epoch boundaries.

### How long does it take to get the first reward after staking? How frequently are rewards paid out?

Rewards are compounded every epoch, and paid out when you withdraw your stake. You must stake for the entire duration of an epoch to receive rewards for that epoch.

### Is there a minimum and maximum staking amount (for validation and delegation)?

There will be a minimum amount required and a maximum amount allowed, as well as limits on stake changes within an epoch.

 * Validation: Requires a high minimum amount of HANEUL.
 * Delegation: No minimum

Specific amounts will be provided when available.

### How does slashing work, and what are the penalties?

There will not be slashing for the principal stake allocated. Instead, validators will get penalized by having fewer future rewards when these get paid out. Rewards that have already been accrued are not at risk.

### Does Haneul support on-chain governance or voting?

On-chain governance is not implemented for Haneul. There is no plan to add it in the near future.

### Where can I find block details?

Answers to the following questions address specific details around surfacing block details.

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

### How do I query for balance changes?

The following solution is interim:
Use the `BalanceChangeEvent` using the event query API. `BalanceChangeEvent` was added in October 2022 in this [PR](https://github.com/GeunhwaJeong/haneul/pull/5289).

### How do I track block generation?

Haneul uses checkpoints, but this is still under development. Checkpoints are like blocks that are created periodically (likely every few minutes), but generated asynchronously instead of on the critical path for execution. Each checkpoint contains all of the confirmed transactions since the previous checkpoint.

A significant amount of the performance benefits of Haneul comes from carefully separating the work required to actually finalize a transaction from bookkeeping work, like checkpoint construction. We're experimenting with a number of different generation intervals, and this may change over time as we better understand the traffic patterns.

**Temporary solution** For now, we have created one block per transaction until Checkpoints become available. See sample [here](https://github.com/GeunhwaJeong/haneul/blob/91a5e988a91b41d920a082f3de3c2c7372627b00/crates/haneul-rosetta/src/state.rs#L61-L74).

    ```rust
    #[async_trait]
    pub trait BlockProvider {
        async fn get_block_by_index(&self, index: u64) -> Result<BlockResponse, Error>;
        async fn get_block_by_hash(&self, hash: BlockHash) -> Result<BlockResponse, Error>;
        async fn current_block(&self) -> Result<BlockResponse, Error>;
        fn genesis_block_identifier(&self) -> BlockIdentifier;
        async fn oldest_block_identifier(&self) -> Result<BlockIdentifier, Error>;
        async fn current_block_identifier(&self) -> Result<BlockIdentifier, Error>;
        async fn get_balance_at_block(
            &self,
            addr: HaneulAddress,
            block_height: u64,
        ) -> Result<u128, Error>;
    }
    ```

### How are transactions proposed by validators if they're not included in blocks? Does a validator propose blocks or just individual transactions?

Validators form a certificate (a quorum of signatures) for each transaction, and then propose checkpoints consisting of certificates since the last checkpoint. You can read more in section 4.3 [here](https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf).

### How do I get test Devnet coins?

- You can find our [faucet in Discord](https://discord.com/channels/916379725201563759/971488439931392130).

### How can I get in touch and request more information?

- Please visit our [Discord server](https://discord.gg/haneul).
