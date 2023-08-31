// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { UseMutationOptions } from '@tanstack/react-query';
import { useMutation } from '@tanstack/react-query';
import { useHaneulClientContext } from './useHaneulClient.js';
import type { HaneulRpcMethods } from './useHaneulClientQuery.js';

export type UseHaneulClientMutationOptions<T extends keyof HaneulRpcMethods> = Omit<
	UseMutationOptions<HaneulRpcMethods[T]['result'], Error, HaneulRpcMethods[T]['result'], unknown[]>,
	'mutationFn'
>;

export function useHaneulClientMutation<T extends keyof HaneulRpcMethods>(
	{
		method,
		params,
	}: {
		method: T;
		params: HaneulRpcMethods[T]['params'];
	},
	options: UseHaneulClientMutationOptions<T> = {},
) {
	const haneulContext = useHaneulClientContext();

	return useMutation({
		...options,
		mutationFn: async () => {
			return await haneulContext.client[method](params as never);
		},
	});
}
