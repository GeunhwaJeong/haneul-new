// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFeatureIsOn } from '@growthbook/growthbook-react';
import { useHaneulClientContext } from '@haneullabs/dapp-kit';
import { useQuery } from '@tanstack/react-query';
import { Network } from '~/utils/api/DefaultRpcClient';

type UseVerifiedSourceCodeArgs = {
	packageId: string;
	moduleName: string;
};

type UseVerifiedSourceCodeResponse = {
	source?: string;
	error?: string;
};

const networksWithSourceCodeVerification: Network[] = [
	Network.DEVNET,
	Network.TESTNET,
	Network.MAINNET,
];

/**
 * Hook that retrieves the source code for verified modules.
 */
export function useVerifiedSourceCode({ packageId, moduleName }: UseVerifiedSourceCodeArgs) {
	const { network } = useHaneulClientContext();
	const isEnabled = useFeatureIsOn('module-source-verification');

	return useQuery({
		queryKey: ['verified-source-code', packageId, moduleName, network],
		queryFn: async () => {
			const response = await fetch(
				`https://source.haneul-labs.com/api?network=${network.toLowerCase()}&address=${packageId}&module=${moduleName}`,
			);
			if (!response.ok) {
				throw new Error(`Encountered unexpected response: ${response.status}`);
			}

			const jsonResponse: UseVerifiedSourceCodeResponse = await response.json();
			return jsonResponse.source || null;
		},
		enabled: isEnabled && networksWithSourceCodeVerification.includes(network as Network),
	});
}
