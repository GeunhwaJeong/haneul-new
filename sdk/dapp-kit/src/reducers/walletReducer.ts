// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { WalletWithHaneulFeatures, WalletAccount } from '@haneullabs/wallet-standard';

export type WalletState = {
	wallets: WalletWithHaneulFeatures[];
	currentWallet: WalletWithHaneulFeatures | null;
	accounts: readonly WalletAccount[];
	currentAccount: WalletAccount | null;
	connectionStatus: 'disconnected' | 'connecting' | 'connected';
};

export type WalletAction = void;

export function walletReducer(state: WalletState): WalletState {
	return state;
}
