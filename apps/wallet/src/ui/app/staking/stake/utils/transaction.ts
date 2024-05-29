// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Transaction } from '@haneullabs/haneul/transactions';
import { HANEUL_SYSTEM_STATE_OBJECT_ID } from '@haneullabs/haneul/utils';

export function createStakeTransaction(amount: bigint, validator: string) {
	const tx = new Transaction();
	const stakeCoin = tx.splitCoins(tx.gas, [amount]);
	tx.moveCall({
		target: '0x3::haneul_system::request_add_stake',
		arguments: [
			tx.sharedObjectRef({
				objectId: HANEUL_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: true,
			}),
			stakeCoin,
			tx.pure.address(validator),
		],
	});
	return tx;
}

export function createUnstakeTransaction(stakedHaneulId: string) {
	const tx = new Transaction();
	tx.moveCall({
		target: '0x3::haneul_system::request_withdraw_stake',
		arguments: [tx.object(HANEUL_SYSTEM_STATE_OBJECT_ID), tx.object(stakedHaneulId)],
	});
	return tx;
}
