---
title: Exchange Integration FAQ
---

**This is outdated/deprecated content**

---

## Haneul Exchange Integration FAQs

Get answers to common questions about Haneul.

### How to change the amount of an existing stake?

During the staking period, you can add to or withdraw your stake from a validator. To modify your stake amount you can use the following functions:
 * Use the `request_add_stake` and `request_add_stake_with_locked_coin` methods to add to the staked amount.
 * Use the `request_withdraw_stake` method to withdraw staked HANEUL.

### How is a staking transaction different from a typical transaction regarding construction, signing, and broadcasting?

Staking transactions are Move call transactions that call specific Move functions in the [haneul_system](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/sources/governance/haneul_system.move) module of the Haneul Framework. The staking transaction uses a shared object, and is no different from other shared object transactions.

### Is there a minimum and maximum staking amount (for Validators and user staking)?

There will be a minimum amount required, as well as limits on stake changes within an epoch.

 * **Validators:** Requires a minimum of 30 Million HANEUL to join as a Haneul Validator. Validators must maintain a minimum of 20 Million HANEUL to continue as a validator. Any validator that drops below 15 Million HANEUL is removed at the next epoch boundary.
 * **User staking:** The minimum amount to stake with a validator is 1 HANEUL.

### How to stake and un-stake HANEUL?

Haneul Wallet supports both stake and un-staking. Staking via Move code or the Haneul CLI is also possible - the relevant functions are in the [haneul_system](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/sources/governance/haneul_system.move) module.

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

You can delete objects (in most cases) only if the Move module that defines the object type includes a Move function that can delete the object, such as when a Move contract writer explicitly wants the object to be deletable.[https://docs.haneul.io/devnet/build/programming-with-objects/ch2-using-objects#option-1-delete-the-object](https://docs.haneul.io/devnet/build/programming-with-objects/ch2-using-objects#option-1-delete-the-object)

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

Yes, an address can own multiple coins of different amounts. Haneul supports staking coins owned by an address to different validators. The minimum user staking amount is 1 HANEUL.

### Can I use one account address to stake with multiple validators?

Yes, if an address owns multiple coins, you can stake each coin with a different validator.

### Can I change the amount of an existing stake during the staking period?

Yes, you can add to or withdraw your stake from a validator. Use the following methods to modify the stake amount:

Use the [`request_add_stake`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_add_stake) and [`request_add_stake_with_locked_coin`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_add_stake_with_locked_coin) methods to add to the staked amount.

Use the [`request_withdraw_stake`](https://github.com/GeunhwaJeong/haneul/blob/58229627970a6e9ff558b156c1cb193f246eaf88/crates/haneul-framework/docs/haneul_system.md#0x2_haneul_system_request_withdraw_stake) method to withdraw all or part of the stake.

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
      "method": "haneul_queryTransactions",
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

