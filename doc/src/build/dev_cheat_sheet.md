---
title: Haneul Developer Cheat Sheet
---

Quick reference on best practices for Haneul Network developers.

# Move

### General

- Read about [package upgrades](https://docs.haneul.io/build/package-upgrades) and write upgrade-friendly code:
    - Packages are immutable, so buggy package code can be called forever. Add protections at the object level instead.
    - If you upgrade a package `P` to `P'`, other packages and clients that depend on `P` will continue using `P`, not auto-update to `P'`. Both dependent packages and client code must be explicitly updated to point at `P'`.
    - `public` functions cannot be deleted or changed, but `public(friend)` functions can. Use `public(friend)` or private visibility liberally unless you are exposing library functions that will live forever.
    - It is not possible to delete`struct` types, add new fields (though you can add [dynamic fields](https://docs.haneul.io/devnet/build/programming-with-objects/ch5-dynamic-fields)), or add new [abilities](https://move-language.github.io/move/abilities.html) via an upgrade. Introduce new types liberally—they live forever.
- Use `vector`-backed collections (`vector`, `VecSet`, `VecMap`, `PriorityQueue`) with a **known** maximum size of ≤ 1000 items.
    - Use dynamic field-backed collections (`Table`, `Bag`, `ObjectBag`, `ObjectTable`, `LinkedTable`) for any collection that allows third-party addition, larger collections, and collections of unknown size.
    - Haneul Move objects have a maximum size of 250KB—any attempt to create a larger object will lead to an aborted transaction. Ensure that your objects do not have an ever-growing `vector`-backed collection.
- If your function `f` needs a payment in (e.g.) HANEUL from the caller, use `fun f(payment: Coin<HANEUL>)` not `fun f(payment: &mut Coin<HANEUL>, amount: u64)`. This is safer for callers—they know exactly how much they are paying, and do not need to trust `f` to extract the right amount.
- Don’t micro-optimize gas usage. Haneul computation costs are rounded up to the closest *[bucket](https://docs.haneul.io/devnet/learn/tokenomics/gas-in-haneul#gas-units)*, so only very drastic changes will make a difference. In particular, if your transaction is already in the lowest cost bucket, it can’t get any cheaper.
- Follow the [Move coding conventions](https://move-language.github.io/move/coding-conventions.html) for consistent style.

### Composability

- Use the [`display`](https://docs.haneul.io/build/haneul-object-display) standard to customize how your objects show up in wallets, apps, and explorers
- Avoid “self-transfers”—whenever possible, instead of writing `transfer::transfer(obj, tx_context::sender(ctx))`, return `obj` from the current function. This allows a caller or [programmable transaction block](https://docs.haneul.io/build/prog-trans-ts-sdk) to use `obj`.

### Testing

- Use [`haneul::test_scenario`](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/packages/haneul-framework/sources/test/test_scenario.move)` to mimic multi-transaction, multi-sender test scenarios
- Use the [`haneul::test_utils`](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/packages/haneul-framework/sources/test/test_utils.move#L5)` module for better test error messages via `assert_eq`, debug printing via `print`, and test-only destruction via `destroy`.
- Use `haneul move test --coverage` to compute code coverage information for your tests, and `haneul move coverage source --module <name>` to see uncovered lines highlighted in red. Push coverage all the way to 100% if feasible.

# Apps

- For optimal performance and data consistency, apps should submit writes and reads for the same full node. In the TS SDK, this means that apps should use the wallet's [`signTransactionBlock`](https://haneul-wallet-kit.vercel.app/) API, then submit the transaction via a call to [`execute_transactionBlock`](https://docs.haneul.io/haneul-jsonrpc#haneul_executeTransactionBlock) on the app's full node, *not* use the wallet's `signAndExecuteTransactionBlock` API. This ensures read-after-write-consistency--reads from the app's full node will reflect writes from the transaction right away instead of waiting for a checkpoint.
- For lower latency, use [`executeTransactionBlock`](https://docs.haneul.io/haneul-jsonrpc#haneul_executeTransactionBlock) with `"showEffects": false` and `"showEvents": false` if your app needs to know that a transaction was confirmed, but does not immediately need to see the transaction effects or read the objects/events written by the transaction.
- Apps should implement a local cache for frequently read data rather than over-fetching from the full node.
- Whenever possible, use [programmable transaction blocks](https://docs.haneul.io/build/prog-trans-ts-sdk) to compose existing on-chain functionality rather than publishing new smart contract code. Programmable transaction blocks allow large-scale batching and heterogenous composition, driving already-low gas fees down even further.
- Apps should leave gas budget, gas price, and coin selection to the wallet. This gives wallets more flexibility, and it’s the wallet’s responsibility to dry run a transaction to ensure it doesn't fail.

# Signing

- **Never** sign two concurrent transactions that are touching the same owned object. Either use independent owned objects, or wait for one transaction to conclude before sending the next one. Violating this rule might lead to client [equivocation](https://docs.haneul.io/learn/haneul-glossary#equivocation), which locks up the owned objects involved in the two transactions until the end of the current epoch.
- Any `haneul client` command that crafts a transaction (e.g., `haneul client publish`, `haneul client call`) can accept the `--serialize-output` flag to output a base64 transaction to be signed.
    - Haneul supports several [signature schemes](https://docs.haneul.io/devnet/learn/cryptography/haneul-offline-signing) for transaction signing, including native [multisig](https://docs.haneul.io/devnet/learn/cryptography/haneul-multisig).
