// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClient } from '@haneullabs/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetAddressMetrics() {
	const client = useHaneulClient();
	return useQuery({
		queryKey: ['home', 'addresses'],
		queryFn: () => client.getAddressMetrics(),
		gcTime: 24 * 60 * 60 * 1000,
		staleTime: Infinity,
		retry: 5,
	});
}
