// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '../api/RpcClientContext';
import { useQuery } from '@tanstack/react-query';
import { chunkArray } from '../utils/chunkArray';
import { HaneulObjectDataOptions } from '@haneullabs/haneul.js/src/client';

export function useMultiGetObjects(
	ids: string[],
	options: HaneulObjectDataOptions,
	queryOptions?: { keepPreviousData?: boolean },
) {
	const rpc = useRpcClient();
	return useQuery({
		queryKey: ['multiGetObjects', ids],
		queryFn: async () => {
			const responses = await Promise.all(
				chunkArray(ids, 50).map((chunk) =>
					rpc.multiGetObjects({
						ids: chunk,
						options,
					}),
				),
			);
			return responses.flat();
		},
		enabled: !!ids?.length,
		...queryOptions,
	});
}
