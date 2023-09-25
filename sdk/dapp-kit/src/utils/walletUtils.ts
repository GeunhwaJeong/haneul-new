// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
	MinimallyRequiredFeatures,
	Wallet,
	WalletWithFeatures,
} from '@haneullabs/wallet-standard';
import { getWallets, isWalletWithRequiredFeatureSet } from '@haneullabs/wallet-standard';

export function getRegisteredWallets<AdditionalFeatures extends Wallet['features']>(
	preferredWallets: string[],
	requiredFeatures?: (keyof AdditionalFeatures)[],
) {
	const walletsApi = getWallets();
	const wallets = walletsApi.get();

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
