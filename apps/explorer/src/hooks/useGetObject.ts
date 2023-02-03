// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    is,
    HaneulObject,
    type GetObjectDataResponse,
    normalizeHaneulAddress,
    type MoveHaneulSystemObjectFields,
} from '@haneullabs/haneul.js';
import { useQuery, type UseQueryResult } from '@tanstack/react-query';

import { useRpc } from './useRpc';

export function useGetSystemObject() {
    // TODO: Replace with `haneul_getHaneulSystemState` once it's supported:
    const { data, ...query } = useGetObject('0x5');

    const systemObject =
        data &&
        is(data.details, HaneulObject) &&
        data.details.data.dataType === 'moveObject'
            ? (data.details.data.fields as MoveHaneulSystemObjectFields)
            : null;

    return {
        ...query,
        data: systemObject,
    };
}

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
