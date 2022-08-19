// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isTransactionBytes } from '../../index.guard';
import { JsonRpcClient } from '../../rpc/client';
import { Base64DataBuffer } from '../../serialization/base64';
import { HaneulAddress } from '../../types';
import {
  MoveCallTransaction,
  MergeCoinTransaction,
  SplitCoinTransaction,
  TransferObjectTransaction,
  TransferHaneulTransaction,
  PublishTransaction,
  TxnDataSerializer,
} from './txn-data-serializer';

/**
 * This is a temporary implementation of the `TxnDataSerializer` class
 * that uses the Haneul Gateway RPC API to serialize a transaction into BCS bytes.
 * This class will be deprecated once we support BCS serialization in TypeScript.
 * It is not safe to use this class in production because one cannot authenticate
 * the encoding.
 */
export class RpcTxnDataSerializer implements TxnDataSerializer {
  private client: JsonRpcClient;

  /**
   * Establish a connection to a Haneul RPC endpoint
   *
   * @param endpoint URL to the Haneul RPC endpoint
   * @param skipDataValidation default to `false`. If set to `true`, the rpc
   * client will not check if the responses from the RPC server conform to the schema
   * defined in the TypeScript SDK. The mismatches often happen when the SDK
   * is in a different version than the RPC server. Skipping the validation
   * can maximize the version compatibility of the SDK, as not all the schema
   * changes in the RPC response will affect the caller, but the caller needs to
   * understand that the data may not match the TypeSrcript definitions.
   */
  constructor(endpoint: string, private skipDataValidation: boolean = false) {
    this.client = new JsonRpcClient(endpoint);
  }

  async newTransferObject(
    signerAddress: HaneulAddress,
    t: TransferObjectTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_transferObject',
        [signerAddress, t.objectId, t.gasPayment, t.gasBudget, t.recipient],
        isTransactionBytes,
        this.skipDataValidation
      );
      return new Base64DataBuffer(resp.txBytes);
    } catch (err) {
      throw new Error(`Error transferring object: ${err} with args ${t}`);
    }
  }

  async newTransferHaneul(
    signerAddress: HaneulAddress,
    t: TransferHaneulTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_transferHaneul',
        [signerAddress, t.haneulObjectId, t.gasBudget, t.recipient, t.amount],
        isTransactionBytes,
        this.skipDataValidation
      );
      return new Base64DataBuffer(resp.txBytes);
    } catch (err) {
      throw new Error(`Error transferring Haneul coin: ${err} with args ${t}`);
    }
  }

  async newMoveCall(
    signerAddress: HaneulAddress,
    t: MoveCallTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_moveCall',
        [
          signerAddress,
          t.packageObjectId,
          t.module,
          t.function,
          t.typeArguments,
          t.arguments,
          t.gasPayment,
          t.gasBudget,
        ],
        isTransactionBytes,
        this.skipDataValidation
      );
      return new Base64DataBuffer(resp.txBytes);
    } catch (err) {
      throw new Error(`Error executing a move call: ${err} with args ${t}`);
    }
  }

  async newMergeCoin(
    signerAddress: HaneulAddress,
    t: MergeCoinTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_mergeCoins',
        [
          signerAddress,
          t.primaryCoin,
          t.coinToMerge,
          t.gasPayment,
          t.gasBudget,
        ],
        isTransactionBytes,
        this.skipDataValidation
      );
      return new Base64DataBuffer(resp.txBytes);
    } catch (err) {
      throw new Error(`Error merging coin: ${err}`);
    }
  }

  async newSplitCoin(
    signerAddress: HaneulAddress,
    t: SplitCoinTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_splitCoin',
        [
          signerAddress,
          t.coinObjectId,
          t.splitAmounts,
          t.gasPayment,
          t.gasBudget,
        ],
        isTransactionBytes,
        this.skipDataValidation
      );
      return new Base64DataBuffer(resp.txBytes);
    } catch (err) {
      throw new Error(`Error splitting coin: ${err}`);
    }
  }

  async newPublish(
    signerAddress: HaneulAddress,
    t: PublishTransaction
  ): Promise<Base64DataBuffer> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_publish',
        [signerAddress, t.compiledModules, t.gasPayment, t.gasBudget],
        isTransactionBytes,
        this.skipDataValidation
      );
      return new Base64DataBuffer(resp.txBytes);
    } catch (err) {
      throw new Error(`Error publishing package ${err}`);
    }
  }
}
