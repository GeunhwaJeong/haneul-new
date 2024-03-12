// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { SentryHttpTransport } from '@haneullabs/core';
import { HaneulClient, HaneulHTTPTransport, getFullnodeUrl } from '@haneullabs/haneul.js/client';

export enum Network {
	LOCAL = 'LOCAL',
	DEVNET = 'DEVNET',
	TESTNET = 'TESTNET',
	MAINNET = 'MAINNET',
}

export const NetworkConfigs: Record<Network, { url: string }> = {
	[Network.LOCAL]: { url: getFullnodeUrl('localnet') },
	[Network.DEVNET]: { url: 'https://haneul-devnet.haneul-labs.com/json-rpc' },
	[Network.TESTNET]: { url: 'https://haneul-testnet.haneul-labs.com/json-rpc' },
	[Network.MAINNET]: { url: 'https://haneul-mainnet.haneul-labs.com/json-rpc' },
};

const defaultClientMap: Map<Network | string, HaneulClient> = new Map();

// NOTE: This class should not be used directly in React components, prefer to use the useHaneulClient() hook instead
export const createHaneulClient = (network: Network | string) => {
	const existingClient = defaultClientMap.get(network);
	if (existingClient) return existingClient;

	const networkUrl = network in Network ? NetworkConfigs[network as Network].url : network;

	const client = new HaneulClient({
		transport:
			network in Network && network === Network.MAINNET
				? new SentryHttpTransport(networkUrl)
				: new HaneulHTTPTransport({ url: networkUrl }),
	});
	defaultClientMap.set(network, client);
	return client;
};
