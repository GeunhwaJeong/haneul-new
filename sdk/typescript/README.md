# Haneul TypeScript SDK

This is the Haneul TypeScript SDK built on the Haneul [JSON RPC API](https://github.com/GeunhwaJeong/haneul/blob/main/doc/src/build/json-rpc.md). It provides utility classes and functions for applications to sign transactions and interact with the Haneul network.

WARNING: Note that we are still iterating on the RPC and SDK API before TestNet, therefore please expect frequent breaking changes in the short-term. We expect the API to stabilize after the upcoming TestNet launch.

## Working with DevNet

The SDK will be published to [npm registry](https://www.npmjs.com/package/@haneullabs/haneul.js) with the same bi-weekly release cycle as the DevNet validators and [RPC Server](https://github.com/GeunhwaJeong/haneul/blob/main/doc/src/build/json-rpc.md). To use the SDK in your project, you can do:

```bash
$ npm install @haneullabs/haneul.js
```

You can also use your preferred npm client, such as yarn or pnpm.

## Working with local network

Note that the `latest` tag for the [published SDK](https://www.npmjs.com/package/@haneullabs/haneul.js) might go out of sync with the RPC server on the `main` branch until the next release. If you're developing against a local network, we recommend using the `experimental`-tagged packages, which contain the latest changes from `main`.

```bash
npm install @haneullabs/haneul.js@experimental
```

Refer to the [JSON RPC](https://github.com/GeunhwaJeong/haneul/blob/main/doc/src/build/json-rpc.md) topic for instructions about how to start a local network and local RPC server.

## Building Locally

To get started you need to install [pnpm](https://pnpm.io/), then run the following command:

```bash
# Install all dependencies
$ pnpm install
# Run the build for the TypeScript SDK
$ pnpm sdk build
```

> All `pnpm` commands are intended to be run in the root of the Haneul repo. You can also run them within the `sdk/typescript` directory, and remove change `pnpm sdk` to just `pnpm` when running commands.

## Type Doc

You can view the generated [Type Doc](https://typedoc.org/) for the [current release of the SDK](https://www.npmjs.com/package/@haneullabs/haneul.js) at http://typescript-sdk-docs.s3-website-us-east-1.amazonaws.com/.

For the latest docs for the `main` branch, run `pnpm doc` and open the [doc/index.html](doc/index.html) in your browser.

## Testing

To run unit tests

```
pnpm sdk test:unit
```

To run E2E tests against local network

```
pnpm sdk prepare:e2e

// This will run all e2e tests
pnpm sdk test:e2e

// Alternatively you can choose to run only one test file
npx vitest txn-builder.test.ts
```

Troubleshooting:

If you see errors like `ECONNRESET or "socket hang up"`, run `node -v` to make sure your node version is `v18.x.x`. Refer to this [guide](https://blog.logrocket.com/how-switch-node-js-versions-nvm/) to switch node version.

Some more follow up here is if you used homebrew to install node, there could be multiple paths to node on your machine. https://stackoverflow.com/questions/52676244/node-version-not-updating-after-nvm-use-on-mac

To run E2E tests against DevNet

```
VITE_FAUCET_URL='https://faucet.devnet.haneul.io:443/gas' VITE_FULLNODE_URL='https://fullnode.devnet.haneul.io' pnpm sdk exec vitest e2e
```

## Connecting to Haneul Network

The `JsonRpcProvider` class provides a connection to the JSON-RPC Server and should be used for all read-only operations. The default URLs to connect with the RPC server are:

- local: http://127.0.0.1:9000
- DevNet: https://fullnode.devnet.haneul.io

```typescript
import { JsonRpcProvider, devnetConnection } from '@haneullabs/haneul.js';
// connect to Devnet
const provider = new JsonRpcProvider(devnetConnection);
// get tokens from the DevNet faucet server
await provider.requestHaneulFromFaucet(
  '0xbff6ccc8707aa517b4f1b95750a2a8c666012df3',
);
```

For local development, you can run `cargo run --bin haneul-test-validator` to spin up a local network with a local validator, a fullnode, and a faucet server. Refer to [this guide](https://docs.haneul.io/build/haneul-local-network) for more information.

```typescript
import { JsonRpcProvider, localnetConnection } from '@haneullabs/haneul.js';
// connect to local RPC server
const provider = new JsonRpcProvider(localnetConnection);
// get tokens from the local faucet server
await provider.requestHaneulFromFaucet(
  '0xbff6ccc8707aa517b4f1b95750a2a8c666012df3',
);
```

You can also construct your own in custom connections, with your own URLs to your fullnode and faucet server

```typescript
import { JsonRpcProvider, Connection } from '@haneullabs/haneul.js';
// Construct your connection:
const connection = new Connection({
  fullnode: 'https://fullnode.devnet.haneul.io',
  faucet: 'https://faucet.devnet.haneul.io/gas',
});
// connect to a custom RPC server
const provider = new JsonRpcProvider(connection);
// get tokens from a custom faucet server
await provider.requestHaneulFromFaucet(
  '0xbff6ccc8707aa517b4f1b95750a2a8c666012df3',
);
```

## Writing APIs

For a primer for building transactions, refer to [this guide](https://docs.haneul.io/build/prog-trans-ts-sdk).

### Transfer Object

```typescript
import {
  Ed25519Keypair,
  JsonRpcProvider,
  RawSigner,
  TransactionBlock,
} from '@haneullabs/haneul.js';
// Generate a new Ed25519 Keypair
const keypair = new Ed25519Keypair();
const provider = new JsonRpcProvider();
const signer = new RawSigner(keypair, provider);
const tx = new TransactionBlock();
tx.transferObjects(
  [tx.object('0x5015b016ab570df14c87649eda918e09e5cc61e0')],
  tx.pure('0xd84058cb73bdeabe123b56632713dcd65e1a6c92'),
);
const result = await signer.signAndExecuteTransaction({ transactionBlock: tx });
console.log({ result });
```

### Transfer Haneul

To transfer `1000` GEUNHWA to another address:

```typescript
import {
  Ed25519Keypair,
  JsonRpcProvider,
  RawSigner,
  TransactionBlock,
} from '@haneullabs/haneul.js';
// Generate a new Keypair
const keypair = new Ed25519Keypair();
const provider = new JsonRpcProvider();
const signer = new RawSigner(keypair, provider);
const tx = new TransactionBlock();
const [coin] = tx.splitCoins(tx.gas, tx.pure(1000));
tx.transferObjects([coin], tx.pure(keypair.getPublicKey().toHaneulAddress()));
const result = await signer.signAndExecuteTransaction({ transactionBlock: tx });
console.log({ result });
```

### Merge coins

```typescript
import {
  Ed25519Keypair,
  JsonRpcProvider,
  RawSigner,
  TransactionBlock,
} from '@haneullabs/haneul.js';
// Generate a new Keypair
const keypair = new Ed25519Keypair();
const provider = new JsonRpcProvider();
const signer = new RawSigner(keypair, provider);
const tx = new TransactionBlock();
tx.mergeCoin(tx.object('0x5015b016ab570df14c87649eda918e09e5cc61e0'), [
  tx.object('0xcc460051569bfb888dedaf5182e76f473ee351af'),
]);
const result = await signer.signAndExecuteTransaction({ transactionBlock: tx });
console.log({ result });
```

### Move Call

```typescript
import {
  Ed25519Keypair,
  JsonRpcProvider,
  RawSigner,
  TransactionBlock,
} from '@haneullabs/haneul.js';
// Generate a new Keypair
const keypair = new Ed25519Keypair();
const provider = new JsonRpcProvider();
const signer = new RawSigner(keypair, provider);
const packageObjectId = '0x...';
const tx = new TransactionBlock();
tx.moveCall({
  target: `${packageObjectId}::nft::mint`,
  arguments: [tx.pure('Example NFT')],
});
const result = await signer.signAndExecuteTransaction({ transactionBlock: tx });
console.log({ result });
```

### Publish Modules

To publish a package:

```typescript
import {
  Ed25519Keypair,
  JsonRpcProvider,
  RawSigner,
  TransactionBlock,
  normalizeHaneulObjectId,
} from '@haneullabs/haneul.js';
const { execSync } = require('child_process');
// Generate a new Keypair
const keypair = new Ed25519Keypair();
const provider = new JsonRpcProvider();
const signer = new RawSigner(keypair, provider);
const compiledModulesAndDependencies = JSON.parse(
  execSync(
    `${cliPath} move build --dump-bytecode-as-base64 --path ${packagePath}`,
    { encoding: 'utf-8' },
  ),
);
const tx = new TransactionBlock();
tx.publish(
  compiledModulesAndDeps.modules.map((m: any) => Array.from(fromB64(m))),
  compiledModulesAndDeps.dependencies.map((addr: string) =>
    normalizeHaneulObjectId(addr),
  ),
);
const result = await signer.signAndExecuteTransaction({ transactionBlock: tx });
console.log({ result });
```

## Reading APIs

### Get Owned Objects

Fetch objects owned by the address `0xbff6ccc8707aa517b4f1b95750a2a8c666012df3`

```typescript
import { JsonRpcProvider } from '@haneullabs/haneul.js';
const provider = new JsonRpcProvider();
const objects = await provider.getOwnedObjects({
  owner: '0xbff6ccc8707aa517b4f1b95750a2a8c666012df3',
});
```

### Get Object

Fetch object details for the object with id `0xcff6ccc8707aa517b4f1b95750a2a8c666012df3`

```typescript
import { JsonRpcProvider } from '@haneullabs/haneul.js';
const provider = new JsonRpcProvider();
const txn = await provider.getObject({
  id: '0xcff6ccc8707aa517b4f1b95750a2a8c666012df3',
  // fetch the object content field
  options: { showContent: true },
});
// You can also fetch multiple objects in one batch request
const txns = await provider.multiGetObjects({
  ids: [
    '0xcff6ccc8707aa517b4f1b95750a2a8c666012df3',
    '0xdff6ccc8707aa517b4f1b95750a2a8c666012df3',
  ],
  // only fetch the object type
  options: { showType: true },
});
```

### Get Transaction

Fetch transaction details from transaction digests:

```typescript
import { JsonRpcProvider } from '@haneullabs/haneul.js';
const provider = new JsonRpcProvider();
const txn = await provider.getTransaction({
  digest: '6mn5W1CczLwitHCO9OIUbqirNrQ0cuKdyxaNe16SAME=',
  // only fetch the effects field
  options: { showEffects: true },
});
// You can also fetch multiple transactions in one batch request
const txns = await provider.multiGetTransactions({
  digests: [
    '6mn5W1CczLwitHCO9OIUbqirNrQ0cuKdyxaNe16SAME=',
    '7mn5W1CczLwitHCO9OIUbqirNrQ0cuKdyxaNe16SAME=',
  ],
  // fetch both the input transaction data as well as effects
  options: { showInput: true, showEffects: true },
});
```

### Get Coins

Fetch coins of type `0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC` owned by an address:

```typescript
import { JsonRpcProvider } from '@haneullabs/haneul.js';
const provider = new JsonRpcProvider();
// If coin type is not specified, it defaults to 0x2::haneul::HANEUL
const coins = await provider.getCoins({
  owner: '0xbff6ccc8707aa517b4f1b95750a2a8c666012df3',
  coinType: '0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC',
});
```

Fetch all coin objects owned by an address:

```typescript
import { JsonRpcProvider } from '@haneullabs/haneul.js';
const provider = new JsonRpcProvider();
const allCoins = await provider.getAllCoins({
  owner: '0xbff6ccc8707aa517b4f1b95750a2a8c666012df3',
});
```

Fetch the total coin balance for one coin type, owned by an address:

```typescript
import { JsonRpcProvider } from '@haneullabs/haneul.js';
const provider = new JsonRpcProvider();
// If coin type is not specified, it defaults to 0x2::haneul::HANEUL
const coinBalance = await provider.getBalance({
  owner: '0xbff6ccc8707aa517b4f1b95750a2a8c666012df3',
  coinType: '0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC',
});
```

### Events API

Querying events created by transactions sent by account
`0xbff6ccc8707aa517b4f1b95750a2a8c666012df3`

```typescript
import { JsonRpcProvider } from '@haneullabs/haneul.js';
const provider = new JsonRpcProvider();
const events = provider.queryEvents({
  query: { Sender: toolbox.address() },
  limit: 2,
});
```

Subscribe to all events created by transactions sent by account `0xbff6ccc8707aa517b4f1b95750a2a8c666012df3`

```typescript
import { JsonRpcProvider, HaneulEvent } from '@haneullabs/haneul.js';
const provider = new JsonRpcProvider();

// calls RPC method 'haneul_subscribeEvent' with params:
// [ { Sender: '0xbff6ccc8707aa517b4f1b95750a2a8c666012df3' } ]
const subscriptionId = await provider.subscribeEvent({
  filter: { Sender: '0xbff6ccc8707aa517b4f1b95750a2a8c666012df3' },
  onMessage(event: HaneulEvent) {
    // handle subscription notification message here. This function is called once per subscription message.
  },
});

// later, to unsubscribe
// calls RPC method 'haneul_unsubscribeEvent' with params: [ subscriptionId ]
const subFoundAndRemoved = await provider.unsubscribeEvent({
  id: subscriptionId,
});
```

Subscribe to all events created by a package's `nft` module

```typescript
import { JsonRpcProvider, HaneulEvent } from '@haneullabs/haneul.js';
const provider = new JsonRpcProvider();

const package = '0x...';
const devnetNftFilter = {
    { MoveModule: { package, module: 'nft'} },
};
const devNftSub = await provider.subscribeEvent({
  filter: devnetNftFilter,
  onMessage(event: HaneulEvent) {
    // handle subscription notification message here
  },
});
```
