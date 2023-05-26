// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    useGetOwnedObjects,
    useGetOriginByteKioskContents,
    hasDisplayData,
} from '@haneullabs/core';
import { type HaneulObjectData, type HaneulAddress } from '@haneullabs/haneul.js';

import useAppSelector from './useAppSelector';

export function useGetNFTs(address?: HaneulAddress | null) {
    const {
        data,
        isLoading,
        error,
        isError,
        isFetchingNextPage,
        hasNextPage,
        fetchNextPage,
        isInitialLoading,
    } = useGetOwnedObjects(
        address,
        {
            MatchNone: [{ StructType: '0x2::coin::Coin' }],
        },
        50
    );
    const { apiEnv } = useAppSelector((state) => state.app);

    const shouldFetchKioskContents = apiEnv === 'mainnet';
    const { data: obKioskContents, isLoading: areKioskContentsLoading } =
        useGetOriginByteKioskContents(address, !shouldFetchKioskContents);

    const filteredKioskContents =
        obKioskContents
            ?.filter(hasDisplayData)
            .map(({ data }) => data as HaneulObjectData) || [];

    const nfts = [
        ...filteredKioskContents,
        ...(data?.pages
            .flatMap((page) => page.data)
            .filter(hasDisplayData)
            .map(({ data }) => data as HaneulObjectData) || []),
    ];
    return {
        data: nfts,
        isInitialLoading,
        hasNextPage,
        isFetchingNextPage,
        fetchNextPage,
        isLoading:
            isLoading || (shouldFetchKioskContents && areKioskContentsLoading),
        isError: isError,
        error,
    };
}
