// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { hasDisplayData, isKioskOwnerToken, useGetOwnedObjects } from '@haneullabs/core';
import { type HaneulObjectData } from '@haneullabs/haneul.js/client';
import { useMemo } from 'react';

import { useHiddenAssets } from '../pages/home/hidden-assets/HiddenAssetsProvider';

type OwnedAssets = {
	visual: HaneulObjectData[];
	other: HaneulObjectData[];
	hidden: HaneulObjectData[];
};

export enum AssetFilterTypes {
	visual = 'visual',
	other = 'other',
}

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
	const { hiddenAssetIds } = useHiddenAssets();

	const assets = useMemo(() => {
		const ownedAssets: OwnedAssets = {
			visual: [],
			other: [],
			hidden: [],
		};
		return data?.pages
			.flatMap((page) => page.data)
			.filter((asset) => !hiddenAssetIds.includes(asset.data?.objectId!))
			.reduce((acc, curr) => {
				if (hasDisplayData(curr) || isKioskOwnerToken(curr))
					acc.visual.push(curr.data as HaneulObjectData);
				if (!hasDisplayData(curr)) acc.other.push(curr.data as HaneulObjectData);
				if (hiddenAssetIds.includes(curr.data?.objectId!))
					acc.hidden.push(curr.data as HaneulObjectData);
				return acc;
			}, ownedAssets);
	}, [hiddenAssetIds, data?.pages]);

	return {
		data: assets,
		isInitialLoading,
		hasNextPage,
		isFetchingNextPage,
		fetchNextPage,
		isLoading: isLoading,
		isError: isError,
		error,
	};
}
