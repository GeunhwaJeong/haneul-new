// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    type GetObjectDataResponse,
    normalizeHaneulAddress,
} from '@haneullabs/haneul.js';
import { useQuery, type UseQueryResult } from '@tanstack/react-query';

import { useRpc } from '_hooks';

export function useGetObject(
    objectId: string
): UseQueryResult<GetObjectDataResponse, unknown> {
    const rpc = useRpc();
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
