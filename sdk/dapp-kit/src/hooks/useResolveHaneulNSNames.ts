// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClientQuery } from './useHaneulClientQuery.js';

export function useResolveHaneulNSName(address?: string | null) {
	const { data, ...rest } = useHaneulClientQuery(
		'resolveNameServiceNames',
		{
			address: address!,
			limit: 1,
		},
		{
			enabled: !!address,
			refetchOnWindowFocus: false,
			retry: false,
		},
	);

	return { data: data?.data?.[0] ?? null, ...rest };
}
