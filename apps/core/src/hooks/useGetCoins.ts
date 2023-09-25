// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClient } from '@haneullabs/dapp-kit';
import { PaginatedCoins } from '@haneullabs/haneul.js/client';
import { useInfiniteQuery, UseInfiniteQueryResult } from '@tanstack/react-query';

const MAX_COINS_PER_REQUEST = 10;

export function useGetCoins(
	coinType: string,
	address?: string | null,
	maxCoinsPerRequest = MAX_COINS_PER_REQUEST,
): UseInfiniteQueryResult<PaginatedCoins> {
	const client = useHaneulClient();
	return useInfiniteQuery(
		['get-coins', address, coinType, maxCoinsPerRequest],
		({ pageParam }) =>
			client.getCoins({
				owner: address!,
				coinType,
				cursor: pageParam ? pageParam.cursor : null,
				limit: maxCoinsPerRequest,
			}),
		{
			getNextPageParam: ({ hasNextPage, nextCursor }) =>
				hasNextPage
					? {
							cursor: nextCursor,
					  }
					: false,
			enabled: !!address,
		},
	);
}
