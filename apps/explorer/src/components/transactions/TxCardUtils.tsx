// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin } from '@haneullabs/core';
import { X12 } from '@haneullabs/icons';
import {
    getExecutionStatusType,
    getTotalGasUsed,
    getTransactionSender,
    type GetTxnDigestsResponse,
    type JsonRpcProvider,
    HANEUL_TYPE_ARG,
    type HaneulTransactionResponse,
} from '@haneullabs/haneul.js';
import clsx from 'clsx';
import { type ReactNode } from 'react';

import { getAmount } from '../../utils/getAmount';
import { TxTimeType } from '../tx-time/TxTimeType';

import styles from './RecentTxCard.module.css';

import { AddressLink, TransactionLink } from '~/ui/InternalLink';

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
export const genTableDataFromTxData = (results: HaneulTransactionResponse[]) => ({
    data: results.map((transaction) => {
        const status = getExecutionStatusType(transaction);
        const transfer = getAmount({
            txnData: transaction,
            haneulCoinOnly: true,
        })[0];

        // use only absolute value of haneul amount
        const haneulAmount = transfer?.amount ? Math.abs(transfer.amount) : null;
        const sender = getTransactionSender(transaction);

        return {
            date: (
                <TxTableCol>
                    <TxTimeType timestamp={transaction.timestampMs} />
                </TxTableCol>
            ),
            transactionId: (
                <TxTableCol isFirstCol isHighlightedOnHover>
                    <TransactionLink
                        digest={transaction.digest}
                        before={
                            status === 'success' ? (
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
                    <HaneulAmount amount={haneulAmount} />
                </TxTableCol>
            ),
            gas: (
                <TxTableCol>
                    <HaneulAmount amount={getTotalGasUsed(transaction)} />
                </TxTableCol>
            ),
            sender: (
                <TxTableCol isHighlightedOnHover>
                    {sender ? <AddressLink address={sender} /> : '-'}
                </TxTableCol>
            ),
        };
    }),
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
        .then((transactions) =>
            // Remove failed transactions
            transactions.filter((item) => item)
        );
