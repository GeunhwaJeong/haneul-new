// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { SerializedSignature } from '../../cryptography/signature';
import {
  ObjectId,
  PureArg,
  HaneulAddress,
  HaneulJsonValue,
  TypeTag,
} from '../../types';

///////////////////////////////
// Exported Types
export interface TransactionCommon {
  /* This field is required for regular transaction but can be omitted for devinspect transaction */
  gasBudget?: number;
  /* If omitted, reference gas price fetched from the connected fullnode will be used */
  gasPrice?: number;
}

export interface TransferObjectTransaction extends TransactionCommon {
  objectId: ObjectId;
  recipient: HaneulAddress;
  gasPayment?: ObjectId;
  gasOwner?: HaneulAddress;
}

export interface TransferHaneulTransaction extends TransactionCommon {
  haneulObjectId: ObjectId;
  recipient: HaneulAddress;
  amount: number | null;
}

/// Send Coin<T> to a list of addresses, where `T` can be any coin type, following a list of amounts,
/// The object specified in the `gas` field will be used to pay the gas fee for the transaction.
/// The gas object can not appear in `input_coins`. If the gas object is not specified, the RPC server
/// will auto-select one.
export interface PayTransaction extends TransactionCommon {
  /**
   * use `provider.selectCoinSetWithCombinedBalanceGreaterThanOrEqual` to
   * derive a minimal set of coins with combined balance greater than or
   * equal to sent amounts
   */
  inputCoins: ObjectId[];
  recipients: HaneulAddress[];
  amounts: number[];
  gasPayment?: ObjectId;
  gasOwner?: HaneulAddress;
}

/// Send HANEUL coins to a list of addresses, following a list of amounts.
/// This is for HANEUL coin only and does not require a separate gas coin object.
/// Specifically, what pay_haneul does are:
/// 1. debit each input_coin to create new coin following the order of
/// amounts and assign it to the corresponding recipient.
/// 2. accumulate all residual HANEUL from input coins left and deposit all HANEUL to the first
/// input coin, then use the first input coin as the gas coin object.
/// 3. the balance of the first input coin after tx is sum(input_coins) - sum(amounts) - actual_gas_cost
/// 4. all other input coins other than the first one are deleted.
export interface PayHaneulTransaction extends TransactionCommon {
  /**
   * use `provider.selectCoinSetWithCombinedBalanceGreaterThanOrEqual` to
   * derive a minimal set of coins with combined balance greater than or
   * equal to (sent amounts + gas budget).
   */
  inputCoins: ObjectId[];
  recipients: HaneulAddress[];
  amounts: number[];
}

/// Send all HANEUL coins to one recipient.
/// This is for HANEUL coin only and does not require a separate gas coin object.
/// Specifically, what pay_all_haneul does are:
/// 1. accumulate all HANEUL from input coins and deposit all HANEUL to the first input coin
/// 2. transfer the updated first coin to the recipient and also use this first coin as gas coin object.
/// 3. the balance of the first input coin after tx is sum(input_coins) - actual_gas_cost.
/// 4. all other input coins other than the first are deleted.
export interface PayAllHaneulTransaction extends TransactionCommon {
  inputCoins: ObjectId[];
  recipient: HaneulAddress;
}

export interface MergeCoinTransaction extends TransactionCommon {
  primaryCoin: ObjectId;
  coinToMerge: ObjectId;
  gasPayment?: ObjectId;
  gasOwner?: HaneulAddress;
}

export interface SplitCoinTransaction extends TransactionCommon {
  coinObjectId: ObjectId;
  splitAmounts: number[];
  gasPayment?: ObjectId;
  gasOwner?: HaneulAddress;
}

export interface MoveCallTransaction extends TransactionCommon {
  packageObjectId: ObjectId;
  module: string;
  function: string;
  typeArguments: string[] | TypeTag[];
  arguments: (HaneulJsonValue | PureArg)[];
  gasPayment?: ObjectId;
  gasOwner?: HaneulAddress;
}

export interface RawMoveCall {
  packageObjectId: ObjectId;
  module: string;
  function: string;
  typeArguments: string[];
  arguments: HaneulJsonValue[];
}

/** @deprecated Use `Transaction` class. */
export type UnserializedSignableTransaction =
  | {
      kind: 'moveCall';
      data: MoveCallTransaction;
    }
  | {
      kind: 'transferHaneul';
      data: TransferHaneulTransaction;
    }
  | {
      kind: 'transferObject';
      data: TransferObjectTransaction;
    }
  | {
      kind: 'mergeCoin';
      data: MergeCoinTransaction;
    }
  | {
      kind: 'splitCoin';
      data: SplitCoinTransaction;
    }
  | {
      kind: 'pay';
      data: PayTransaction;
    }
  | {
      kind: 'payHaneul';
      data: PayHaneulTransaction;
    }
  | {
      kind: 'payAllHaneul';
      data: PayAllHaneulTransaction;
    }
  | {
      kind: 'publish';
      data: PublishTransaction;
    };

export type SignedTransaction = {
  transactionBytes: string;
  signature: SerializedSignature;
};

export type SignedMessage = {
  messageBytes: string;
  signature: SerializedSignature;
};

/**
 * A type that represents the possible transactions that can be signed:
 * @deprecated Use `Transaction` instead.
 */
export type SignableTransaction =
  | UnserializedSignableTransaction
  | {
      kind: 'bytes';
      data: Uint8Array;
    };

export type SignableTransactionKind = SignableTransaction['kind'];
export type SignableTransactionData = SignableTransaction['data'];

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
 * // const modulesInBytes = modules.map((m) => Array.from(fromB64(m)));
 * // ... publish logic ...
 * ```
 *
 */
export interface PublishTransaction extends TransactionCommon {
  compiledModules: ArrayLike<string> | ArrayLike<ArrayLike<number>>;
  gasPayment?: ObjectId;
  gasOwner?: HaneulAddress;
}

export type TransactionBuilderMode = 'Commit' | 'DevInspect';

///////////////////////////////
// Exported Abstracts
/**
 * Serializes a transaction to a string that can be signed by a `Signer`.
 */
export interface TxnDataSerializer {
  serializeToBytes(
    signerAddress: HaneulAddress,
    txn: UnserializedSignableTransaction,
    mode: TransactionBuilderMode,
  ): Promise<Uint8Array>;
}
