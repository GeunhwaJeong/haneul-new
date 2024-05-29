// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulWalletFeatures, WalletWithRequiredFeatures } from '@haneullabs/wallet-standard';
import { STASHED_WALLET_NAME } from '@haneullabs/zksend';

import { createInMemoryStore } from '../utils/stateStorage.js';

export const HANEUL_WALLET_NAME = 'Haneul Wallet';

export const DEFAULT_STORAGE =
	typeof window !== 'undefined' && window.localStorage ? localStorage : createInMemoryStore();

export const DEFAULT_STORAGE_KEY = 'haneul-dapp-kit:wallet-connection-info';

const SIGN_FEATURES = [
	'haneul:signTransaction',
	'haneul:signTransactionBlock',
] satisfies (keyof HaneulWalletFeatures)[];

export const DEFAULT_WALLET_FILTER = (wallet: WalletWithRequiredFeatures) =>
	SIGN_FEATURES.some((feature) => wallet.features[feature]);

export const DEFAULT_PREFERRED_WALLETS = [HANEUL_WALLET_NAME, STASHED_WALLET_NAME];
