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

export const COIN_GECKO_HANEUL_URL = 'https://www.coingecko.com/en/coins/haneul';

export function useHaneulCoinData() {
	const { request } = useAppsBackend();
	return useQuery({
		queryKey: ['haneul-coin-data'],
		queryFn: () => request<CoinData>('coins/haneul', {}),
		gcTime: 24 * 60 * 60 * 1000,
		staleTime: Infinity,
	});
}
