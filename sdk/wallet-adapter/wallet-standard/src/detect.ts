// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Wallet } from '@wallet-standard/core';
import { WalletWithHaneulFeatures } from './features';

// These features are absolutely required for wallets to function in the Haneul ecosystem.
// Eventually, as wallets have more consistent support of features, we may want to extend this list.
const REQUIRED_FEATURES: (keyof WalletWithHaneulFeatures['features'])[] = [
	'standard:connect',
	'standard:events',
];

export function isWalletWithHaneulFeatures(
	wallet: Wallet,
	/** Extra features that are required to be present, in addition to the expected feature set. */
	features: string[] = [],
): wallet is WalletWithHaneulFeatures {
	return [...REQUIRED_FEATURES, ...features].every((feature) => feature in wallet.features);
}
