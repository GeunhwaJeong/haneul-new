// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClient } from '@haneullabs/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useNormalizedMoveModule(packageId?: string | null, moduleName?: string | null) {
	const client = useHaneulClient();
	return useQuery({
		queryKey: ['normalized-module', packageId, moduleName],
		queryFn: async () =>
			await client.getNormalizedMoveModule({
				package: packageId!,
				module: moduleName!,
			}),
		enabled: !!(packageId && moduleName),
	});
}
