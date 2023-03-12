// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@haneullabs/core';
import { useQuery } from '@tanstack/react-query';

import {
    genTableDataFromTxData,
    getDataOnTxDigests,
    type TxnData,
} from '~/components/transaction-card/TxCardUtils';
import { TableCard } from '~/ui/TableCard';

export function CheckpointTransactions({
    digest,
    transactions,
}: {
    digest: string;
    transactions: string[];
}) {
    const rpc = useRpcClient();
    const { data: txData, isLoading } = useQuery(
        ['checkpoint-transactions', digest],
        async () => {
            // todo: replace this with `haneul_queryTransactions` call when we are
            // able to query by checkpoint digest
            const txData = await getDataOnTxDigests(rpc, transactions!);
            return genTableDataFromTxData(txData as TxnData[]);
        },
        { enabled: !!transactions.length }
    );
    if (isLoading) return null;

    return txData ? (
        <TableCard data={txData?.data} columns={txData?.columns} />
    ) : null;
}
