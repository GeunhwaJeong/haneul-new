// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulWallet } from '_src/dapp-interface/WalletStandardInterface';
import { TransactionBlock } from '@haneullabs/haneul.js/transactions';
import { getWallets, ReadonlyWalletAccount, type Wallet } from '@haneullabs/wallet-standard';
import { useEffect, useState } from 'react';
import ReactDOM from 'react-dom/client';

function getDemoTransaction(address: string) {
	const txb = new TransactionBlock();
	const [coin] = txb.splitCoins(txb.gas, [txb.pure(1)]);
	txb.transferObjects([coin], txb.pure(address));
	return txb;
}

function getAccount(account: ReadonlyWalletAccount, useWrongAccount: boolean) {
	if (useWrongAccount && account) {
		const newAccount = new ReadonlyWalletAccount({
			address: '0x00000001',
			chains: account.chains,
			features: account.features,
			publicKey: account.publicKey,
			icon: account.icon,
			label: account.label,
		});
		return newAccount;
	}
	return account;
}

function findHaneulWallet(wallets: readonly Wallet[]) {
	return (wallets.find((aWallet) => aWallet.name.includes('Haneul Wallet')) ||
		null) as HaneulWallet | null;
}

function App() {
	const [haneulWallet, setHaneulWallet] = useState<HaneulWallet | null>(() =>
		findHaneulWallet(getWallets().get()),
	);
	const [error, setError] = useState<string | null>(null);
	const [accounts, setAccounts] = useState<ReadonlyWalletAccount[]>(
		() => haneulWallet?.accounts || [],
	);
	const [useWrongAccounts, setUseWrongAccounts] = useState(false);

	useEffect(() => {
		const walletsApi = getWallets();
		function updateWallets() {
			setHaneulWallet(findHaneulWallet(walletsApi.get()));
		}
		const unregister1 = walletsApi.on('register', updateWallets);
		const unregister2 = walletsApi.on('unregister', updateWallets);
		return () => {
			unregister1();
			unregister2();
		};
	}, []);
	useEffect(() => {
		if (haneulWallet) {
			return haneulWallet.features['standard:events'].on('change', ({ accounts }) => {
				if (accounts) {
					setAccounts(haneulWallet.accounts);
				}
			});
		}
	}, [haneulWallet]);
	if (!haneulWallet) {
		return <h1>Haneul Wallet not found</h1>;
	}
	return (
		<>
			<h1>Haneul Wallet is installed. ({haneulWallet.name})</h1>
			{accounts.length ? (
				<ul data-testid="accounts-list">
					{accounts.map((anAccount) => (
						<li key={anAccount.address}>{anAccount.address}</li>
					))}
				</ul>
			) : (
				<button onClick={async () => haneulWallet.features['standard:connect'].connect()}>
					Connect
				</button>
			)}
			<label>
				<input
					type="checkbox"
					checked={useWrongAccounts}
					onChange={() => setUseWrongAccounts((v) => !v)}
				/>
				Use wrong account
			</label>
			<button
				onClick={async () => {
					setError(null);
					const txb = getDemoTransaction(accounts[0]?.address);
					try {
						await haneulWallet.features[
							'haneul:signAndExecuteTransactionBlock'
						].signAndExecuteTransactionBlock({
							transactionBlock: txb,
							account: getAccount(accounts[0], useWrongAccounts),
							chain: 'haneul:unknown',
						});
					} catch (e) {
						setError((e as Error).message);
					}
				}}
			>
				Send transaction
			</button>
			<button
				onClick={async () => {
					setError(null);
					const txb = getDemoTransaction(accounts[0]?.address);
					try {
						await haneulWallet.features['haneul:signTransactionBlock'].signTransactionBlock({
							transactionBlock: txb,
							account: getAccount(accounts[0], useWrongAccounts),
							chain: 'haneul:unknown',
						});
					} catch (e) {
						setError((e as Error).message);
					}
				}}
			>
				Sign transaction
			</button>
			<button
				onClick={async () => {
					setError(null);
					try {
						await haneulWallet.features['haneul:signMessage']?.signMessage({
							account: getAccount(accounts[0], useWrongAccounts),
							message: new TextEncoder().encode('Test message'),
						});
					} catch (e) {
						setError((e as Error).message);
					}
				}}
			>
				Sign message
			</button>
			{error ? (
				<div>
					<h6>Error</h6>
					<div>{error}</div>
				</div>
			) : null}
		</>
	);
}

ReactDOM.createRoot(document.getElementById('root')!).render(<App />);
