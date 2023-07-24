// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { hasDisplayData, isKioskOwnerToken, useGetOwnedObjects } from '@haneullabs/core';
import { type HaneulObjectData } from '@haneullabs/haneul.js';

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

	const ownedAssets =
		data?.pages
			.flatMap((page) => page.data)
			.sort((object) => (hasDisplayData(object) ? -1 : 1))
			.sort((object) => (isKioskOwnerToken(object) ? -1 : 1))
			.map(({ data }) => data as HaneulObjectData) ?? [];

	return {
		data: ownedAssets,
		isInitialLoading,
		hasNextPage,
		isFetchingNextPage,
		fetchNextPage,
		isLoading: isLoading,
		isError: isError,
		error,
	};
}
