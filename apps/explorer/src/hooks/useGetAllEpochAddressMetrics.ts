// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClient } from '@haneullabs/dapp-kit';
import { type HaneulClient } from '@haneullabs/haneul.js/client';
import { useQuery } from '@tanstack/react-query';

export function useGetAllEpochAddressMetrics(
	...input: Parameters<HaneulClient['getAllEpochAddressMetrics']>
) {
	const client = useHaneulClient();
	return useQuery({
		queryKey: ['get', 'all', 'epoch', 'addresses', ...input],
		queryFn: () => client.getAllEpochAddressMetrics(...input),
	});
}
