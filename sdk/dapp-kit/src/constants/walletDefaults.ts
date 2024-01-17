// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { WalletWithRequiredFeatures } from '@haneullabs/wallet-standard';
import { ZKSEND_WALLET_NAME } from '@haneullabs/zksend';

import { createInMemoryStore } from '../utils/stateStorage.js';

export const HANEUL_WALLET_NAME = 'Haneul Wallet';

export const DEFAULT_STORAGE =
	typeof window !== 'undefined' && window.localStorage ? localStorage : createInMemoryStore();

export const DEFAULT_STORAGE_KEY = 'haneul-dapp-kit:wallet-connection-info';

export const DEFAULT_REQUIRED_FEATURES: (keyof WalletWithRequiredFeatures['features'])[] = [
	'haneul:signTransactionBlock',
];

export const DEFAULT_PREFERRED_WALLETS = [HANEUL_WALLET_NAME, ZKSEND_WALLET_NAME];
