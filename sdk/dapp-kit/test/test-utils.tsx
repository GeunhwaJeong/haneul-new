// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulClient } from '@haneullabs/haneul.js/client';
import type { IdentifierRecord, ReadonlyWalletAccount } from '@haneullabs/wallet-standard';
import { getWallets } from '@haneullabs/wallet-standard';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { HaneulClientProvider } from 'dapp-kit/src';
import { WalletProvider } from 'dapp-kit/src/components/WalletProvider.js';
import type { ComponentProps } from 'react';

import { createMockAccount } from './mocks/mockAccount.js';
import { MockWallet } from './mocks/mockWallet.js';

export function createHaneulClientContextWrapper(client: HaneulClient) {
	return function HaneulClientContextWrapper({ children }: { children: React.ReactNode }) {
		return <HaneulClientProvider networks={{ test: client }}>{children}</HaneulClientProvider>;
	};
}

export function createWalletProviderContextWrapper(
	providerProps: Omit<ComponentProps<typeof WalletProvider>, 'children'> = {},
) {
	const queryClient = new QueryClient();
	return function WalletProviderContextWrapper({ children }: { children: React.ReactNode }) {
		return (
			<HaneulClientProvider>
				<QueryClientProvider client={queryClient}>
					<WalletProvider {...providerProps}>{children}</WalletProvider>;
				</QueryClientProvider>
			</HaneulClientProvider>
		);
	};
}

export function registerMockWallet({
	walletName,
	accounts = [createMockAccount()],
	features = {},
}: {
	walletName: string;
	accounts?: ReadonlyWalletAccount[];
	features?: IdentifierRecord<unknown>;
}) {
	const walletsApi = getWallets();
	const mockWallet = new MockWallet(walletName, accounts, features);
	return {
		unregister: walletsApi.register(mockWallet),
		mockWallet,
	};
}
