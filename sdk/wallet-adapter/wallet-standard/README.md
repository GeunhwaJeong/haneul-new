# `@haneullabs/wallet-standard`

A suite of standard utilities for implementing wallets and libraries based on the
[Wallet Standard](https://github.com/wallet-standard/wallet-standard/).

## Implementing the Wallet Standard in an extension wallet

### Creating a wallet interface

You need to create a class that represents your wallet. You can use the `Wallet` interface from
`@haneullabs/wallet-standard` to help ensure your class adheres to the standard.

```typescript
import { Wallet, HANEUL_DEVNET_CHAIN } from '@haneullabs/wallet-standard';

class YourWallet implements Wallet {
	get version() {
		// Return the version of the Wallet Standard this implements (in this case, 1.0.0).
		return '1.0.0';
	}
	get name() {
		return 'Wallet Name';
	}
	get icon() {
		return 'some-icon-data-url';
	}

	// Return the Haneul chains that your wallet supports.
	get chains() {
		return [HANEUL_DEVNET_CHAIN];
	}
}
```

### Implementing features

Features are standard methods consumers can use to interact with a wallet. To be listed in the Haneul
wallet adapter, you must implement the following features in your wallet:

- `standard:connect` - Used to initiate a connection to the wallet.
- `standard:events` - Used to listen for changes that happen within the wallet, such as accounts
  being added or removed.
- `haneul:signTransactionBlock` - Used to prompt the user to sign a transaction block, and return the
  serializated transaction block and signature back to the user. This method does not submit the
  transaction block for execution.
- `haneul:signAndExecuteTransactionBlock` - Used to prompt the user to sign a transaction block, then
  submit it for execution to the blockchain.

You can implement these features in your wallet class under the `features` property:

```typescript
import {
  StandardConnectFeature,
  StandardConnectMethod,
  StandardEventsFeature,
  StandardEventsOnMethod,
  HaneulFeatures,
  HaneulSignTransactionBlockMethod,
  HaneulSignAndExecuteTransactionBlockMethod
} from "@haneullabs/wallet-standard";

class YourWallet implements Wallet {
  get features(): StandardConnectFeature & StandardEventsFeature & HaneulFeatures {
    return {
      "standard:connect": {
        version: "1.0.0",
        connect: this.#connect,
      },
      "standard:events": {
        version: "1.0.0",
        on: this.#on,
      },
      "haneul:signTransactionBlock": {
        version: "1.0.0",
        signTransactionBlock: this.#signTransactionBlock,
      },
      "haneul:signAndExecuteTransactionBlock": {
        version: "1.1.0",
        signAndExecuteTransactionBlock: this.#signAndExecuteTransactionBlock,
      },
      'haneul:signMessage': {
        version: '1.0.0',
        signMessage: this.#signMessage,
      },
    };
  },

  #on: StandardEventsOnMethod = () => {
    // Your wallet's on implementation.
  };

  #connect: StandardConnectMethod = () => {
    // Your wallet's implementation
  };

  #signTransactionBlock: HaneulSignTransactionBlockMethod = () => {
    // Your wallet's implementation
  };

  #signAndExecuteTransactionBlock: HaneulSignAndExecuteTransactionBlockMethod = () => {
    // Your wallet's implementation
  };

  #signMessage: HaneulSignMessageMethod = () => {
    // Your wallet's implementation
  };
}
```

### Exposing accounts

The last requirement of the wallet interface is to expose an `acccounts` interface. This should
expose all of the accounts that a connected dapp has access to. It can be empty prior to initiating
a connection through the `standard:connect` feature.

The accounts can use the `ReadonlyWalletAccount` class to easily construct an account matching the
required interface.

```typescript
import { ReadonlyWalletAccount } from '@haneullabs/wallet-standard';

class YourWallet implements Wallet {
	get accounts() {
		// Assuming we already have some internal representation of accounts:
		return someWalletAccounts.map(
			(walletAccount) =>
				// Return
				new ReadonlyWalletAccount({
					address: walletAccount.haneulAddress,
					publicKey: walletAccount.pubkey,
					// The Haneul chains that your wallet supports.
					chains: [HANEUL_DEVNET_CHAIN],
					// The features that this account supports. This can be a subset of the wallet's supported features.
					// These features must exist on the wallet as well.
					features: ['haneul:signAndExecuteTransactionBlock'],
				}),
		);
	}
}
```

### Registering in the window

Once you have a compatible interface for your wallet, you can register it using the `registerWallet`
function.

```typescript
import { registerWallet } from '@haneullabs/wallet-standard';

registerWallet(new YourWallet());
```

> If you're interested in the internal implementation of the `registerWallet` method, you can
> [see how it works here](https://github.com/wallet-standard/wallet-standard/blob/b4794e761de688906827829d5380b24cb8ed5fd5/packages/core/wallet/src/register.ts#L9).
