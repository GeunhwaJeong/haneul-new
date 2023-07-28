---
'@haneullabs/haneul.js': minor
---

The Haneul TS SDK has been broken up into a set of modular exports, and all exports from the root of
the package have been deprecated. The following export paths have been added:

- `@haneullabs/haneul.js/client` - A client for interacting with Haneul RPC nodes.
- `@haneullabs/haneul.js/bcs` - A BCS builder with pre-defined types for Haneul.
- `@haneullabs/haneul.js/transaction` - Utilities for building and interacting with transactions.
- `@haneullabs/haneul.js/keypairs/*` - Modular exports for specific KeyPair implementations.
- `@haneullabs/haneul.js/verify` - Methods for verifying transactions and messages.
- `@haneullabs/haneul.js/cryptography` - Shared types and classes for cryptography.
- `@haneullabs/haneul.js/multisig` - Utilities for working with multisig signatures.
- `@haneullabs/haneul.js/utils` - Utilities for formatting and parsing various Haneul types.
- `@haneullabs/haneul.js/faucet`- Methods for requesting haneul from a faucet.

As part of this refactor we are deprecating a number of existing APIs:

- `JsonRPCProvider` - This Provider pattern is being replaced by a new `HaneulClient`
- `SignerWithProver` and `RawSigner` - The Concept of Signers is being removed from the SDK. Signing
  in verifying has been moved to the KeyPair classes, and the `signAndExecuteTransactionBlock`
  method has been moved to the new `HaneulClient`.
- The `superstruct` type definitions for types used by JsonRPCProvider are being replaced with
  generated types exported from `@haneullabs/haneul.js/client`. The new type definitions are pure
  typescript types and can't be used for runtime validation. By generating these as types, it will
  be easier to keep them in sync with the RPC definitions and avoid discrepancies between the type
  definitions in the SDK and the data returned by RPC methods.
- A large number of "getters" are being deprecated. These getters were intended to reduce friction
  caused by rapid iteration in the RPC layer leading up to the mainnet launch. Now that mainnet has
  been launched the RPC API should be more stable, and many of these helpers can be replaced by
  simply accessing the nested properties in the returned data directly.

The current release should be mostly backwards compatible, and all existing exports will continue to
be available in this release (with deprecation warnings). With the large number of deprecations
there may be functionality that should be moved into the new modular version of the SDK. If you find
there are features that were deprecated without a suitable replacement, we have created a
[Github Discussion thread](https://github.com/GeunhwaJeong/haneul/discussions/13150) to track those
use-cases.

#### Migrating imports

To migrate imports, you should be able to hover over the deprecated import in the editor of you
choice, this should provide either the deprecation message letting you know where to import the
replacement from, or a like "The declaration was marked as deprecated here." with a link to the
deprecation comment which will tell you how to update your import

#### Migrating JsonRpcProvider

The new HaneulClient should mostly work as a drop in replacement for the `JsonRpcProvider` provider.
Setting up a `HaneulClient` is slightly different, but once constructed should work just like a
provider.

```diff
- import { JsonRpcProvider, devnetConnection } from '@haneullabs/haneul.js';
+ import { HaneulClient, getFullnodeUrl } from '@haneullabs/haneul.js/client';

- const provider = new JsonRpcProvider(localnetConnection);
+ const client = new HaneulClient({ url: getFullnodeUrl('localnet')});
```

#### Signing TransactionBlocks

Signing and sending transaction blocks has change slightly with the deprecation of the `Signer`
pattern:

```diff
- import {
-    Ed25519Keypair,
-    JsonRpcProvider,
-    RawSigner,
-    TransactionBlock,
-    localnetConnection,
- } from '@haneullabs/haneul.js';
+ import { Ed25519Keypair } from '@haneullabs/haneul.js/keypairs/ed25519';
+ import { HaneulClient, getFullnodeUrl } from '@haneullabs/haneul.js/client';
+ import { TransactionBlock } from '@haneullabs/haneul.js/transactions';

  const keypair = new Ed25519Keypair()
- const provider = new JsonRpcProvider(localnetConnection);
- const signer = new RawSigner(keyPair, provider);
+ const client = new HaneulClient({ url: getFullnodeUrl('localnet')});

- const result = await signer.signAndExecuteTransactionBlock({
+ const result = await client.signAndExecuteTransactionBlock({
+   signer: keypair,
    transactionBlock: tx,
    options: { ... }
  })
```

#### Migrating faucet requests

The ability to request Haneul from a faucet was not added to `HaneulClient`, instead you will need to use
a method `@haneullabs/haneul.js/faucet` to make these requests

```diff
- import { JsonRpcProvider, devnetConnection } from '@haneullabs/haneul.js';
- const provider = new JsonRpcProvider(devnetConnection);
+ import { requestHaneulFromFaucetV0, getFaucetHost } from '@haneullabs/haneul.js/faucet';

- await provider.requestHaneulFromFaucet(
-  '<YOUR HANEUL ADDRESS>'
- );
+ await requestHaneulFromFaucetV0({
+   host: getFaucetHost('devnet'),
+   recipient: '<YOUR HANEUL ADDRESS>',
+});
```
