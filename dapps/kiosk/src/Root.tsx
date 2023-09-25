// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { KioskClient, Network } from '@haneullabs/kiosk';
import { getFullnodeUrl, HaneulClient } from '@haneullabs/haneul.js/client';
import { WalletKitProvider } from '@haneullabs/wallet-kit';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Toaster } from 'react-hot-toast';
import { Outlet } from 'react-router-dom';

import { Header } from './components/Base/Header';
import { KioskClientContext } from './context/KioskClientContext';
import { RpcClientContext } from './context/RpcClientContext';

const queryClient = new QueryClient();
const haneulClient = new HaneulClient({ url: getFullnodeUrl('testnet') });

const kioskClient = new KioskClient({
	client: haneulClient,
	network: Network.TESTNET,
});

export default function Root() {
	return (
		<WalletKitProvider>
			<QueryClientProvider client={queryClient}>
				<RpcClientContext.Provider value={haneulClient}>
					<KioskClientContext.Provider value={kioskClient}>
						<Header></Header>
						<div className="min-h-[80vh]">
							<Outlet />
						</div>
						<div className="mt-6 border-t border-primary text-center py-6">
							Copyright © Haneul Labs, Inc.
						</div>
						<Toaster position="bottom-center" />
					</KioskClientContext.Provider>
				</RpcClientContext.Provider>
			</QueryClientProvider>
		</WalletKitProvider>
	);
}
