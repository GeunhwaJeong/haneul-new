// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    getPayHaneulTransaction,
    getPayTransaction,
    getTransferHaneulTransaction,
    getTransferObjectTransaction,
    getTransactionKindName,
} from '@haneullabs/haneul.js';

import type {
    HaneulTransactionKind,
    TransactionEffects,
    HaneulEvent,
} from '@haneullabs/haneul.js';

const getCoinType = (
    txEffects: TransactionEffects,
    address: string
): string | null => {
    const events = txEffects?.events || [];
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
    amount?: number | bigint | null;
    coinType?: string | null;
    isCoin?: boolean;
    recipientAddress: string;
}[];

export function getAmount(
    txnData: HaneulTransactionKind,
    txnEffect?: TransactionEffects
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
                          txnEffect && getCoinType(txnEffect, txn.recipient),
                      isCoin: true,
                  },
              ]
            : null;
    }

    const payHaneulData =
        getPayHaneulTransaction(txnData) ?? getPayTransaction(txnData);

    const amountByRecipient = payHaneulData?.recipients.reduce(
        (acc, value, index) => {
            return {
                ...acc,
                [value]: {
                    amount:
                        payHaneulData.amounts[index] +
                        (value in acc ? acc[value].amount : 0),
                    coinType: txnEffect
                        ? getCoinType(
                              txnEffect,
                              payHaneulData.recipients[index] ||
                                  payHaneulData.recipients[0]
                          )
                        : null,
                    recipientAddress:
                        payHaneulData.recipients[index] ||
                        payHaneulData.recipients[0],
                    isCoin: true,
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
