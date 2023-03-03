// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import {
    type GetObjectDataResponse,
    normalizeHaneulAddress,
} from '@haneullabs/haneul.js';
import { useQuery, type UseQueryResult } from '@tanstack/react-query';

export function useGetObject(
    objectId: string
): UseQueryResult<GetObjectDataResponse, unknown> {
    const rpc = useRpcClient();
    const normalizedObjId = normalizeHaneulAddress(objectId);
    const response = useQuery(
        ['object', normalizedObjId],
        async () => {
            return rpc.getObject(normalizedObjId);
        },
        { enabled: !!objectId }
    );

    return response;
}
