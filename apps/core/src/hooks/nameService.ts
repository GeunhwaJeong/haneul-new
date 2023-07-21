// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useQuery } from '@tanstack/react-query';
import { useRpcClient } from '../api/RpcClientContext';
import { useFeatureIsOn } from '@growthbook/growthbook-react';

const HANEUL_NS_FEATURE_FLAG = 'haneulns';

// This should align with whatever names we want to be able to resolve.
const HANEUL_NS_DOMAINS = ['.haneul'];
export function isHaneulNSName(name: string) {
	return HANEUL_NS_DOMAINS.some((domain) => name.endsWith(domain));
}

export function useHaneulNSEnabled() {
	return useFeatureIsOn(HANEUL_NS_FEATURE_FLAG);
}

export function useResolveHaneulNSAddress(name?: string | null) {
	const rpc = useRpcClient();
	const enabled = useHaneulNSEnabled();

	return useQuery({
		queryKey: ['resolve-haneulns-address', name],
		queryFn: async () => {
			return await rpc.resolveNameServiceAddress({
				name: name!,
			});
		},
		enabled: !!name && enabled,
		refetchOnWindowFocus: false,
		retry: false,
	});
}

export function useResolveHaneulNSName(address?: string | null) {
	const rpc = useRpcClient();
	const enabled = useHaneulNSEnabled();

	return useQuery({
		queryKey: ['resolve-haneulns-name', address],
		queryFn: async () => {
			// NOTE: We only fetch 1 here because it's the default name.
			const { data } = await rpc.resolveNameServiceNames({
				address: address!,
				limit: 1,
			});

			return data[0] || null;
		},
		enabled: !!address && enabled,
		refetchOnWindowFocus: false,
		retry: false,
	});
}
