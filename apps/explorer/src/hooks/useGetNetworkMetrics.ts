// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClient } from '@haneullabs/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetNetworkMetrics() {
	const client = useHaneulClient();
	return useQuery({
		queryKey: ['home', 'metrics'],
		queryFn: () => client.getNetworkMetrics(),
		gcTime: 24 * 60 * 60 * 1000,
		staleTime: Infinity,
		retry: 5,
	});
}
