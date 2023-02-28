// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useQuery } from '@tanstack/react-query';

import { useSigner } from '_hooks';

import type { SignerWithProvider, HaneulAddress } from '@haneullabs/haneul.js';

export type TransactionDryRun = Parameters<
    SignerWithProvider['dryRunTransaction']
>['0'];

export function useTransactionDryRun(
    txData: TransactionDryRun,
    addressForTransaction: HaneulAddress
) {
    const signer = useSigner(addressForTransaction);
    const response = useQuery({
        queryKey: ['executeDryRunTxn', txData, addressForTransaction],
        queryFn: async () => {
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            return signer!.dryRunTransaction(txData);
        },
        enabled: !!signer,
    });
    return response;
}
