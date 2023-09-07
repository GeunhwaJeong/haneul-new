// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulClient } from '@haneullabs/haneul.js/client';
import type { IdentifierRecord } from '@haneullabs/wallet-standard';
import { getWallets } from '@haneullabs/wallet-standard';
import { HaneulClientProvider, WalletProvider } from 'dapp-kit/src';
import { MockWallet } from './mockWallet.js';
import type { ComponentProps } from 'react';

export function createHaneulClientContextWrapper(client: HaneulClient) {
	return function HaneulClientContextWrapper({ children }: { children: React.ReactNode }) {
		return <HaneulClientProvider networks={{ test: client }}>{children}</HaneulClientProvider>;
	};
}

export function createWalletProviderContextWrapper(
	providerProps: Omit<ComponentProps<typeof WalletProvider>, 'children'> = {},
) {
	return function WalletProviderContextWrapper({ children }: { children: React.ReactNode }) {
		return <WalletProvider {...providerProps}>{children}</WalletProvider>;
	};
}

export function registerMockWallet(
	walletName: string,
	additionalFeatures: IdentifierRecord<unknown> = {},
) {
	const walletsApi = getWallets();
	return walletsApi.register(new MockWallet(walletName, additionalFeatures));
}
