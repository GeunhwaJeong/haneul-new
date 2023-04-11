// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '../api/RpcClientContext';
import { normalizeHaneulAddress } from '@haneullabs/haneul.js';
import { useQuery } from '@tanstack/react-query';

const defaultOptions = {
    showType: true,
    showContent: true,
    showOwner: true,
    showPreviousTransaction: true,
    showStorageRebate: true,
    showDisplay: true,
};

export function useGetObject(objectId?: string | null) {
    const rpc = useRpcClient();
    const normalizedObjId = objectId && normalizeHaneulAddress(objectId);
    return useQuery(
        ['object', normalizedObjId],
        () =>
            rpc.getObject({
                id: normalizedObjId!,
                options: defaultOptions,
            }),
        { enabled: !!normalizedObjId }
    );
}
