// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulClient, getFullnodeUrl } from '@haneullabs/haneul.js/client';
import { HaneulClientContext } from '../hooks/useHaneulClient.js';
import { useMemo } from 'react';

export interface HaneulClientProviderProps {
	children: React.ReactNode;
	client?: HaneulClient;
	url?: string;
	queryKeyPrefix: string;
}

export const HaneulClientProvider = (props: HaneulClientProviderProps) => {
	const ctx = useMemo(() => {
		const client =
			props.client ??
			new HaneulClient({
				url: props.url ?? getFullnodeUrl('devnet'),
			});

		return {
			client,
			queryKey: (key: unknown[]) => [props.queryKeyPrefix, ...key],
		};
	}, [props.client, props.url, props.queryKeyPrefix]);

	return <HaneulClientContext.Provider value={ctx}>{props.children}</HaneulClientContext.Provider>;
};
