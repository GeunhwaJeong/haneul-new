// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin, useRpcClient } from '@haneullabs/core';
import { type HaneulAddress, HANEUL_TYPE_ARG, Transaction } from '@haneullabs/haneul.js';
import { useQuery } from '@tanstack/react-query';
import { useMemo } from 'react';

export function useTransactionData(
    sender?: HaneulAddress | null,
    transaction?: Transaction | null
) {
    const rpc = useRpcClient();
    const clonedTransaction = useMemo(() => {
        if (!transaction) return;

        const tx = new Transaction(transaction);
        if (sender) {
            tx.setSenderIfNotSet(sender);
        }
        return tx;
    }, [transaction, sender]);

    return useQuery(
        ['transaction-data', clonedTransaction?.serialize()],
        async () => {
            // Build the transaction to bytes, which will ensure that the transaction data is fully populated:
            await clonedTransaction!.build({ provider: rpc });
            return clonedTransaction!.transactionData;
        },
        {
            enabled: !!clonedTransaction,
        }
    );
}

export function useTransactionGasBudget(
    sender?: HaneulAddress | null,
    transaction?: Transaction | null
) {
    const { data, ...rest } = useTransactionData(sender, transaction);

    const [formattedGas] = useFormatCoin(data?.gasConfig.budget, HANEUL_TYPE_ARG);

    return {
        data: formattedGas,
        ...rest,
    };
}
