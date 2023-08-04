// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulClient, getFullnodeUrl } from '@haneullabs/haneul.js/client';
import { HaneulClientContext } from '../hooks/useHaneulClient.js';
import { useMemo } from 'react';

export interface HaneulClientProviderProps {
	children: React.ReactNode;
	client?: HaneulClient;
	url?: string;
}

export const HaneulClientProvider = (props: HaneulClientProviderProps) => {
	const client = useMemo(
		() =>
			props.client ??
			new HaneulClient({
				url: props.url ?? getFullnodeUrl('devnet'),
			}),
		[props.client, props.url],
	);

	return <HaneulClientContext.Provider value={client}>{props.children}</HaneulClientContext.Provider>;
};
