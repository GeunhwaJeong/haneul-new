# Kiosk SDK

> **This package is still in active development. Use at your own risk**.
> Currently, the only supported environment is **Haneul Testnet**.


This Kiosk SDK library provides different utilities to interact/create/manage a [Kiosk](https://github.com/GeunhwaJeong/haneul/tree/main/kiosk).

## Installation

To install, add `@haneullabs/kiosk` package to your project

```
npm i @haneullabs/kiosk
```

You can also use your preferred package manager, such as yarn or pnpm.

## Examples

Here are some indicative examples on how to use the kiosk SDK.

<details>
<summary>Getting the listings & items by the kiosk's id</summary>

```typescript
import { fetchKiosk } from '@haneullabs/kiosk';
import { Connection, JsonRpcProvider } from '@haneullabs/haneul.js';

const provider = new JsonRpcProvider(
  new Connection({ fullnode: 'https://fullnode.testnet.haneul.io:443' }),
);

const getKiosk = async () => {
  const kioskAddress = `0xSomeKioskAddress`;

  const {
    data: res,
    nextCursor,
    hasNextPage,
  } = await fetchKiosk(provider, kioskAddress, { limit: 100 }); // could also add `cursor` for pagination

  console.log(res); // { listings: [], items: [],  itemIds: [],  listingIds: [] }
  console.log(nextCursor); // null
  console.log(hasNextPage); // false
};
```

</details>

<details>
<summary>Create a kiosk, share it and get transfer the `kioskOwnerCap` to the wallet's address</summary>

```typescript
import { createKioskAndShare } from '@haneullabs/kiosk';
import { TransactionBlock } from '@haneullabs/haneul.js';

const createKiosk = async () => {
  const accountAddress = '0xSomeHaneulAddress';

  const tx = new TransactionBlock();
  const kiosk_cap = createKioskAndShare(tx);

  tx.transferObjects([kiosk_cap], tx.pure(accountAddress, 'address'));

  // ... continue to sign and execute the transaction
  // ...
};
```

</details>

<details>
<summary>Place an item and list it for sale in the kiosk</summary>

```typescript
import { placeAndList } from '@haneullabs/kiosk';
import { TransactionBlock } from '@haneullabs/haneul.js';

const placeAndListToKiosk = async () => {
  const kiosk = 'SomeKioskId';
  const kioskCap = 'KioskCapObjectId';
  const itemType = '0xItemAddr::some:ItemType';
  const item = 'SomeItemId';
  const price = '100000';

  const tx = new TransactionBlock();

  placeAndList(tx, itemType, kiosk, kioskCap, item, price);

  // ... continue to sign and execute the transaction
  // ...
};
```

</details>

<details>
<summary>Withdraw profits from your kiosk</summary>

```typescript
import { withdrawFromKiosk } from '@haneullabs/kiosk';
import { TransactionBlock } from '@haneullabs/haneul.js';

const withdraw = async () => {
  const kiosk = 'SomeKioskId';
  const kioskCap = 'KioskCapObjectId';
  const amount = '100000';

  const tx = new TransactionBlock();

  withdrawFromKiosk(tx, kiosk, kioskCap, amount);

  // ... continue to sign and execute the transaction
  // ...
};
```

</details>
