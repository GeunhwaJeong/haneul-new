// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulAddress, type Transaction } from '@haneullabs/haneul.js';
import { useQuery } from '@tanstack/react-query';

import { useSigner } from '_hooks';

export function useTransactionDryRun(
    sender: HaneulAddress | undefined,
    transaction: Transaction
) {
    const signer = useSigner(sender);
    const response = useQuery({
        queryKey: ['dryRunTransaction', transaction.serialize()],
        queryFn: () => {
            return signer.dryRunTransaction({ transaction });
        },
    });
    return response;
}
