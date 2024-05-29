# HaneulNS TypeScript SDK

This is a lightweight SDK (1kB minified bundle size), providing utility classes and functions for
applications to interact with on-chain `.haneul` names registered from
[Haneul Name Service (haneulns.io)](https://haneulns.io).

## Getting started

The SDK is published to [npm registry](https://www.npmjs.com/package/@haneullabs/haneulns-toolkit). To use
it in your project:

```bash
$ npm install @haneullabs/haneulns-toolkit
```

You can also use yarn or pnpm.

## Examples

Create an instance of HaneulnsClient:

```typescript
import { HaneulClient } from '@haneullabs/haneul/client';
import { HaneulnsClient } from '@haneullabs/haneulns-toolkit';

const client = new HaneulClient();
export const haneulnsClient = new HaneulnsClient(client);
```

Choose network type:

```typescript
export const haneulnsClient = new HaneulnsClient(client, {
	networkType: 'testnet',
});
```

> **Note:** To ensure best performance, please make sure to create only one instance of the
> HaneulnsClient class in your application. Then, import the created `haneulnsClient` instance to use its
> functions.

Fetch an address linked to a name:

```typescript
const address = await haneulnsClient.getAddress('haneulns.haneul');
```

Fetch the default name of an address:

```typescript
const defaultName = await haneulnsClient.getName(
	'0xc2f08b6490b87610629673e76bab7e821fe8589c7ea6e752ea5dac2a4d371b41',
);
```

Fetch a name object:

```typescript
const nameObject = await haneulnsClient.getNameObject('haneulns.haneul');
```

Fetch a name object including the owner:

```typescript
const nameObject = await haneulnsClient.getNameObject('haneulns.haneul', {
	showOwner: true,
});
```

Fetch a name object including the Avatar the owner has set (it automatically includes owner too):

```typescript
const nameObject = await haneulnsClient.getNameObject('haneulns.haneul', {
	showOwner: true, // this can be skipped as showAvatar includes it by default
	showAvatar: true,
});
```

## License

[Apache-2.0](https://github.com/HaneulNSdapp/toolkit/blob/main/LICENSE)
