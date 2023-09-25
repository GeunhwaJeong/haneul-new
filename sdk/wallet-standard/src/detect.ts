// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Wallet, WalletWithFeatures } from '@wallet-standard/core';

import { MinimallyRequiredFeatures, WalletWithHaneulFeatures } from './features';

// These features are absolutely required for wallets to function in the Haneul ecosystem.
// Eventually, as wallets have more consistent support of features, we may want to extend this list.
const REQUIRED_FEATURES: (keyof MinimallyRequiredFeatures)[] = [
	'standard:connect',
	'standard:events',
];

/** @deprecated Use isWalletWithRequiredFeatureSet instead since it provides more accurate typing! */
export function isWalletWithHaneulFeatures(
	wallet: Wallet,
	/** Extra features that are required to be present, in addition to the expected feature set. */
	features: string[] = [],
): wallet is WalletWithHaneulFeatures {
	return [...REQUIRED_FEATURES, ...features].every((feature) => feature in wallet.features);
}

export function isWalletWithRequiredFeatureSet<AdditionalFeatures extends Wallet['features']>(
	wallet: Wallet,
	additionalFeatures: (keyof AdditionalFeatures)[] = [],
): wallet is WalletWithFeatures<MinimallyRequiredFeatures & AdditionalFeatures> {
	return [...REQUIRED_FEATURES, ...additionalFeatures].every(
		(feature) => feature in wallet.features,
	);
}
