// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import { type HaneulAddress } from '@haneullabs/haneul.js';
import { useQuery } from '@tanstack/react-query';

export function useGetAllBalances(address?: HaneulAddress | null) {
    const rpc = useRpcClient();
    return useQuery({
        queryKey: ['get-all-balances', address],
        queryFn: async () => await rpc.getAllBalances({ owner: address! }),
        enabled: !!address,
    });
}
