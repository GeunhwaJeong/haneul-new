// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import { type HaneulAddress } from '@haneullabs/haneul.js';
import { useQuery } from '@tanstack/react-query';

export function useGetAllBalances(address?: HaneulAddress | null) {
    const rpc = useRpcClient();
    return useQuery(
        ['get-all-balance', address],
        () => rpc.getAllBalances(address!),
        // refetchInterval is set to 4 seconds
        { enabled: !!address, refetchInterval: 4000 }
    );
}
