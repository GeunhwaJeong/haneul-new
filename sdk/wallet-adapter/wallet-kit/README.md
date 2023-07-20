# Haneul Wallet Kit

> **⚠️ These packages are experimental and will change rapidly as they are being developed. Do not consider these APIs to be stable. If you have any feedback, [open an issue](https://github.com/GeunhwaJeong/haneul/issues/new/choose) or message us on [Discord](https://discord.gg/Haneul).**

Haneul Wallet Kit is a library that makes it easy to connect your dApp to Haneul wallets. It wraps the underlying Haneul Wallet Adapters and comes pre-configured with sane defaults.

## Getting started

To get started in a React application, you can install the following packages:

```bash
npm install @haneullabs/wallet-kit
```

At the root of your application, you can then set up the wallet kit provider:

```tsx
import { WalletKitProvider } from '@haneullabs/wallet-kit';

export function App() {
	return <WalletKitProvider>{/* Your application... */}</WalletKitProvider>;
}
```

> The `WalletKitProvider` also supports an `adapters` prop, which lets you override the default Haneul Wallet Adapters.

You can then add a **Connect Wallet** button to your page:

```tsx
import { ConnectButton, useWalletKit } from '@haneullabs/wallet-kit';
import { formatAddress } from '@haneullabs/haneul.js';

function ConnectToWallet() {
	const { currentAccount } = useWalletKit();
	return (
		<ConnectButton
			connectText={'Connect Wallet'}
			connectedText={`Connected: ${formatAddress(currentAccount.address)}`}
		/>
	);
}
```

To get access to the currently connected wallet, use the `useWalletKit()` hook to interact with the wallet, such as proposing transactions:

```tsx
import { TransactionBlock } from '@haneullabs/haneul.js/transactions';
import { useWalletKit } from '@haneullabs/wallet-kit';

export function SendTransaction() {
	const { signAndExecuteTransactionBlock } = useWalletKit();

	const handleClick = async () => {
		const tx = new TransactionBlock();
		tx.moveCall({
			target: '0x2::devnet_nft::mint',
			arguments: [
				tx.pure('some name'),
				tx.pure('some description'),
				tx.pure(
					'https://cdn.britannica.com/94/194294-138-B2CF7780/overview-capybara.jpg?w=800&h=450&c=crop',
				),
			],
		});
		await signAndExecuteTransactionBlock({ transactionBlock: tx });
	};

	return (
		<Button onClick={handleClick} disabled={!connected}>
			Send Transaction
		</Button>
	);
}
```

### Usage without React

We do not currently have non-React UI libraries for connecting to wallets. The wallet adapters and logic in the React library (`@haneullabs/wallet-adapter-react`) can be used as reference for implementing a wallet connection UI in other UI libraries.

## Supported wallets

Wallet Kit comes pre-configured with every supported wallet. You can also install individual wallet adapters that you plan on using in your project.

### Wallet Standard

The `WalletStandardAdapterProvider` adapter (published under `@haneullabs/wallet-adapter-wallet-standard`) automatically supports wallets that adhere to the cross-chain [Wallet Standard](https://github.com/wallet-standard/wallet-standard/). This adapter detects the available wallets in users' browsers. You do not need to configure additional adapters.
