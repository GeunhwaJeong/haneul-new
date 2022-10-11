# `@haneullabs/wallet-standard`

A suite of standard utilities for implementing wallets and libraries based on the [Wallet Standard](https://github.com/wallet-standard/wallet-standard/).

## Implementing the Wallet Standard in an extension wallet

### Creating a wallet interface

You need to create a class that represents your wallet. You can use the `Wallet` interface from `@haneullabs/wallet-standard` to help ensure your class adheres to the standard.

```typescript
import { Wallet, HANEUL_DEVNET_CHAIN } from "@haneullabs/wallet-standard";

class YourWallet implements Wallet {
  get version() {
    // Return the version of the Wallet Standard this implements (in this case, 1.0.0).
    return "1.0.0";
  }
  get name() {
    return "Wallet Name";
  }
  get icon() {
    return "some-icon-data-url";
  }

  // Return the Haneul chains that your wallet supports.
  get chains() {
    return [HANEUL_DEVNET_CHAIN];
  }
}
```

### Implementing features

Features are standard methods consumers can use to interact with a wallet. To be listed in the Haneul wallet adapter, you must implement the following features in your wallet:

- `standard:connect` - Used to initiate a connection to the wallet.
- `standard:events` - Used to listen for changes that happen within the wallet, such as accounts being added or removed.
- `haneul:signAndExecuteTransaction` - Used to prompt the user to sign a transaction, then submit it for execution to the blockchain.

You can implement these features in your wallet class under the `features` property:

```typescript
import {
  ConnectFeature,
  ConnectMethod,
  EventsFeature,
  EventsOnMethod,
  HaneulSignAndExecuteTransactionFeature,
  HaneulSignAndExecuteTransactionMethod
} from "@haneullabs/wallet-standard";

class YourWallet implements Wallet {
  get features(): ConnectFeature & EventsFeature & HaneulSignAndExecuteTransactionFeature {
    return {
      "standard:connect": {
        version: "1.0.0",
        connect: this.#connect,
      },
      "standard:events": {
        version: "1.0.0",
        on: this.#on,
      }
      "haneul:signAndExecuteTransaction": {
        version: "1.0.0",
        signAndExecuteTransaction: this.#signAndExecuteTransaction,
      },
    };
  },

  #on: EventsOnMethod = () => {
    // Your wallet's events on implementation.
  };

	#connect: ConnectMethod = () => {
		// Your wallet's connect implementation
	};

	#signAndExecuteTransaction: HaneulSignAndExecuteTransactionMethod = () => {
		// Your wallet's signAndExecuteTransaction implementation
	};
}
```

### Exposing accounts

The last requirement of the wallet interface is to expose an `acccounts` interface. This should expose all of the accounts that a connected dapp has access to. It can be empty prior to initiating a connection through the `standard:connect` feature.

The accounts can use the `ReadonlyWalletAccount` class to easily construct an account matching the required interface.

```typescript
import { ReadonlyWalletAccount } from "@haneullabs/wallet-standard";

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
          features: ["haneul:signAndExecuteTransaction", "standard:signMessage"],
        })
    );
  }
}
```

### Registering in the window

Once you have a compatible interface for your wallet, you can register it in the window under the `window.navigator.wallets` interface. Wallets self-register by pushing their standard wallet interface to this array-like interface.

```typescript
// This makes TypeScript aware of the `window.navigator.wallets` interface.
declare const window: import("@haneullabs/wallet-standard").WalletsWindow;

(window.navigator.wallets || []).push(({ register }) => {
  register(new YourWallet());
});
```

> Note that while this interface is array-like, it is not always an array, and the only method that should be called on it is `push`.
