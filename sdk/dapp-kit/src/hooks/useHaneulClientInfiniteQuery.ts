// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulClient } from '@haneullabs/haneul/client';
import type {
	InfiniteData,
	UseInfiniteQueryOptions,
	UseInfiniteQueryResult,
} from '@tanstack/react-query';
import { useInfiniteQuery } from '@tanstack/react-query';

import type { PartialBy } from '../types/utilityTypes.js';
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
	[K in HaneulRpcPaginatedMethodName]: HaneulClient[K] extends (
		input: infer Params,
	) => Promise<
		infer Result extends { hasNextPage?: boolean | null; nextCursor?: infer Cursor | null }
	>
		? {
				name: K;
				result: Result;
				params: Params;
				cursor: Cursor;
			}
		: never;
};

export type UseHaneulClientInfiniteQueryOptions<
	T extends keyof HaneulRpcPaginatedMethods,
	TData,
> = PartialBy<
	Omit<
		UseInfiniteQueryOptions<
			HaneulRpcPaginatedMethods[T]['result'],
			Error,
			TData,
			HaneulRpcPaginatedMethods[T]['result'],
			unknown[]
		>,
		'queryFn' | 'initialPageParam' | 'getNextPageParam'
	>,
	'queryKey'
>;

export function useHaneulClientInfiniteQuery<
	T extends keyof HaneulRpcPaginatedMethods,
	TData = InfiniteData<HaneulRpcPaginatedMethods[T]['result']>,
>(
	method: T,
	params: HaneulRpcPaginatedMethods[T]['params'],
	{
		queryKey = [],
		enabled = !!params,
		...options
	}: UseHaneulClientInfiniteQueryOptions<T, TData> = {},
): UseInfiniteQueryResult<TData, Error> {
	const haneulContext = useHaneulClientContext();

	return useInfiniteQuery({
		...options,
		initialPageParam: null,
		queryKey: [haneulContext.network, method, params, ...queryKey],
		enabled,
		queryFn: ({ pageParam }) =>
			haneulContext.client[method]({
				...(params ?? {}),
				cursor: pageParam,
			} as never),
		getNextPageParam: (lastPage) => (lastPage.hasNextPage ? lastPage.nextCursor ?? null : null),
	});
}
