// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { UnserializedSignableTransaction } from '../signers/txn-data-serializers/txn-data-serializer';
import {
  normalizeHaneulAddress,
  ObjectId,
  HaneulAddress,
  HANEUL_FRAMEWORK_ADDRESS,
} from '../types';

/**
 * Address of the Haneul System object.
 * Always the same in every Haneul network (local, devnet, testnet).
 */
export const HANEUL_SYSTEM_STATE_OBJECT_ID: string = normalizeHaneulAddress('0x5');

export const HANEUL_SYSTEM_MODULE_NAME = 'haneul_system';
export const ADD_DELEGATION_MUL_COIN_FUN_NAME =
  'request_add_delegation_mul_coin';
export const ADD_DELEGATION_LOCKED_COIN_FUN_NAME =
  'request_add_delegation_mul_locked_coin';
export const WITHDRAW_DELEGATION_FUN_NAME = 'request_withdraw_delegation';

/**
 * Utility class for `0x5` object
 */
export class HaneulSystemStateUtil {
  /**
   * Create a new transaction for delegating coins ready to be signed and executed with `signer-and-provider`.
   *
   * @param coins the coins to be used in delegation
   * @param amount the amount to delegate
   * @param gasBudget omittable only for DevInspect mode
   */
  public static async newRequestAddDelegationTxn(
    coins: ObjectId[],
    amount: bigint,
    validatorAddress: HaneulAddress,
    gasBudget?: number,
    gasPayment?: ObjectId,
    gasPrice?: number,
  ): Promise<UnserializedSignableTransaction> {
    // TODO: validate coin types and handle locked coins
    return {
      kind: 'moveCall',
      data: {
        packageObjectId: HANEUL_FRAMEWORK_ADDRESS,
        module: HANEUL_SYSTEM_MODULE_NAME,
        function: ADD_DELEGATION_MUL_COIN_FUN_NAME,
        typeArguments: [],
        arguments: [
          HANEUL_SYSTEM_STATE_OBJECT_ID,
          coins,
          [String(amount)],
          validatorAddress,
        ],
        gasBudget,
        gasPayment,
        gasPrice,
      },
    };
  }

  /**
   * Create a new transaction for withdrawing coins ready to be signed and
   * executed with `signer-and-provider`.
   *
   * @param delegation the delegation object created in the requestAddDelegation txn
   * @param stakedCoinId the coins to withdraw
   * @param gasBudget omittable only for DevInspect mode
   */
  public static async newRequestWithdrawlDelegationTxn(
    delegation: ObjectId,
    stakedCoinId: ObjectId,
    gasBudget?: number,
    gasPayment?: ObjectId,
    gasPrice?: number,
  ): Promise<UnserializedSignableTransaction> {
    return {
      kind: 'moveCall',
      data: {
        packageObjectId: HANEUL_FRAMEWORK_ADDRESS,
        module: HANEUL_SYSTEM_MODULE_NAME,
        function: WITHDRAW_DELEGATION_FUN_NAME,
        typeArguments: [],
        arguments: [HANEUL_SYSTEM_STATE_OBJECT_ID, delegation, stakedCoinId],
        gasBudget,
        gasPayment,
        gasPrice,
      },
    };
  }
}
