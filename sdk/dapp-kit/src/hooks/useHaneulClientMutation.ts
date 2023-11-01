// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { UseMutationOptions, UseMutationResult } from '@tanstack/react-query';
import { useMutation } from '@tanstack/react-query';

import { useHaneulClientContext } from './useHaneulClient.js';
import type { HaneulRpcMethods } from './useHaneulClientQuery.js';

export type UseHaneulClientMutationOptions<T extends keyof HaneulRpcMethods> = Omit<
	UseMutationOptions<HaneulRpcMethods[T]['result'], Error, HaneulRpcMethods[T]['params'], unknown[]>,
	'mutationFn'
>;

export function useHaneulClientMutation<T extends keyof HaneulRpcMethods>(
	method: T,
	options: UseHaneulClientMutationOptions<T> = {},
): UseMutationResult<HaneulRpcMethods[T]['result'], Error, HaneulRpcMethods[T]['params'], unknown[]> {
	const haneulContext = useHaneulClientContext();

	return useMutation({
		...options,
		mutationFn: async (params) => {
			return await haneulContext.client[method](params as never);
		},
	});
}
