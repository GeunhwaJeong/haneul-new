// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulTransactionResponse, type HaneulAddress } from '@haneullabs/haneul.js';
import { useQuery } from '@tanstack/react-query';

import { useRpc } from '_hooks';

export function useGetTransactionsByAddress(address: HaneulAddress | null) {
    const rpc = useRpc();

    const response = useQuery<HaneulTransactionResponse[], Error>(
        ['transactions-by-address', address],
        async () => {
            if (!address) return [];
            const txnIdDs = await rpc.getTransactions({
                ToAddress: address,
            });
            return rpc.getTransactionWithEffectsBatch(txnIdDs.data);
        },
        { enabled: !!address, staleTime: 10 * 1000 }
    );
    return response;
}
