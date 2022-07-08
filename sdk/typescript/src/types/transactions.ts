// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ObjectOwner, HaneulAddress, TransactionDigest } from './common';
import { HaneulMovePackage, HaneulObject, HaneulObjectRef } from './objects';

export type TransferObject = {
  recipient: HaneulAddress;
  objectRef: HaneulObjectRef;
};

export type TransactionKindName = 'TransferObject' | 'Publish' | 'Call';
export type HaneulTransactionKind =
  | { TransferObject: TransferObject }
  | { Publish: HaneulMovePackage }
  | { Call: MoveCall };
export type TransactionData = {
  transactions: HaneulTransactionKind[];
  sender: HaneulAddress;
  gasPayment: HaneulObjectRef;
  gasBudget: number;
};

// TODO: support u64
export type EpochId = number;

export type AuthorityQuorumSignInfo = {
  epoch: EpochId;
  signatures: AuthoritySignature[];
};

export type CertifiedTransaction = {
  transactionDigest: TransactionDigest;
  data: TransactionData;
  txSignature: string;
  authSignInfo: AuthorityQuorumSignInfo;
};

export type GasCostSummary = {
  computationCost: number;
  storageCost: number;
  storageRebate: number;
};

export type ExecutionStatusType = 'success' | 'failure';
export type ExecutionStatus = {
  status: ExecutionStatusType;
  error?: string;
};

// TODO: change the tuple to struct from the server end
export type OwnedObjectRef = {
  owner: ObjectOwner;
  reference: HaneulObjectRef;
};

export type TransactionEffects = {
  /** The status of the execution */
  status: ExecutionStatus;
  gasUsed: GasCostSummary;
  /** The object references of the shared objects used in this transaction. Empty if no shared objects were used. */
  sharedObjects?: HaneulObjectRef[];
  /** The transaction digest */
  transactionDigest: TransactionDigest;
  /** ObjectRef and owner of new objects created */
  created?: OwnedObjectRef[];
  /** ObjectRef and owner of mutated objects, including gas object */
  mutated?: OwnedObjectRef[];
  /**
   * ObjectRef and owner of objects that are unwrapped in this transaction.
   * Unwrapped objects are objects that were wrapped into other objects in the past,
   * and just got extracted out.
   */
  unwrapped?: OwnedObjectRef[];
  /** Object Refs of objects now deleted (the old refs) */
  deleted?: HaneulObjectRef[];
  /** Object refs of objects now wrapped in other objects */
  wrapped?: HaneulObjectRef[];
  /**
   * The updated gas object reference. Have a dedicated field for convenient access.
   * It's also included in mutated.
   */
  gasObject: OwnedObjectRef;
  /** The events emitted during execution. Note that only successful transactions emit events */
  // TODO: properly define type when this is being used
  events?: any[];
  /** The set of transaction digests this transaction depends on */
  dependencies?: TransactionDigest[];
};

export type TransactionEffectsResponse = {
  certificate: CertifiedTransaction;
  effects: TransactionEffects;
  timestamp_ms: number | null;
};

export type GatewayTxSeqNumber = number;

export type GetTxnDigestsResponse = [GatewayTxSeqNumber, TransactionDigest][];

export type MoveCall = {
  package: HaneulObjectRef;
  module: string;
  function: string;
  typeArguments?: string[];
  arguments?: HaneulJsonValue[];
};

export type HaneulJsonValue =
  | boolean
  | number
  | string
  | Array<boolean | number | string>;

export type EmptySignInfo = object;
export type AuthorityName = string;
export type AuthoritySignature = string;

export type TransactionBytes = {
  txBytes: string;
  gas: HaneulObjectRef;
  // TODO: Add input_objects field
};

export type MergeCoinResponse = {
  certificate: CertifiedTransaction;
  updatedCoin: HaneulObject;
  updatedGas: HaneulObject;
};

export type SplitCoinResponse = {
  certificate: CertifiedTransaction;
  updatedCoin: HaneulObject;
  newCoins: HaneulObject[];
  updatedGas: HaneulObject;
};

export type TransactionResponse =
  | {
      EffectResponse: TransactionEffectsResponse;
      // TODO: Add Publish Response
    }
  | {
      SplitCoinResponse: SplitCoinResponse;
    }
  | {
      MergeCoinResponse: MergeCoinResponse;
    };

/* -------------------------------------------------------------------------- */
/*                              Helper functions                              */
/* -------------------------------------------------------------------------- */

/* ---------------------------------- CertifiedTransaction --------------------------------- */
export function getTransactionDigest(
  tx: CertifiedTransaction
): TransactionDigest {
  return tx.transactionDigest;
}

export function getTransactionSignature(tx: CertifiedTransaction): string {
  return tx.txSignature;
}

export function getTransactionAuthorityQuorumSignInfo(
  tx: CertifiedTransaction
): AuthorityQuorumSignInfo {
  return tx.authSignInfo;
}

export function getTransactionData(tx: CertifiedTransaction): TransactionData {
  return tx.data;
}

/* ----------------------------- TransactionData ---------------------------- */

export function getTransactionSender(tx: CertifiedTransaction): HaneulAddress {
  return tx.data.sender;
}

export function getTransactionGasObject(
  tx: CertifiedTransaction
): HaneulObjectRef {
  return tx.data.gasPayment;
}

export function getTransactionGasBudget(tx: CertifiedTransaction): number {
  return tx.data.gasBudget;
}

export function getTransferObjectTransaction(
  data: HaneulTransactionKind
): TransferObject | undefined {
  return 'TransferObject' in data ? data.TransferObject : undefined;
}

export function getPublishTransaction(
  data: HaneulTransactionKind
): HaneulMovePackage | undefined {
  return 'Publish' in data ? data.Publish : undefined;
}

export function getMoveCallTransaction(
  data: HaneulTransactionKind
): MoveCall | undefined {
  return 'Call' in data ? data.Call : undefined;
}

export function getTransactions(
  data: CertifiedTransaction
): HaneulTransactionKind[] {
  return data.data.transactions;
}

export function getTransactionKindName(
  data: HaneulTransactionKind
): TransactionKindName {
  return Object.keys(data)[0] as TransactionKindName;
}

/* ----------------------------- ExecutionStatus ---------------------------- */

export function getExecutionStatusType(
  data: TransactionEffectsResponse
): ExecutionStatusType {
  return getExecutionStatus(data).status;
}

export function getExecutionStatus(
  data: TransactionEffectsResponse
): ExecutionStatus {
  return data.effects.status;
}

export function getExecutionStatusError(
  data: TransactionEffectsResponse
): string | undefined {
  return getExecutionStatus(data).error;
}

export function getExecutionStatusGasSummary(
  data: TransactionEffectsResponse
): GasCostSummary {
  return data.effects.gasUsed;
}

export function getTotalGasUsed(data: TransactionEffectsResponse): number {
  const gasSummary = getExecutionStatusGasSummary(data);
  return (
    gasSummary.computationCost +
    gasSummary.storageCost -
    gasSummary.storageRebate
  );
}

/* --------------------------- TransactionResponse -------------------------- */

export function getTransactionEffectsResponse(
  data: TransactionResponse
): TransactionEffectsResponse | undefined {
  return 'EffectResponse' in data ? data.EffectResponse : undefined;
}

export function getSplitCoinResponse(
  data: TransactionResponse
): SplitCoinResponse | undefined {
  return 'SplitCoinResponse' in data ? data.SplitCoinResponse : undefined;
}

export function getMergeCoinResponse(
  data: TransactionResponse
): MergeCoinResponse | undefined {
  return 'MergeCoinResponse' in data ? data.MergeCoinResponse : undefined;
}

/**
 * Get the updated coin after a merge.
 * @param data the response for executing a merge coin transaction
 * @returns the updated state of the primary coin after the merge
 */
export function getCoinAfterMerge(
  data: TransactionResponse
): HaneulObject | undefined {
  return getMergeCoinResponse(data)?.updatedCoin;
}

/**
 * Get the updated coin after a split.
 * @param data the response for executing a Split coin transaction
 * @returns the updated state of the original coin object used for the split
 */
export function getCoinAfterSplit(
  data: TransactionResponse
): HaneulObject | undefined {
  return getSplitCoinResponse(data)?.updatedCoin;
}

/**
 * Get the newly created coin after a split.
 * @param data the response for executing a Split coin transaction
 * @returns the updated state of the original coin object used for the split
 */
export function getNewlyCreatedCoinsAfterSplit(
  data: TransactionResponse
): HaneulObject[] | undefined {
  return getSplitCoinResponse(data)?.newCoins;
}
