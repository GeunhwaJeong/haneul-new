// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulAddress } from '@haneullabs/haneul.js';
import { type TransactionBlock } from '@haneullabs/haneul.js/transactions';
import { useQuery } from '@tanstack/react-query';

import { useSigner } from '_hooks';

export function useTransactionDryRun(
	sender: HaneulAddress | undefined,
	transactionBlock: TransactionBlock,
) {
	const signer = useSigner(sender);
	const response = useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: ['dryRunTransaction', transactionBlock.serialize()],
		queryFn: () => {
			return signer!.dryRunTransactionBlock({ transactionBlock });
		},
		enabled: !!signer,
	});
	return response;
}
