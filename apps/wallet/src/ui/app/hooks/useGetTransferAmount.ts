// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HANEUL_TYPE_ARG, getTransactionKinds } from '@haneullabs/haneul.js';
import { useMemo } from 'react';

import { getEventsSummary, getAmount } from '_helpers';

import type { HaneulTransactionResponse, HaneulAddress } from '@haneullabs/haneul.js';

export function useGetTransferAmount({
    txn,
    activeAddress,
}: {
    txn: HaneulTransactionResponse;
    activeAddress: HaneulAddress;
}) {
    const { effects, events } = txn;
    const { coins } = getEventsSummary(events!, activeAddress);

    const haneulTransfer = useMemo(() => {
        const txdetails = getTransactionKinds(txn)![0];
        return getAmount(txdetails, effects!, events!)?.map(
            ({ amount, coinType, recipientAddress }) => {
                return {
                    amount: amount || 0,
                    coinType: coinType || HANEUL_TYPE_ARG,
                    receiverAddress: recipientAddress,
                };
            }
        );
    }, [txn, effects, events]);

    const transferAmount = useMemo(() => {
        return haneulTransfer?.length
            ? haneulTransfer
            : coins.filter(
                  ({ receiverAddress }) => receiverAddress === activeAddress
              );
    }, [haneulTransfer, coins, activeAddress]);

    return haneulTransfer ?? transferAmount;
}
