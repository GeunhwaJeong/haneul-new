// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import { type HaneulClient } from '@haneullabs/haneul.js/client';
import { useQuery } from '@tanstack/react-query';

export function useGetAllEpochAddressMetrics(
	...input: Parameters<HaneulClient['getAllEpochAddressMetrics']>
) {
	const rpc = useRpcClient();
	return useQuery({
		queryKey: ['get', 'all', 'epoch', 'addresses', ...input],
		queryFn: () => rpc.getAllEpochAddressMetrics(...input),
	});
}
