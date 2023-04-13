// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import {
    type HaneulTransactionBlockResponse,
    type HaneulAddress,
} from '@haneullabs/haneul.js';
import { useQuery } from '@tanstack/react-query';

export function useQueryTransactionsByAddress(address: HaneulAddress | null) {
    const rpc = useRpcClient();

    return useQuery(
        ['transactions-by-address', address],
        async () => {
            // combine from and to transactions
            const [txnIds, fromTxnIds] = await Promise.all([
                rpc.queryTransactionBlocks({
                    filter: {
                        ToAddress: address!,
                    },
                    options: {
                        showInput: true,
                        showEffects: true,
                        showEvents: true,
                    },
                }),
                rpc.queryTransactionBlocks({
                    filter: {
                        FromAddress: address!,
                    },
                    options: {
                        showInput: true,
                        showEffects: true,
                        showEvents: true,
                    },
                }),
            ]);

            const inserted = new Map();
            const uniqueList: HaneulTransactionBlockResponse[] = [];

            [...txnIds.data, ...fromTxnIds.data]
                .sort(
                    (a, b) =>
                        Number(b.timestampMs ?? 0) - Number(a.timestampMs ?? 0)
                )
                .forEach((txb) => {
                    if (inserted.get(txb.digest)) return;
                    uniqueList.push(txb);
                    inserted.set(txb.digest, true);
                });

            return uniqueList;
        },
        { enabled: !!address, staleTime: 10 * 1000 }
    );
}
