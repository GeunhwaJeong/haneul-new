// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useQuery } from '@tanstack/react-query';

import { useAppsBackend } from './useAppsBackend';

// TODO: We should consider using tRPC or something for apps-backend
type CoinData = {
    marketCap: string;
    fullyDilutedMarketCap: string;
    currentPrice: number;
    priceChangePercentageOver24H: number;
    circulatingSupply: number;
    totalSupply: number;
};

export function useHaneulCoinData() {
    const makeAppsBackendRequest = useAppsBackend();
    return useQuery(
        ['haneul-coin-data'],
        () => makeAppsBackendRequest<CoinData>('coins/haneul', {}),
        {
            // Cache this forever because we have limited API bandwidth at the moment
            cacheTime: Infinity,
        }
    );
}
