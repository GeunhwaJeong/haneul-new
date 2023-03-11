// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Commands, Transaction } from '../builder';
import { Provider } from '../providers/provider';
import {
  getObjectReference,
  normalizeHaneulObjectId,
  ObjectId,
  HaneulAddress,
  HANEUL_FRAMEWORK_ADDRESS,
} from '../types';

/**
 * Address of the Haneul System object.
 * Always the same in every Haneul network (local, devnet, testnet).
 */
export const HANEUL_SYSTEM_STATE_OBJECT_ID: string = normalizeHaneulObjectId('0x5');

export const HANEUL_SYSTEM_MODULE_NAME = 'haneul_system';
export const ADD_STAKE_FUN_NAME = 'request_add_stake';
export const ADD_STAKE_LOCKED_COIN_FUN_NAME =
  'request_add_stake_with_locked_coin';
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
    provider: Provider,
    coins: ObjectId[],
    amount: bigint,
    validatorAddress: HaneulAddress,
  ): Promise<Transaction> {
    // TODO: validate coin types and handle locked coins
    const tx = new Transaction();
    const coin = tx.add(Commands.SplitCoin(tx.gas, tx.pure(amount)));
    tx.add(
      Commands.MoveCall({
        target: `${HANEUL_FRAMEWORK_ADDRESS}::${HANEUL_SYSTEM_MODULE_NAME}::${ADD_STAKE_FUN_NAME}`,
        arguments: [
          tx.object(HANEUL_SYSTEM_STATE_OBJECT_ID),
          coin,
          tx.pure(validatorAddress),
        ],
      }),
    );
    const coinObjects = await provider.getObjectBatch(coins, {
      showOwner: true,
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
    stake: ObjectId,
    stakedCoinId: ObjectId,
  ): Promise<Transaction> {
    const tx = new Transaction();
    tx.add(
      Commands.MoveCall({
        target: `${HANEUL_FRAMEWORK_ADDRESS}::${HANEUL_SYSTEM_MODULE_NAME}::${WITHDRAW_STAKE_FUN_NAME}`,
        arguments: [
          tx.object(HANEUL_SYSTEM_STATE_OBJECT_ID),
          tx.object(stake),
          tx.object(stakedCoinId),
        ],
      }),
    );
    return tx;
  }
}
