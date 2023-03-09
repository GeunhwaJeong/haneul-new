// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    getPayHaneulTransaction,
    getPayTransaction,
    getTransferHaneulTransaction,
    getTransferObjectTransaction,
    getTransactionKindName,
    getTransactionSender,
    getTransactionKinds,
    HANEUL_TYPE_ARG,
    getCoinBalanceChangeEvent,
} from '@haneullabs/haneul.js';

import type {
    HaneulTransactionKind,
    TransactionEffects,
    HaneulTransactionResponse,
    HaneulEvent,
    TransactionEvents,
} from '@haneullabs/haneul.js';

const getCoinType = (
    events: TransactionEvents | null,
    address: string
): string | null => {
    if (!events) return null;

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
    txnEffect?: TransactionEffects,
    events?: TransactionEvents
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
                      coinType: events && getCoinType(events, txn.recipient),
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
                    Number(payData.amounts[index]) +
                    (recipient in acc ? acc[recipient].amount : 0),

                // for PayHaneulTransaction the coinType is HANEUL
                coinType:
                    txKindName === 'PayHaneul'
                        ? HANEUL_TYPE_ARG
                        : getCoinType(events || null, recipient),
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
    events: TransactionEvents,
    address: string
): FormattedBalance[] {
    const coinsMeta = {} as { [coinType: string]: FormattedBalance };

    events.forEach((event) => {
        if (
            event.type === 'coinBalanceChange' &&
            event?.content?.changeType &&
            ['Receive', 'Pay'].includes(event?.content?.changeType) &&
            event?.content?.transactionModule !== 'gas'
        ) {
            const coinBalanceChange = getCoinBalanceChangeEvent(event)!;
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
    const { effects, events } = txnData;
    const txnDetails = getTransactionKinds(txnData)![0];
    const sender = getTransactionSender(txnData);
    const haneulTransfer = getTransfersAmount(txnDetails, effects);
    const coinBalanceChange = getTxnAmountFromCoinBalanceEvent(
        events!,
        sender!
    );
    const transfers = haneulTransfer || coinBalanceChange;
    if (haneulCoinOnly) {
        return transfers?.filter(({ coinType }) => coinType === HANEUL_TYPE_ARG);
    }

    return transfers;
}
