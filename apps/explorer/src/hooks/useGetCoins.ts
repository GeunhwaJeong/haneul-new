// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import { useInfiniteQuery } from '@tanstack/react-query';

import type { HaneulAddress } from '@haneullabs/haneul.js';

const MAX_COINS_PER_REQUEST = 10;

// Fetch all coins for an address, this will keep calling the API until all coins are fetched
export function useGetCoins(coinType: string, address?: HaneulAddress | null) {
    const rpc = useRpcClient();
    return useInfiniteQuery(
        ['get-coins', address, coinType],
        async ({ pageParam }) =>
            await rpc.getCoins({
                owner: address!,
                coinType,
                cursor: pageParam ? pageParam.cursor : null,
                limit: MAX_COINS_PER_REQUEST,
            }),
        {
            getNextPageParam: (lastPage) =>
                lastPage?.hasNextPage
                    ? {
                          cursor: lastPage.nextCursor,
                      }
                    : false,
            enabled: !!address,
        }
    );
}
