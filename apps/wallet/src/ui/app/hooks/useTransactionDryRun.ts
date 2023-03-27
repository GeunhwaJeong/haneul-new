// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulAddress, type TransactionBlock } from '@haneullabs/haneul.js';
import { useQuery } from '@tanstack/react-query';

import { useSigner } from '_hooks';

export function useTransactionDryRun(
    sender: HaneulAddress | undefined,
    transactionBlock: TransactionBlock
) {
    const signer = useSigner(sender);
    const response = useQuery({
        queryKey: ['dryRunTransaction', transactionBlock.serialize()],
        queryFn: () => {
            return signer.dryRunTransactionBlock({ transactionBlock });
        },
    });
    return response;
}
