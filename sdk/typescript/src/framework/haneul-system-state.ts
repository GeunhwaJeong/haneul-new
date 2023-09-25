// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { TransactionBlock } from '../builder/index.js';
import type { HaneulClient } from '../client/index.js';
import type { JsonRpcProvider } from '../providers/json-rpc-provider.js';
import { getObjectReference } from '../types/index.js';
import { normalizeHaneulObjectId } from '../utils/haneul-types.js';
import { HANEUL_SYSTEM_ADDRESS } from './framework.js';

/**
 * Address of the Haneul System object.
 * Always the same in every Haneul network (local, devnet, testnet).
 */
export const HANEUL_SYSTEM_STATE_OBJECT_ID: string = normalizeHaneulObjectId('0x5');

export const HANEUL_SYSTEM_MODULE_NAME = 'haneul_system';
export const ADD_STAKE_FUN_NAME = 'request_add_stake';
export const ADD_STAKE_LOCKED_COIN_FUN_NAME = 'request_add_stake_with_locked_coin';
export const WITHDRAW_STAKE_FUN_NAME = 'request_withdraw_stake';

/**
 * Utility class for `0x5` object
 */
export class HaneulSystemStateUtil {
	/**
	 * Create a new transaction for staking coins ready to be signed and executed with `signer-and-provider`.
	 *
	 * @param coins the coins to be staked
	 * @param amount the amount to stake
	 * @param gasBudget omittable only for DevInspect mode
	 */
	public static async newRequestAddStakeTxn(
		client: JsonRpcProvider | HaneulClient,
		coins: string[],
		amount: bigint,
		validatorAddress: string,
	): Promise<TransactionBlock> {
		// TODO: validate coin types and handle locked coins
		const tx = new TransactionBlock();

		const coin = tx.splitCoins(tx.gas, [tx.pure(amount)]);
		tx.moveCall({
			target: `${HANEUL_SYSTEM_ADDRESS}::${HANEUL_SYSTEM_MODULE_NAME}::${ADD_STAKE_FUN_NAME}`,
			arguments: [tx.object(HANEUL_SYSTEM_STATE_OBJECT_ID), coin, tx.pure(validatorAddress)],
		});
		const coinObjects = await client.multiGetObjects({
			ids: coins,
			options: {
				showOwner: true,
			},
		});
		tx.setGasPayment(coinObjects.map((obj) => getObjectReference(obj)!));
		return tx;
	}

	/**
	 * Create a new transaction for withdrawing coins ready to be signed and
	 * executed with `signer-and-provider`.
	 *
	 * @param stake the stake object created in the requestAddStake txn
	 * @param stakedCoinId the coins to withdraw
	 * @param gasBudget omittable only for DevInspect mode
	 */
	public static async newRequestWithdrawlStakeTxn(
		stake: string,
		stakedCoinId: string,
	): Promise<TransactionBlock> {
		const tx = new TransactionBlock();
		tx.moveCall({
			target: `${HANEUL_SYSTEM_ADDRESS}::${HANEUL_SYSTEM_MODULE_NAME}::${WITHDRAW_STAKE_FUN_NAME}`,
			arguments: [tx.object(HANEUL_SYSTEM_STATE_OBJECT_ID), tx.object(stake), tx.object(stakedCoinId)],
		});

		return tx;
	}
}
