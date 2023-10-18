// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulClient } from '@haneullabs/haneul.js/client';
import type { UseInfiniteQueryOptions } from '@tanstack/react-query';
import { useInfiniteQuery } from '@tanstack/react-query';

import { useHaneulClientContext } from './useHaneulClient.js';

interface PaginatedResult {
	data?: unknown;
	nextCursor?: unknown;
	hasNextPage: boolean;
}
export type HaneulRpcPaginatedMethodName = {
	[K in keyof HaneulClient]: HaneulClient[K] extends (input: any) => Promise<PaginatedResult> ? K : never;
}[keyof HaneulClient];

export type HaneulRpcPaginatedMethods = {
	[K in HaneulRpcPaginatedMethodName]: HaneulClient[K] extends (input: infer P) => Promise<{
		data?: infer R;
		nextCursor?: infer Cursor | null;
		hasNextPage: boolean;
	}>
		? {
				name: K;
				result: {
					data?: R;
					nextCursor?: Cursor | null;
					hasNextPage: boolean;
				};
				params: P;
				cursor: Cursor;
		  }
		: never;
};

export type UseHaneulClientInfiniteQueryOptions<T extends keyof HaneulRpcPaginatedMethods> = Omit<
	UseInfiniteQueryOptions<
		HaneulRpcPaginatedMethods[T]['result'],
		Error,
		HaneulRpcPaginatedMethods[T]['result'],
		HaneulRpcPaginatedMethods[T]['result'],
		unknown[]
	>,
	'queryFn'
>;

export function useHaneulClientInfiniteQuery<T extends keyof HaneulRpcPaginatedMethods>(
	method: T,
	params: HaneulRpcPaginatedMethods[T]['params'],
	{ queryKey = [], enabled = !!params, ...options }: UseHaneulClientInfiniteQueryOptions<T> = {},
) {
	const haneulContext = useHaneulClientContext();

	return useInfiniteQuery({
		...options,
		queryKey: [haneulContext.network, method, params, ...queryKey],
		enabled,
		queryFn: () => haneulContext.client[method](params as never),
		getNextPageParam: (lastPage) => {
			return (lastPage as PaginatedResult).nextCursor ?? null;
		},
	});
}
