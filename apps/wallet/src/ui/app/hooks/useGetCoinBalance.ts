// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulAddress } from '@haneullabs/haneul.js';
import { useQuery } from '@tanstack/react-query';

import { useRpc } from '_hooks';

export function useGetCoinBalance(
    coinType: string,
    address?: HaneulAddress | null
) {
    const rpc = useRpc();
    return useQuery(
        ['coin-balance', address, coinType],
        () => rpc.getBalance(address!, coinType),
        {
            enabled: !!address && !!coinType,
            refetchInterval: 4000,
        }
    );
}
