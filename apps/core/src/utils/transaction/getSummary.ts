// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import {
    type HaneulAddress,
    type DryRunTransactionBlockResponse,
    HaneulTransactionBlockResponse,
    getTransactionSender,
    is,
} from '@haneullabs/haneul.js';

import {
    type BalanceChangeSummary,
    getBalanceChangeSummary,
} from './getBalanceChangeSummary';
import {
    ObjectChangeSummary,
    getObjectChangeSummary,
} from './getObjectChangeSummary';
import { GasSummaryType, getGasSummary } from './getGasSummary';

export type TransactionSummary = {
    digest?: string;
    sender?: HaneulAddress;
    timestamp?: string;
    balanceChanges: BalanceChangeSummary[] | null;
    gas?: GasSummaryType;
    objectSummary: ObjectChangeSummary | null;
} | null;

export const getTransactionSummary = (
    transaction: DryRunTransactionBlockResponse | HaneulTransactionBlockResponse,
    currentAddress: HaneulAddress
): TransactionSummary => {
    const { effects } = transaction;
    if (!effects) return null;

    const sender = is(transaction, HaneulTransactionBlockResponse)
        ? getTransactionSender(transaction)
        : undefined;
    const gasSummary = getGasSummary(transaction);

    const balanceChangeSummary = getBalanceChangeSummary(transaction);
    const objectChangeSummary = getObjectChangeSummary(
        transaction,
        currentAddress
    );

    return {
        sender,
        balanceChanges: balanceChangeSummary,
        gas: gasSummary,
        objectSummary: objectChangeSummary,
    };
};
