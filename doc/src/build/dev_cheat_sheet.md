---
title: Haneul Developer Cheat Sheet
---

Quick reference on best practices for Haneul Network developers.

# Move

### General

- Read about [package upgrades](https://docs.haneul.io/build/package-upgrades) and write upgrade-friendly code:
    - Packages are immutable, so anyone can call buggy package code forever. Add protections at the object level instead.
    - If you upgrade a package `P` to `P'`, other packages and clients that depend on `P` will continue using `P`, not auto-update to `P'`. Both dependent packages and client code must be explicitly updated to point at `P'`.
    - Packages that expect to be extended by dependent packages can avoid breaking their extensions with each upgrade by providing a standard (unchanging) interface that all versions conform to. See this example for [message sending](https://github.com/wormhole-foundation/wormhole/blob/74dea3bf22f0e27628b432c3e9eac05c85786a99/haneul/wormhole/sources/publish_message.move) across a bridge from Wormhole. Extension packages that produce messages to send can use [`prepare_message`](https://github.com/wormhole-foundation/wormhole/blob/74dea3bf22f0e27628b432c3e9eac05c85786a99/haneul/wormhole/sources/publish_message.move#L68-L90) from any version of the Wormhole package to produce a [`MessageTicket`](https://github.com/wormhole-foundation/wormhole/blob/74dea3bf22f0e27628b432c3e9eac05c85786a99/haneul/wormhole/sources/publish_message.move#L52-L66) while client code to send the message must pass that `MessageTicket` into [`publish_message`](https://github.com/wormhole-foundation/wormhole/blob/74dea3bf22f0e27628b432c3e9eac05c85786a99/haneul/wormhole/sources/publish_message.move#L92-L152) in the latest version of the package.
    - `public` function signatures cannot be deleted or changed, but `public(friend)` functions can. Use `public(friend)` or private visibility liberally unless you are exposing library functions that will live forever.
    - It is not possible to delete `struct` types, add new fields (though you can add [dynamic fields](https://docs.haneul.io/devnet/build/programming-with-objects/ch5-dynamic-fields)), or add new [abilities](https://move-language.github.io/move/abilities.html) via an upgrade. Introduce new types carefully—they will live forever!
- Use `vector`-backed collections (`vector`, `VecSet`, `VecMap`, `PriorityQueue`) with a **known** maximum size of ≤ 1000 items.
    - Use dynamic field-backed collections (`Table`, `Bag`, `ObjectBag`, `ObjectTable`, `LinkedTable`) for any collection that allows third-party addition, larger collections, and collections of unknown size.
    - Haneul Move objects have a maximum size of 250KB—any attempt to create a larger object will lead to an aborted transaction. Ensure that your objects do not have an ever-growing `vector`-backed collection.
- If your function `f` needs a payment in (e.g.) HANEUL from the caller, use `fun f(payment: Coin<HANEUL>)` not `fun f(payment: &mut Coin<HANEUL>, amount: u64)`. This is safer for callers—they know exactly how much they are paying, and do not need to trust `f` to extract the right amount.
- Don’t micro-optimize gas usage. Haneul computation costs are rounded up to the closest *[bucket](https://docs.haneul.io/learn/tokenomics/gas-in-haneul#gas-units)*, so only very drastic changes will make a difference. In particular, if your transaction is already in the lowest cost bucket, it can’t get any cheaper.
- Follow the [Move coding conventions](https://move-language.github.io/move/coding-conventions.html) for consistent style.

### Composability

- Use the [`display`](https://docs.haneul.io/build/haneul-object-display) standard to customize how your objects show up in wallets, apps, and explorers
- Avoid “self-transfers”—whenever possible, instead of writing `transfer::transfer(obj, tx_context::sender(ctx))`, return `obj` from the current function. This allows a caller or [programmable transaction block](https://docs.haneul.io/build/prog-trans-ts-sdk) to use `obj`.

### Testing

- Use [`haneul::test_scenario`](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/packages/haneul-framework/sources/test/test_scenario.move) to mimic multi-transaction, multi-sender test scenarios.
- Use the [`haneul::test_utils`](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/packages/haneul-framework/sources/test/test_utils.move#L5) module for better test error messages via `assert_eq`, debug printing via `print`, and test-only destruction via `destroy`.
- Use `haneul move test --coverage` to compute code coverage information for your tests, and `haneul move coverage source --module <name>` to see uncovered lines highlighted in red. Push coverage all the way to 100% if feasible.

#### Declaring test modules as friends

Consider the following module:

```move
module package::mod {
    fun foo() {}
}
```

If you don't intend the function `mod::foo()` to be `public`, but you want the ability to test it outside
of `package::mod`, then create a test module and declare it a `friend` of the function's module in `package::mod`.

To make the test function available, change the `mod::foo()` function declaration to `public(friend)`, as in the
following example:

```move
module package::mod {
    friend package::test_mod;

    public(friend) fun foo() {}
}
```

As mentioned previously, you can always change the signatures of `public(friend)` functions in future package versions.

When using commands such as `haneul move build` and `haneul move coverage` with these modules, you must include the `--test` flag. See [Build and Test the Haneul Move Package](move/build-test.md#building-your-package) for more information on building packages.

# Apps

- For optimal performance and data consistency, apps should submit writes and reads for the same full node. In the TS SDK, this means that apps should use the wallet's [`signTransactionBlock`](https://haneul-wallet-kit.vercel.app/) API, then submit the transaction via a call to [`execute_transactionBlock`](https://docs.haneul.io/haneul-jsonrpc#haneul_executeTransactionBlock) on the app's full node, *not* use the wallet's `signAndExecuteTransactionBlock` API. This ensures read-after-write-consistency--reads from the app's full node will reflect writes from the transaction right away instead of waiting for a checkpoint.
- For lower latency, use [`executeTransactionBlock`](https://docs.haneul.io/haneul-jsonrpc#haneul_executeTransactionBlock) with `"showEffects": false` and `"showEvents": false` if your app needs to know that a transaction was confirmed, but does not immediately need to see the transaction effects or read the objects/events written by the transaction.
- Apps should implement a local cache for frequently read data rather than over-fetching from the full node.
- Whenever possible, use [programmable transaction blocks](https://docs.haneul.io/build/prog-trans-ts-sdk) to compose existing on-chain functionality rather than publishing new smart contract code. Programmable transaction blocks allow large-scale batching and heterogenous composition, driving already-low gas fees down even further.
- Apps should leave gas budget, gas price, and coin selection to the wallet. This gives wallets more flexibility, and it’s the wallet’s responsibility to dry run a transaction to ensure it doesn't fail.

# Signing

- **Never** sign two concurrent transactions that are touching the same owned object. Either use independent owned objects, or wait for one transaction to conclude before sending the next one. Violating this rule might lead to client [equivocation](https://docs.haneul.io/learn/haneul-glossary#equivocation), which locks up the owned objects involved in the two transactions until the end of the current epoch.
- Any `haneul client` command that crafts a transaction (e.g., `haneul client publish`, `haneul client call`) can accept the `--serialize-unsigned-transaction` flag to output a base64 transaction to be signed.
- Haneul supports several [signature schemes](https://docs.haneul.io/learn/cryptography/haneul-offline-signing) for transaction signing, including native [multisig](https://docs.haneul.io/learn/cryptography/haneul-multisig).
