// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { Wallet, WalletWithHaneulFeatures } from '@haneullabs/wallet-standard';
import { isWalletWithHaneulFeatures } from '@haneullabs/wallet-standard';

export function sortWallets(
	wallets: readonly Wallet[],
	preferredWallets: string[],
	requiredFeatures?: string[],
): WalletWithHaneulFeatures[] {
	const haneulWallets = wallets.filter((wallet): wallet is WalletWithHaneulFeatures =>
		isWalletWithHaneulFeatures(wallet, requiredFeatures),
	);

	return [
		// Preferred wallets, in order:
		...(preferredWallets
			.map((name) => haneulWallets.find((wallet) => wallet.name === name))
			.filter(Boolean) as WalletWithHaneulFeatures[]),

		// Wallets in default order:
		...haneulWallets.filter((wallet) => !preferredWallets.includes(wallet.name)),
	];
}
