// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    getTransactionKindName,
    getMoveCallTransaction,
    getTransactionKinds,
} from '@haneullabs/haneul.js';

import type { HaneulTransactionResponse } from '@haneullabs/haneul.js';

export function checkStakingTxn(txn: HaneulTransactionResponse) {
    const [transaction] = getTransactionKinds(txn)!;
    const txnKind = getTransactionKindName(transaction);

    if (txnKind !== 'Call') return null;

    const moveCallTxn = getMoveCallTransaction(transaction);
    if (
        moveCallTxn?.module === 'haneul_system' &&
        moveCallTxn?.function === 'request_add_delegation_mul_coin'
    )
        return 'Staked';
    if (
        moveCallTxn?.module === 'haneul_system' &&
        moveCallTxn?.function === 'request_withdraw_delegation'
    )
        return 'Unstaked';
    return null;
}
