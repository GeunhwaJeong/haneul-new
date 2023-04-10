// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import { type HaneulAddress } from '@haneullabs/haneul.js';
import { useQuery } from '@tanstack/react-query';

export function useGetOwnedObjects(address?: HaneulAddress | null) {
    const rpc = useRpcClient();
    return useQuery(
        ['get-owned-objects', address],
        async () =>
            await rpc.getOwnedObjects({
                owner: address!,
                options: {
                    showType: true,
                    showContent: true,
                    showDisplay: true,
                },
            }),
        { enabled: !!address }
    );
}
