// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Base64DataBuffer } from '../../serialization/base64';
import {
  CallArg,
  ObjectId,
  HaneulAddress,
  HaneulJsonValue,
  TypeTag,
} from '../../types';

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
  /**
   * Usage: pass in HaneulJsonValue[] if you use RpcTxnDataSerializer,
   * Otherwise you need to pass in CallArg[].
   */
  arguments: HaneulJsonValue[] | CallArg[];
  gasPayment?: ObjectId;
  gasBudget: number;
}

export interface PublishTransaction {
  /**
   * Transaction type used for publishing Move modules to the Haneul.
   * Should be already compiled using `haneul-move`, example:
   * ```
   * $ haneul move build
   * $ cat build/project_name/bytecode_modules/module.mv
   * ```
   * In JS:
   *
   * ```
   * // If you are using `RpcTxnDataSerializer`,
   * let file = fs.readFileSync('./move/build/project_name/bytecode_modules/module.mv', 'base64');
   * let compiledModules = [file.toString()]
   *
   * // If you are using `LocalTxnDataSerializer`,
   * let file = fs.readFileSync('./move/build/project_name/bytecode_modules/module.mv');
   * let modules = [ Array.from(file) ];
   *
   * // ... publish logic ...
   * ```
   *
   * Each module should be represented as a sequence of bytes.
   */
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
