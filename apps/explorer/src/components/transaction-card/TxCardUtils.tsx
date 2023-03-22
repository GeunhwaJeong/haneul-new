// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin } from '@haneullabs/core';
import { X12 } from '@haneullabs/icons';
import {
    type ExecutionStatusType,
    getExecutionStatusType,
    getTotalGasUsed,
    getTransactionDigest,
    getTransactionKind,
    getTransactionKindName,
    getTransactionSender,
    type GetTxnDigestsResponse,
    type JsonRpcProvider,
    HANEUL_TYPE_ARG,
    type TransactionKindName,
} from '@haneullabs/haneul.js';
import clsx from 'clsx';
import { type ReactNode } from 'react';

import { getAmount } from '../../utils/getAmount';
import { TxTimeType } from '../tx-time/TxTimeType';

import styles from './RecentTxCard.module.css';

import { AddressLink, TransactionLink } from '~/ui/InternalLink';

export type TxnData = {
    To?: string;
    txId: string;
    status: ExecutionStatusType;
    txGas: number;
    haneulAmount: bigint | number;
    coinType?: string | null;
    kind: TransactionKindName | undefined;
    From: string;
    timestamp_ms?: number;
};

export function HaneulAmount({
    amount,
}: {
    amount: bigint | number | string | undefined | null;
}) {
    const [formattedAmount, coinType] = useFormatCoin(amount, HANEUL_TYPE_ARG);

    if (amount) {
        const HaneulSuffix = <abbr className={styles.haneulsuffix}>{coinType}</abbr>;

        return (
            <section>
                <span className={styles.haneulamount}>
                    {formattedAmount}
                    {HaneulSuffix}
                </span>
            </section>
        );
    }

    return <span className={styles.haneulamount}>--</span>;
}

function TxTableHeader({ label }: { label: string }) {
    return <div className="pl-3">{label}</div>;
}

export function TxTableCol({
    isHighlightedOnHover,
    isFirstCol,
    children,
}: {
    isHighlightedOnHover?: boolean;
    isFirstCol?: boolean;
    children: ReactNode;
}) {
    return (
        <div
            className={clsx(
                'flex h-full items-center rounded',
                !isFirstCol && 'px-3',
                isHighlightedOnHover && 'hover:bg-haneul-light'
            )}
        >
            {children}
        </div>
    );
}

// Generate table data from the transaction data
export const genTableDataFromTxData = (results: TxnData[]) => ({
    data: results.map((txn) => ({
        date: (
            <TxTableCol>
                <TxTimeType timestamp={txn.timestamp_ms} />
            </TxTableCol>
        ),
        transactionId: (
            <TxTableCol isFirstCol isHighlightedOnHover>
                <TransactionLink
                    digest={txn.txId}
                    before={
                        txn.status === 'success' ? (
                            <div className="h-2 w-2 rounded-full bg-success" />
                        ) : (
                            <X12 className="text-issue-dark" />
                        )
                    }
                />
            </TxTableCol>
        ),
        amounts: (
            <TxTableCol>
                <HaneulAmount amount={txn.haneulAmount} />
            </TxTableCol>
        ),
        gas: (
            <TxTableCol>
                <HaneulAmount amount={txn.txGas} />
            </TxTableCol>
        ),
        sender: (
            <TxTableCol isHighlightedOnHover>
                <AddressLink address={txn.From} />
            </TxTableCol>
        ),
    })),
    columns: [
        {
            header: 'Transaction ID',
            accessorKey: 'transactionId',
        },
        {
            header: () => <TxTableHeader label="Sender" />,
            accessorKey: 'sender',
        },
        {
            header: () => <TxTableHeader label="Amount" />,
            accessorKey: 'amounts',
        },
        {
            header: () => <TxTableHeader label="Gas" />,
            accessorKey: 'gas',
        },
        {
            header: () => <TxTableHeader label="Time" />,
            accessorKey: 'date',
        },
    ],
});

const dedupe = (arr: string[]) => Array.from(new Set(arr));

export const getDataOnTxDigests = (
    rpc: JsonRpcProvider,
    transactions: GetTxnDigestsResponse
) =>
    rpc
        .multiGetTransactions({
            digests: dedupe(transactions),
            options: {
                showInput: true,
                showEffects: true,
                showEvents: true,
            },
        })
        .then((txEffs) =>
            txEffs
                .map((txEff) => {
                    const digest = transactions.filter(
                        (transactionId) =>
                            transactionId === getTransactionDigest(txEff)
                    )[0];
                    const txn = getTransactionKind(txEff)!;
                    const txKind = getTransactionKindName(txn);
                    const recipient = null;
                    //     getTransferObjectTransaction(txn)?.recipient ||
                    //     getTransferHaneulTransaction(txn)?.recipient;

                    const transfer = getAmount({
                        txnData: txEff,
                        haneulCoinOnly: true,
                    })[0];

                    // use only absolute value of haneul amount
                    const haneulAmount = transfer?.amount
                        ? Math.abs(transfer.amount)
                        : null;

                    return {
                        txId: digest,
                        status: getExecutionStatusType(txEff)!,
                        txGas: getTotalGasUsed(txEff),
                        haneulAmount,
                        coinType: transfer?.coinType || null,
                        kind: txKind,
                        From: getTransactionSender(txEff),
                        timestamp_ms: txEff.timestampMs,
                        ...(recipient
                            ? {
                                  To: recipient,
                              }
                            : {}),
                    };
                })
                // Remove failed transactions
                .filter((itm) => itm)
        );
