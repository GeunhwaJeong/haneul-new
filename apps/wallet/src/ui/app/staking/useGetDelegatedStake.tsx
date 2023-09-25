// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClient } from '@haneullabs/dapp-kit';
import type { DelegatedStake } from '@haneullabs/haneul.js/client';
import { useQuery, type UseQueryResult } from '@tanstack/react-query';

export function useGetDelegatedStake(address: string): UseQueryResult<DelegatedStake[], Error> {
	const client = useHaneulClient();
	return useQuery({
		queryKey: ['validator', address],
		queryFn: () => client.getStakes({ owner: address }),
		staleTime: 10 * 1000,
		refetchInterval: 30 * 1000,
	});
}
