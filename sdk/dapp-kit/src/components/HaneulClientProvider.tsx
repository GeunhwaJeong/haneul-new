// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulClient, getFullnodeUrl, isHaneulClient } from '@haneullabs/haneul.js/client';
import type { HaneulClientOptions } from '@haneullabs/haneul.js/client';
import { createContext, useMemo, useState } from 'react';

type NetworkConfig = HaneulClient | HaneulClientOptions;
type NetworkConfigs<T extends NetworkConfig = NetworkConfig> = Record<string, T>;

export interface HaneulClientProviderContext {
	client: HaneulClient;
	networks: NetworkConfigs;
	selectedNetwork: string;
	selectNetwork: (network: string) => void;
}

export const HaneulClientContext = createContext<HaneulClientProviderContext | null>(null);

export interface HaneulClientProviderProps<T extends NetworkConfigs> {
	networks?: T;
	createClient?: (name: keyof T, config: T[keyof T]) => HaneulClient;
	defaultNetwork?: keyof T & string;
	children: React.ReactNode;
}

const DEFAULT_NETWORKS = {
	localnet: { url: getFullnodeUrl('localnet') },
};

const DEFAULT_CREATE_CLIENT = function createClient(
	_name: string,
	config: NetworkConfig | HaneulClient,
) {
	if (isHaneulClient(config)) {
		return config;
	}

	return new HaneulClient(config);
};

export function HaneulClientProvider<T extends NetworkConfigs>(props: HaneulClientProviderProps<T>) {
	const networks = (props.networks ?? DEFAULT_NETWORKS) as T;
	const createClient =
		(props.createClient as typeof DEFAULT_CREATE_CLIENT) ?? DEFAULT_CREATE_CLIENT;

	const [selectedNetwork, setSelectedNetwork] = useState<keyof T & string>(
		props.defaultNetwork ?? (Object.keys(networks)[0] as keyof T & string),
	);

	const [client, setClient] = useState<HaneulClient>(() => {
		return createClient(selectedNetwork, networks[selectedNetwork]);
	});

	const ctx = useMemo((): HaneulClientProviderContext => {
		return {
			client,
			networks,
			selectedNetwork,
			selectNetwork: (network) => {
				if (network !== selectedNetwork) {
					setSelectedNetwork(network);
					setClient(createClient(network, networks[network]));
				}
			},
		};
	}, [client, setClient, createClient, selectedNetwork, networks]);

	return <HaneulClientContext.Provider value={ctx}>{props.children}</HaneulClientContext.Provider>;
}
