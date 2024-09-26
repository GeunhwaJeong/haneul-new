// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulClient } from '@haneullabs/haneul/client';
import type {
	UndefinedInitialDataOptions,
	UseQueryOptions,
	UseQueryResult,
} from '@tanstack/react-query';
import { queryOptions, useQuery, useSuspenseQuery } from '@tanstack/react-query';
import { useMemo } from 'react';

import type { PartialBy } from '../types/utilityTypes.js';
import { useHaneulClientContext } from './useHaneulClient.js';

export type HaneulRpcMethodName = {
	[K in keyof HaneulClient]: HaneulClient[K] extends ((input: any) => Promise<any>) | (() => Promise<any>)
		? K
		: never;
}[keyof HaneulClient];

export type HaneulRpcMethods = {
	[K in HaneulRpcMethodName]: HaneulClient[K] extends (input: infer P) => Promise<infer R>
		? {
				name: K;
				result: R;
				params: P;
			}
		: HaneulClient[K] extends () => Promise<infer R>
			? {
					name: K;
					result: R;
					params: undefined | object;
				}
			: never;
};

export type UseHaneulClientQueryOptions<T extends keyof HaneulRpcMethods, TData> = PartialBy<
	Omit<UseQueryOptions<HaneulRpcMethods[T]['result'], Error, TData, unknown[]>, 'queryFn'>,
	'queryKey'
>;

export type GetHaneulClientQueryOptions<T extends keyof HaneulRpcMethods> = {
	client: HaneulClient;
	network: string;
	method: T;
	options?: PartialBy<
		Omit<UndefinedInitialDataOptions<HaneulRpcMethods[T]['result']>, 'queryFn'>,
		'queryKey'
	>;
} & (undefined extends HaneulRpcMethods[T]['params']
	? { params?: HaneulRpcMethods[T]['params'] }
	: { params: HaneulRpcMethods[T]['params'] });

export function getHaneulClientQuery<T extends keyof HaneulRpcMethods>({
	client,
	network,
	method,
	params,
	options,
}: GetHaneulClientQueryOptions<T>) {
	return queryOptions<HaneulRpcMethods[T]['result']>({
		...options,
		queryKey: [network, method, params],
		queryFn: async () => {
			return await client[method](params as never);
		},
	});
}

export function useHaneulClientQuery<
	T extends keyof HaneulRpcMethods,
	TData = HaneulRpcMethods[T]['result'],
>(
	...args: undefined extends HaneulRpcMethods[T]['params']
		? [method: T, params?: HaneulRpcMethods[T]['params'], options?: UseHaneulClientQueryOptions<T, TData>]
		: [method: T, params: HaneulRpcMethods[T]['params'], options?: UseHaneulClientQueryOptions<T, TData>]
): UseQueryResult<TData, Error> {
	const [method, params, { queryKey = [], ...options } = {}] = args as [
		method: T,
		params?: HaneulRpcMethods[T]['params'],
		options?: UseHaneulClientQueryOptions<T, TData>,
	];

	const haneulContext = useHaneulClientContext();

	return useQuery({
		...options,
		queryKey: [haneulContext.network, method, params, ...queryKey],
		queryFn: async () => {
			return await haneulContext.client[method](params as never);
		},
	});
}

export function useHaneulClientSuspenseQuery<
	T extends keyof HaneulRpcMethods,
	TData = HaneulRpcMethods[T]['result'],
>(
	...args: undefined extends HaneulRpcMethods[T]['params']
		? [method: T, params?: HaneulRpcMethods[T]['params'], options?: UndefinedInitialDataOptions<TData>]
		: [method: T, params: HaneulRpcMethods[T]['params'], options?: UndefinedInitialDataOptions<TData>]
) {
	const [method, params, options = {}] = args as [
		method: T,
		params?: HaneulRpcMethods[T]['params'],
		options?: UndefinedInitialDataOptions<TData>,
	];

	const haneulContext = useHaneulClientContext();

	const query = useMemo(() => {
		return getHaneulClientQuery<T>({
			client: haneulContext.client,
			network: haneulContext.network,
			method,
			params,
			options,
		});
	}, [haneulContext.client, haneulContext.network, method, params, options]);

	return useSuspenseQuery(query);
}
