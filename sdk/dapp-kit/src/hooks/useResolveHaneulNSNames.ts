// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { ResolvedNameServiceNames } from '@haneullabs/haneul.js/client';
import type { UseQueryOptions } from '@tanstack/react-query';

import { useHaneulClientQuery } from './useHaneulClientQuery.js';

export function useResolveHaneulNSName(
	address?: string | null,
	options?: Omit<
		UseQueryOptions<ResolvedNameServiceNames, Error, ResolvedNameServiceNames, unknown[]>,
		'queryFn'
	>,
) {
	const { data, ...rest } = useHaneulClientQuery(
		'resolveNameServiceNames',
		{
			address: address!,
			limit: 1,
		},
		{
			refetchOnWindowFocus: false,
			retry: false,
			...options,
			enabled: !!address && options?.enabled !== false,
		},
	);

	return { data: data?.data?.[0] ?? null, ...rest };
}
