// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import { useQuery } from '@tanstack/react-query';

import type { HaneulAddress, HaneulObjectResponseQuery } from '@haneullabs/haneul.js';

export function useObjectsOwnedByAddress(
    address?: HaneulAddress | null,
    query?: HaneulObjectResponseQuery
) {
    const rpc = useRpcClient();
    return useQuery(
        ['objects-owned', address, query],
        () =>
            rpc.getOwnedObjects({
                owner: address!,
                filter: query?.filter,
                options: query?.options,
            }),
        {
            enabled: !!address,
        }
    );
}
