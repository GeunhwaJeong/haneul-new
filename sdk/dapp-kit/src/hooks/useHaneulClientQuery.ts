// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulClient } from '@haneullabs/haneul.js/client';
import type { UseQueryOptions } from '@tanstack/react-query';
import { useQuery } from '@tanstack/react-query';

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
				params: undefined;
		  }
		: never;
};

export type UseHaneulClientQueryOptions<T extends keyof HaneulRpcMethods> = Omit<
	UseQueryOptions<HaneulRpcMethods[T]['result'], Error, HaneulRpcMethods[T]['result'], unknown[]>,
	'queryFn'
>;

export function useHaneulClientQuery<T extends keyof HaneulRpcMethods>(
	{
		method,
		params,
	}: {
		method: T;
		params: HaneulRpcMethods[T]['params'];
	},
	{ queryKey, enabled = !!params, ...options }: UseHaneulClientQueryOptions<T> = {},
) {
	const haneulContext = useHaneulClientContext();

	return useQuery({
		...options,
		queryKey: [haneulContext.network, method, params],
		enabled,
		queryFn: async () => {
			return await haneulContext.client[method](params as never);
		},
	});
}
