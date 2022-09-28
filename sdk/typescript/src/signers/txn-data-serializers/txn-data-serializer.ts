// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Base64DataBuffer } from '../../serialization/base64';
import { ObjectId, HaneulAddress, HaneulJsonValue, TypeTag } from '../../types';

///////////////////////////////
// Exported Types
export interface TransferObjectTransaction {
  objectId: ObjectId;
  gasPayment?: ObjectId;
  gasBudget: number;
  recipient: HaneulAddress;
}

export interface TransferHaneulTransaction {
  haneulObjectId: ObjectId;
  gasBudget: number;
  recipient: HaneulAddress;
  amount: number | null;
}

export interface PayTransaction {
  inputCoins: ObjectId[];
  recipients: HaneulAddress[];
  amounts: number[];
  gasPayment?: ObjectId;
  gasBudget: number;
}

export interface MergeCoinTransaction {
  primaryCoin: ObjectId;
  coinToMerge: ObjectId;
  gasPayment?: ObjectId;
  gasBudget: number;
}

export interface SplitCoinTransaction {
  coinObjectId: ObjectId;
  splitAmounts: number[];
  gasPayment?: ObjectId;
  gasBudget: number;
}

export interface MoveCallTransaction {
  packageObjectId: ObjectId;
  module: string;
  function: string;
  /**
   * Usage: pass in string[] if you use RpcTxnDataSerializer,
   * Otherwise you need to pass in TypeTag[]. We will remove
   * RpcTxnDataSerializer soon.
   */
  typeArguments: string[] | TypeTag[];
  arguments: HaneulJsonValue[];
  gasPayment?: ObjectId;
  gasBudget: number;
}

/**
 * Transaction type used for publishing Move modules to the Haneul.
 *
 * Use the util methods defined in [utils/publish.ts](../../utils/publish.ts)
 * to get `compiledModules` bytes by leveraging the haneul
 * command line tool.
 *
 * ```
 * const { execSync } = require('child_process');
 * const modulesInBase64 = JSON.parse(execSync(
 *   `${cliPath} move build --dump-bytecode-as-base64 --path ${packagePath}`,
 *   { encoding: 'utf-8' }
 * ));
 *
 * // Include the following line if you are using `LocalTxnDataSerializer`, skip
 * // if you are using `RpcTxnDataSerializer`
 * // const modulesInBytes = modules.map((m) => Array.from(new Base64DataBuffer(m).getData()));
 * // ... publish logic ...
 * ```
 *
 */
export interface PublishTransaction {
  compiledModules: ArrayLike<string> | ArrayLike<ArrayLike<number>>;
  gasPayment?: ObjectId;
  gasBudget: number;
}

///////////////////////////////
// Exported Abstracts
/**
 * Serializes a transaction to a string that can be signed by a `Signer`.
 */
export interface TxnDataSerializer {
  newTransferObject(
    signerAddress: HaneulAddress,
    txn: TransferObjectTransaction
  ): Promise<Base64DataBuffer>;

  newTransferHaneul(
    signerAddress: HaneulAddress,
    txn: TransferHaneulTransaction
  ): Promise<Base64DataBuffer>;

  newPay(
    signerAddress: HaneulAddress,
    txn: PayTransaction
  ): Promise<Base64DataBuffer>;

  newMoveCall(
    signerAddress: HaneulAddress,
    txn: MoveCallTransaction
  ): Promise<Base64DataBuffer>;

  newMergeCoin(
    signerAddress: HaneulAddress,
    txn: MergeCoinTransaction
  ): Promise<Base64DataBuffer>;

  newSplitCoin(
    signerAddress: HaneulAddress,
    txn: SplitCoinTransaction
  ): Promise<Base64DataBuffer>;

  newPublish(
    signerAddress: HaneulAddress,
    txn: PublishTransaction
  ): Promise<Base64DataBuffer>;
}
