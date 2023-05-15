// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '../api/RpcClientContext';
import {
    normalizeHaneulObjectId,
    type HaneulMoveNormalizedStruct,
} from '@haneullabs/haneul.js';
import { useQuery, type UseQueryOptions } from '@tanstack/react-query';

type GetNormalizedMoveStructOptions = {
    packageId: string;
    module: string;
    struct: string;
} & Pick<
    UseQueryOptions<HaneulMoveNormalizedStruct, unknown>,
    'onSuccess' | 'onError'
>;

export function useGetNormalizedMoveStruct(
    options: GetNormalizedMoveStructOptions
) {
    const { packageId, module, struct, ...useQueryOptions } = options;
    const rpc = useRpcClient();
    return useQuery({
        queryKey: ['normalized-struct', packageId, module, struct],
        queryFn: () =>
            rpc.getNormalizedMoveStruct({
                package: normalizeHaneulObjectId(packageId),
                module,
                struct,
            }),
        enabled: !!packageId && !!module && !!struct,
        ...useQueryOptions,
    });
}
