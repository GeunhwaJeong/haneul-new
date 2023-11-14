// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { hasDisplayData, isKioskOwnerToken, useGetOwnedObjects } from '@haneullabs/core';
import { useKioskClient } from '@haneullabs/core/src/hooks/useKioskClient';
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
	const kioskClient = useKioskClient();
	const {
		data,
		isPending,
		error,
		isError,
		isFetchingNextPage,
		hasNextPage,
		fetchNextPage,
		isLoading,
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
				if (hasDisplayData(curr) || isKioskOwnerToken(kioskClient.network, curr))
					acc.visual.push(curr.data as HaneulObjectData);
				if (!hasDisplayData(curr)) acc.other.push(curr.data as HaneulObjectData);
				if (hiddenAssetIds.includes(curr.data?.objectId!))
					acc.hidden.push(curr.data as HaneulObjectData);
				return acc;
			}, ownedAssets);
	}, [hiddenAssetIds, data?.pages, kioskClient.network]);

	return {
		data: assets,
		isLoading,
		hasNextPage,
		isFetchingNextPage,
		fetchNextPage,
		isPending: isPending,
		isError: isError,
		error,
	};
}
