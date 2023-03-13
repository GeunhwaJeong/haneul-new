// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    getTransactionSender,
    getTransactionKind,
    HANEUL_TYPE_ARG,
    getCoinBalanceChangeEvent,
} from '@haneullabs/haneul.js';

import type {
    HaneulTransactionKind,
    TransactionEffects,
    HaneulTransactionResponse,
    TransactionEvents,
} from '@haneullabs/haneul.js';

// const getCoinType = (
//     events: TransactionEvents | null,
//     address: string
// ): string | null => {
//     if (!events) return null;

//     const coinType = events
//         ?.map((event: HaneulEvent) => {
//             const data = Object.values(event).find(
//                 (itm) => itm?.owner?.AddressOwner === address
//             );
//             return data?.coinType;
//         })
//         .filter(Boolean);
//     return coinType?.[0] ? coinType[0] : null;
// };

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
    // TODO: Rebuild this for programmable trnasactions.
    return null;
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
            ['Receive', 'Pay'].includes(event?.content?.changeType)
        ) {
            const coinBalanceChange = getCoinBalanceChangeEvent(event)!;
            const { coinType, amount, owner, sender } = coinBalanceChange;

            const AddressOwner =
                owner !== 'Immutable' && 'AddressOwner' in owner
                    ? owner.AddressOwner
                    : null;

            // ChangeEpoch txn includes coinBalanceChange event for other addresses
            if (
                AddressOwner === address ||
                (address === sender && AddressOwner)
            ) {
                coinsMeta[`${AddressOwner}${coinType}`] = {
                    amount:
                        (coinsMeta[`${AddressOwner}${coinType}`]?.amount || 0) +
                        +amount,
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
    const txnDetails = getTransactionKind(txnData)!;
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
