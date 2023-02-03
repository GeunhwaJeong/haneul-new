// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HANEUL_TYPE_ARG } from '@haneullabs/haneul.js';

import { TxnAmount } from '_components/receipt-card/TxnAmount';

import type { HaneulTransactionResponse } from '@haneullabs/haneul.js';

type StakeTxnCardProps = {
    txn: HaneulTransactionResponse;
    amount: number;
    activeAddress: string;
};

// TODO For unstake Transaction there is no reliable way to get the validator address, reward
// For now show the amount
export function UnStakeTxnCard({ amount }: StakeTxnCardProps) {
    return (
        <div className="flex flex-col w-full items-center divide-y divide-solid divide-steel/20 divide-x-0 gap-3.5">
            {amount && (
                <TxnAmount
                    amount={amount}
                    coinType={HANEUL_TYPE_ARG}
                    label="Unstake"
                />
            )}
        </div>
    );
}
