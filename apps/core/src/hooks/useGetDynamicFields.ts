// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClient } from '@haneullabs/dapp-kit';
import { normalizeHaneulAddress } from '@haneullabs/haneul.js/utils';
import { useInfiniteQuery } from '@tanstack/react-query';

const MAX_PAGE_SIZE = 10;

export function useGetDynamicFields(parentId: string, maxPageSize = MAX_PAGE_SIZE) {
	const client = useHaneulClient();
	return useInfiniteQuery(
		['dynamic-fields', parentId],
		({ pageParam = null }) =>
			client.getDynamicFields({
				parentId: normalizeHaneulAddress(parentId),
				cursor: pageParam,
				limit: maxPageSize,
			}),
		{
			enabled: !!parentId,
			getNextPageParam: ({ nextCursor, hasNextPage }) => (hasNextPage ? nextCursor : null),
		},
	);
}
