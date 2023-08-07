// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { UseQueryOptions } from '@tanstack/react-query';
import { useQuery } from '@tanstack/react-query';
import { useHaneulClientContext } from './useHaneulClient.js';
import type { HaneulClient } from '@haneullabs/haneul.js/client';

type HaneulRpcMethodName = {
	[K in keyof HaneulClient]: HaneulClient[K] extends ((input: any) => Promise<any>) | (() => Promise<any>)
		? K
		: never;
}[keyof HaneulClient];

type Methods = {
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
				params: undefined;
		  }
		: never;
};

export type UseHaneulClientQueryOptions<T extends keyof Methods> = Omit<
	UseQueryOptions<Methods[T]['result'], unknown, Methods[T]['result'], unknown[]>,
	'queryFn'
>;

export function useHaneulClientQuery<T extends keyof Methods>(
	{
		method,
		params,
	}: {
		method: T;
		params: Methods[T]['params'];
	},
	{
		queryKey,

		enabled = !!params,
		...options
	}: UseHaneulClientQueryOptions<T> = {},
) {
	const haneulContext = useHaneulClientContext();

	return useQuery({
		...options,
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: haneulContext.queryKey(queryKey ?? [method, params]),
		enabled,
		queryFn: async () => {
			return await haneulContext.client[method](params as never);
		},
	});
}
