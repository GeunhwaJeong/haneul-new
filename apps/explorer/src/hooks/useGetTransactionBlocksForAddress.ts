// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import { useInfiniteQuery } from '@tanstack/react-query';

import type { HaneulAddress, TransactionFilter } from '@haneullabs/haneul.js';

export const DEFAULT_TRANSACTIONS_LIMIT = 20;

// Fetch transaction blocks for an address, w/ toggle for to/from filter
export function useGetTransactionBlocksForAddress(
    address: HaneulAddress,
    filter?: TransactionFilter,
    limit = DEFAULT_TRANSACTIONS_LIMIT
) {
    const rpc = useRpcClient();
    return useInfiniteQuery(
        ['get-transaction-blocks', address],
        async ({ pageParam }) =>
            await rpc.queryTransactionBlocks({
                filter,
                cursor: pageParam ? pageParam.cursor : null,
                order: 'descending',
                limit,
                options: {
                    showEffects: true,
                    showBalanceChanges: true,
                    showInput: true,
                },
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
