// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
	MinimallyRequiredFeatures,
	Wallet,
	WalletWithFeatures,
} from '@haneullabs/wallet-standard';
import { isWalletWithRequiredFeatureSet } from '@haneullabs/wallet-standard';

export function sortWallets<AdditionalFeatures extends Wallet['features']>(
	wallets: readonly Wallet[],
	preferredWallets: string[],
	requiredFeatures?: (keyof AdditionalFeatures)[],
) {
	const haneulWallets = wallets.filter(
		(wallet): wallet is WalletWithFeatures<MinimallyRequiredFeatures & AdditionalFeatures> =>
			isWalletWithRequiredFeatureSet(wallet, requiredFeatures),
	);

	return [
		// Preferred wallets, in order:
		...(preferredWallets
			.map((name) => haneulWallets.find((wallet) => wallet.name === name))
			.filter(Boolean) as WalletWithFeatures<MinimallyRequiredFeatures & AdditionalFeatures>[]),

		// Wallets in default order:
		...haneulWallets.filter((wallet) => !preferredWallets.includes(wallet.name)),
	];
}
