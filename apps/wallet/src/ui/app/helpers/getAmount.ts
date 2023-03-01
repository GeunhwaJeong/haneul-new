// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    getPayHaneulTransaction,
    getPayTransaction,
    getTransferHaneulTransaction,
    getTransferObjectTransaction,
    getTransactionKindName,
    HANEUL_TYPE_ARG,
} from '@haneullabs/haneul.js';

import type {
    HaneulTransactionKind,
    TransactionEffects,
    TransactionEvents,
    HaneulEvent,
} from '@haneullabs/haneul.js';

const getCoinType = (
    txEffects: TransactionEffects,
    events: TransactionEvents,
    address: string
): string | null => {
    const coinType = events
        ?.map((event: HaneulEvent) => {
            const data = Object.values(event).find(
                (itm) => itm?.owner?.AddressOwner === address
            );
            return data?.coinType;
        })
        .filter(Boolean);
    return coinType?.[0] ? coinType[0] : null;
};

type FormattedBalance = {
    amount?: number | null;
    coinType?: string | null;
    recipientAddress: string;
}[];

export function getAmount(
    txnData: HaneulTransactionKind,
    txnEffect: TransactionEffects,
    events: TransactionEvents
): FormattedBalance | null {
    const txKindName = getTransactionKindName(txnData);
    if (txKindName === 'TransferObject') {
        const txn = getTransferObjectTransaction(txnData);
        return txn?.recipient
            ? [
                  {
                      recipientAddress: txn?.recipient,
                  },
              ]
            : null;
    }

    if (txKindName === 'TransferHaneul') {
        const txn = getTransferHaneulTransaction(txnData);
        return txn?.recipient
            ? [
                  {
                      recipientAddress: txn.recipient,
                      amount: txn?.amount,
                      coinType:
                          txnEffect &&
                          getCoinType(txnEffect, events, txn.recipient),
                  },
              ]
            : null;
    }

    const payHaneulData =
        getPayHaneulTransaction(txnData) ?? getPayTransaction(txnData);

    const amountByRecipient = payHaneulData?.recipients.reduce(
        (acc, recipient, index) => {
            const coinType =
                txKindName === 'PayHaneul'
                    ? HANEUL_TYPE_ARG
                    : getCoinType(txnEffect, events, recipient);
            return {
                ...acc,
                [recipient]: {
                    amount:
                        payHaneulData.amounts[index] +
                        (recipient in acc ? acc[recipient].amount : 0),
                    coinType,
                    recipientAddress: recipient,
                },
            };
        },
        {} as {
            [key: string]: {
                amount: number;
                coinType: string | null;
                recipientAddress: string;
            };
        }
    );

    return amountByRecipient ? Object.values(amountByRecipient) : null;
}
