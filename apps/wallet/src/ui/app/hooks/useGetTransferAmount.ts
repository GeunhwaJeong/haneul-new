// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HANEUL_TYPE_ARG, getTransactions } from '@haneullabs/haneul.js';
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
    const { certificate, effects } = txn;
    const { coins } = getEventsSummary(effects, activeAddress);

    const haneulTransfer = useMemo(() => {
        const txdetails = getTransactions(certificate)[0];
        return getAmount(txdetails, effects)?.map(
            ({ amount, coinType, recipientAddress }) => {
                return {
                    amount: amount || 0,
                    coinType: coinType || HANEUL_TYPE_ARG,
                    receiverAddress: recipientAddress,
                };
            }
        );
    }, [certificate, effects]);

    const transferAmount = useMemo(() => {
        return haneulTransfer?.length
            ? haneulTransfer
            : coins.filter(
                  ({ receiverAddress }) => receiverAddress === activeAddress
              );
    }, [haneulTransfer, coins, activeAddress]);

    return haneulTransfer ?? transferAmount;
}
