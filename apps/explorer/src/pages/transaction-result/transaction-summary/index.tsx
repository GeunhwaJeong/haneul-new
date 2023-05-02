// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useTransactionSummary } from '@haneullabs/core';
import {
    getTransactionKind,
    getTransactionKindName,
    type HaneulTransactionBlockResponse,
} from '@haneullabs/haneul.js';

import { BalanceChanges } from './BalanceChanges';
import { ObjectChanges } from './ObjectChanges';

interface TransactionSummaryProps {
    transaction: HaneulTransactionBlockResponse;
}

export function TransactionSummary({ transaction }: TransactionSummaryProps) {
    const summary = useTransactionSummary({
        transaction,
    });

    const transactionKindName = getTransactionKindName(
        getTransactionKind(transaction)!
    );

    const balanceChanges = summary?.balanceChanges;
    const objectSummary = summary?.objectSummary;

    return (
        <div className="flex flex-wrap gap-4 md:gap-8">
            {balanceChanges &&
                transactionKindName === 'ProgrammableTransaction' && (
                    <BalanceChanges changes={balanceChanges} />
                )}
            {objectSummary && <ObjectChanges objectSummary={objectSummary} />}
        </div>
    );
}
