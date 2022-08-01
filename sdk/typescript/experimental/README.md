# Haneul Experimental (dapp) SDK

This package provides a lightweight, browser compatible SDK, originally built for dapp development.

## Feature set

- Local transaction building and signing thanks to BCS
- Supported in both NodeJS and browser environments
- Minimal set of dependencies: *tweetnacl*, *@haneullabs/bcs*, *js-sha3* and *bn.js*
- Object tracking - every state change or query that goes through `HaneulClient` updates the object references storage

## Usage

```ts
import { HaneulClient } from 'experimental';

// also possible: HaneulClient.devnet();
// or:            HaneulClient.local();
// or:        new HaneulClient(gatewayUrl, fullNodeUrl);
const haneul = HaneulClient.devnet();

// ... example to be added here ...
```
