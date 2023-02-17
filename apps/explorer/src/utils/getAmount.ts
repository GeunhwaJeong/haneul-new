// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    getPayHaneulTransaction,
    getPayTransaction,
    getTransferHaneulTransaction,
    getTransferObjectTransaction,
    getTransactionKindName,
    getTransactionSender,
    getTransactions,
    HANEUL_TYPE_ARG,
} from '@haneullabs/haneul.js';

import type {
    HaneulTransactionKind,
    TransactionEffects,
    HaneulTransactionResponse,
    HaneulEvent,
} from '@haneullabs/haneul.js';

const getCoinType = (
    txEffects: TransactionEffects | null,
    address: string
): string | null => {
    if (!txEffects) return null;

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
    amount?: number | null;
    coinType?: string | null;
    address: string;
};

// For TransferObject, TransferHaneul, Pay, PayHaneul, transactions get the amount from the transfer data
export function getTransfersAmount(
    txnData: HaneulTransactionKind,
    txnEffect?: TransactionEffects
): FormattedBalance[] | null {
    const txKindName = getTransactionKindName(txnData);
    if (txKindName === 'TransferObject') {
        const txn = getTransferObjectTransaction(txnData);
        return txn?.recipient
            ? [
                  {
                      address: txn?.recipient,
                  },
              ]
            : null;
    }

    if (txKindName === 'TransferHaneul') {
        const txn = getTransferHaneulTransaction(txnData);
        return txn?.recipient
            ? [
                  {
                      address: txn.recipient,
                      amount: txn?.amount,
                      coinType:
                          txnEffect && getCoinType(txnEffect, txn.recipient),
                  },
              ]
            : null;
    }

    const payData = getPayHaneulTransaction(txnData) ?? getPayTransaction(txnData);

    const amountByRecipient = payData?.recipients.reduce(
        (acc, recipient, index) => ({
            ...acc,
            [recipient]: {
                amount:
                    payData.amounts[index] +
                    (recipient in acc ? acc[recipient].amount : 0),

                // for PayHaneulTransaction the coinType is HANEUL
                coinType:
                    txKindName === 'PayHaneul'
                        ? HANEUL_TYPE_ARG
                        : getCoinType(txnEffect || null, recipient),
                address: recipient,
            },
        }),
        {} as {
            [key: string]: {
                amount: number;
                coinType: string | null;
                address: string;
            };
        }
    );
    return amountByRecipient ? Object.values(amountByRecipient) : null;
}

// Get transaction amount from coinBalanceChange event for Call Txn
// Aggregate coinBalanceChange by coinType and address
function getTxnAmountFromCoinBalanceEvent(
    txEffects: TransactionEffects,
    address: string
): FormattedBalance[] {
    const events = txEffects?.events || [];
    const coinsMeta = {} as { [coinType: string]: FormattedBalance };

    events.forEach((event) => {
        if (
            'coinBalanceChange' in event &&
            event?.coinBalanceChange?.changeType &&
            ['Receive', 'Pay'].includes(event?.coinBalanceChange?.changeType) &&
            event?.coinBalanceChange?.transactionModule !== 'gas'
        ) {
            const { coinBalanceChange } = event;
            const { coinType, amount, owner, sender } = coinBalanceChange;
            const { AddressOwner } = owner as { AddressOwner: string };
            if (AddressOwner === address || address === sender) {
                coinsMeta[`${AddressOwner}${coinType}`] = {
                    amount:
                        (coinsMeta[`${AddressOwner}${coinType}`]?.amount || 0) +
                        amount,
                    coinType: coinType,
                    address: AddressOwner,
                };
            }
        }
    });
    return Object.values(coinsMeta);
}

// Get the amount from events and transfer data
// optional flag to get only HANEUL coin type for table view
export function getAmount({
    txnData,
    haneulCoinOnly = false,
}: {
    txnData: HaneulTransactionResponse;
    haneulCoinOnly?: boolean;
}) {
    const { effects, certificate } = txnData;
    const txnDetails = getTransactions(certificate)[0];
    const sender = getTransactionSender(certificate);
    const haneulTransfer = getTransfersAmount(txnDetails, effects);
    const coinBalanceChange = getTxnAmountFromCoinBalanceEvent(effects, sender);
    const transfers = haneulTransfer || coinBalanceChange;
    if (haneulCoinOnly) {
        return transfers?.filter(({ coinType }) => coinType === HANEUL_TYPE_ARG);
    }

    return transfers;
}
