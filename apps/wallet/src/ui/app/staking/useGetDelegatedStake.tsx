// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import { useQuery, type UseQueryResult } from '@tanstack/react-query';

import type { DelegatedStake } from '@haneullabs/haneul.js';

export function useGetDelegatedStake(
    address: string
): UseQueryResult<DelegatedStake[], Error> {
    const rpc = useRpcClient();
    return useQuery({
        queryKey: ['validator', address],
        queryFn: () => rpc.getStakes({ owner: address }),
        staleTime: 10 * 1000,
    });
}
