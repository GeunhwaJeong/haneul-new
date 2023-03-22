// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HANEUL_SYSTEM_STATE_OBJECT_ID, Transaction } from '@haneullabs/haneul.js';

export function createStakeTransaction(amount: bigint, validator: string) {
    const tx = new Transaction();
    const stakeCoin = tx.splitCoins(tx.gas, [tx.pure(amount)]);
    tx.moveCall({
        target: '0x3::haneul_system::request_add_stake',
        arguments: [
            tx.object(HANEUL_SYSTEM_STATE_OBJECT_ID),
            stakeCoin,
            tx.pure(validator),
        ],
    });
    return tx;
}

export function createUnstakeTransaction(stakedHaneulId: string) {
    const tx = new Transaction();
    tx.moveCall({
        target: '0x3::haneul_system::request_withdraw_stake',
        arguments: [
            tx.object(HANEUL_SYSTEM_STATE_OBJECT_ID),
            tx.object(stakedHaneulId),
        ],
    });
    return tx;
}
