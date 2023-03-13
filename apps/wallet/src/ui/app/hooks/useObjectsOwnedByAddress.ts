// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import { useQuery } from '@tanstack/react-query';

import type { HaneulAddress } from '@haneullabs/haneul.js';

export function useObjectsOwnedByAddress(address?: HaneulAddress | null) {
    const rpc = useRpcClient();
    return useQuery(
        ['objects-owned', address],
        () => rpc.getObjectsOwnedByAddress({ owner: address! }),
        {
            enabled: !!address,
        }
    );
}
