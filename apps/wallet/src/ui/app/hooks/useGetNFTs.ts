// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useGetOwnedObjects, hasDisplayData, useGetKioskContents } from '@haneullabs/core';
import { type HaneulObjectData } from '@haneullabs/haneul.js';

import useAppSelector from './useAppSelector';

export function useGetNFTs(address?: string | null) {
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
		50,
	);
	const { apiEnv } = useAppSelector((state) => state.app);
	const disableOriginByteKiosk = apiEnv !== 'mainnet';

	const { data: kioskData, isLoading: areKioskContentsLoading } = useGetKioskContents(
		address,
		disableOriginByteKiosk,
	);

	const filteredKioskContents = (kioskData?.list ?? [])
		.filter(hasDisplayData)
		.map(({ data }) => (data as HaneulObjectData) || []);

	const nfts = [
		...(filteredKioskContents ?? []),
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
		isLoading: isLoading || areKioskContentsLoading,
		isError: isError,
		error,
	};
}
