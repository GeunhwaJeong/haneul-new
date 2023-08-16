// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { UseHaneulClientQueryOptions } from './useHaneulClientQuery.js';
import { useHaneulClientQuery } from './useHaneulClientQuery.js';

export function useRpcApiVersion(options?: UseHaneulClientQueryOptions<'getRpcApiVersion'>) {
	return useHaneulClientQuery(
		{
			method: 'getRpcApiVersion',
			params: {},
		},
		options,
	);
}
