// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ObjectOwner, HaneulAddress, TransactionDigest } from './common';
import { ObjectId, HaneulMovePackage, HaneulObject, HaneulObjectRef } from './objects';

export type TransferObject = {
  recipient: HaneulAddress;
  objectRef: HaneulObjectRef;
};

export type HaneulTransferHaneul = {
  recipient: HaneulAddress;
  amount: number | null;
};

export type HaneulChangeEpoch = {
  epoch: EpochId;
  storage_charge: number;
  computation_charge: number;
};

export type Pay = {
  coins: HaneulObjectRef[];
  recipients: HaneulAddress[];
  amounts: number[];
};

export type ExecuteTransactionRequestType =
  | 'ImmediateReturn'
  | 'WaitForTxCert'
  | 'WaitForEffectsCert'
  | 'WaitForLocalExecution';

export type TransactionKindName =
  | 'TransferObject'
  | 'Publish'
  | 'Call'
  | 'TransferHaneul'
  | 'ChangeEpoch'
  | 'Pay';

export type HaneulTransactionKind =
  | { TransferObject: TransferObject }
  | { Publish: HaneulMovePackage }
  | { Call: MoveCall }
  | { TransferHaneul: HaneulTransferHaneul }
  | { ChangeEpoch: HaneulChangeEpoch }
  | { Pay: Pay };
export type HaneulTransactionData = {
  transactions: HaneulTransactionKind[];
  sender: HaneulAddress;
  gasPayment: HaneulObjectRef;
  gasBudget: number;
};

// TODO: support u64
export type EpochId = number;
export type GenericAuthoritySignature =
  | AuthoritySignature[]
  | AuthoritySignature;

export type AuthorityQuorumSignInfo = {
  epoch: EpochId;
  signature: GenericAuthoritySignature;
};

export type CertifiedTransaction = {
  transactionDigest: TransactionDigest;
  data: HaneulTransactionData;
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

export type HaneulTransactionResponse = {
  certificate: CertifiedTransaction;
  effects: TransactionEffects;
  timestamp_ms: number | null;
  parsed_data: HaneulParsedTransactionResponse | null;
};

// TODO: this is likely to go away after https://github.com/GeunhwaJeong/haneul/issues/4207
export type HaneulCertifiedTransactionEffects = {
  effects: TransactionEffects;
};

export type HaneulExecuteTransactionResponse =
  | {
      ImmediateReturn: {
        tx_digest: string;
      };
    }
  | { TxCert: { certificate: CertifiedTransaction } }
  | {
      EffectsCert: {
        certificate: CertifiedTransaction;
        effects: HaneulCertifiedTransactionEffects;
      };
    };

export type GatewayTxSeqNumber = number;

export type GetTxnDigestsResponse = TransactionDigest[];
// TODO: remove after we deploy 0.12.0 DevNet
export type GetTxnDigestsResponse__DEPRECATED = [
  GatewayTxSeqNumber,
  TransactionDigest
][];

export type PaginatedTransactionDigests = {
  data: TransactionDigest[];
  nextCursor: TransactionDigest | null;
};

export type TransactionQuery =
  | 'All'
  | {
      MoveFunction: {
        package: ObjectId;
        module: string | null;
        function: string | null;
      };
    }
  | { InputObject: ObjectId }
  | { MutatedObject: ObjectId }
  | { FromAddress: HaneulAddress }
  | { ToAddress: HaneulAddress };

export type Ordering = 'Ascending' | 'Descending';

export type MoveCall = {
  package: HaneulObjectRef;
  module: string;
  function: string;
  typeArguments?: string[];
  arguments?: HaneulJsonValue[];
};

export type HaneulJsonValue = boolean | number | string | Array<HaneulJsonValue>;

export type EmptySignInfo = object;
export type AuthorityName = string;
export type AuthoritySignature = string;

export type TransactionBytes = {
  txBytes: string;
  gas: HaneulObjectRef;
  // TODO: Add input_objects field
};

export type HaneulParsedMergeCoinResponse = {
  updatedCoin: HaneulObject;
  updatedGas: HaneulObject;
};

export type HaneulParsedSplitCoinResponse = {
  updatedCoin: HaneulObject;
  newCoins: HaneulObject[];
  updatedGas: HaneulObject;
};

export type HaneulParsedPublishResponse = {
  createdObjects: HaneulObject[];
  package: HaneulPackage;
  updatedGas: HaneulObject;
};

export type HaneulPackage = {
  digest: string;
  objectId: string;
  version: number;
};

export type HaneulParsedTransactionResponse =
  | {
      SplitCoin: HaneulParsedSplitCoinResponse;
    }
  | {
      MergeCoin: HaneulParsedMergeCoinResponse;
    }
  | {
      Publish: HaneulParsedPublishResponse;
    };

/* -------------------------------------------------------------------------- */
/*                              Helper functions                              */
/* -------------------------------------------------------------------------- */

/* ---------------------------------- CertifiedTransaction --------------------------------- */

export function getCertifiedTransaction(
  tx: HaneulTransactionResponse | HaneulExecuteTransactionResponse
): CertifiedTransaction | undefined {
  if ('certificate' in tx) {
    return tx.certificate;
  } else if ('TxCert' in tx) {
    return tx.TxCert.certificate;
  } else if ('EffectsCert' in tx) {
    return tx.EffectsCert.certificate;
  }
  return undefined;
}

export function getTransactionDigest(
  tx:
    | CertifiedTransaction
    | HaneulTransactionResponse
    | HaneulExecuteTransactionResponse
): TransactionDigest {
  if ('ImmediateReturn' in tx) {
    return tx.ImmediateReturn.tx_digest;
  }
  if ('transactionDigest' in tx) {
    return tx.transactionDigest;
  }
  const ctxn = getCertifiedTransaction(tx)!;
  return ctxn.transactionDigest;
}

export function getTransactionSignature(tx: CertifiedTransaction): string {
  return tx.txSignature;
}

export function getTransactionAuthorityQuorumSignInfo(
  tx: CertifiedTransaction
): AuthorityQuorumSignInfo {
  return tx.authSignInfo;
}

export function getTransactionData(
  tx: CertifiedTransaction
): HaneulTransactionData {
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

export function getTransferHaneulTransaction(
  data: HaneulTransactionKind
): HaneulTransferHaneul | undefined {
  return 'TransferHaneul' in data ? data.TransferHaneul : undefined;
}

export function getPayTransaction(data: HaneulTransactionKind): Pay | undefined {
  return 'Pay' in data ? data.Pay : undefined;
}

export function getChangeEpochTransaction(
  data: HaneulTransactionKind
): HaneulChangeEpoch | undefined {
  return 'ChangeEpoch' in data ? data.ChangeEpoch : undefined;
}

export function getTransactions(
  data: CertifiedTransaction
): HaneulTransactionKind[] {
  return data.data.transactions;
}

export function getTransferHaneulAmount(data: HaneulTransactionKind): bigint | null {
  return 'TransferHaneul' in data && data.TransferHaneul.amount
    ? BigInt(data.TransferHaneul.amount)
    : null;
}

export function getTransactionKindName(
  data: HaneulTransactionKind
): TransactionKindName {
  return Object.keys(data)[0] as TransactionKindName;
}

/* ----------------------------- ExecutionStatus ---------------------------- */

export function getExecutionStatusType(
  data: HaneulTransactionResponse | HaneulExecuteTransactionResponse
): ExecutionStatusType | undefined {
  return getExecutionStatus(data)?.status;
}

export function getExecutionStatus(
  data: HaneulTransactionResponse | HaneulExecuteTransactionResponse
): ExecutionStatus | undefined {
  return getTransactionEffects(data)?.status;
}

export function getExecutionStatusError(
  data: HaneulTransactionResponse | HaneulExecuteTransactionResponse
): string | undefined {
  return getExecutionStatus(data)?.error;
}

export function getExecutionStatusGasSummary(
  data: HaneulTransactionResponse | HaneulExecuteTransactionResponse
): GasCostSummary | undefined {
  return getTransactionEffects(data)?.gasUsed;
}

export function getTotalGasUsed(
  data: HaneulTransactionResponse | HaneulExecuteTransactionResponse
): number | undefined {
  const gasSummary = getExecutionStatusGasSummary(data);
  return gasSummary
    ? gasSummary.computationCost +
        gasSummary.storageCost -
        gasSummary.storageRebate
    : undefined;
}

export function getTransactionEffects(
  data: HaneulExecuteTransactionResponse | HaneulTransactionResponse
): TransactionEffects | undefined {
  if ('effects' in data) {
    return data.effects;
  }
  return 'EffectsCert' in data ? data.EffectsCert.effects.effects : undefined;
}

/* --------------------------- TransactionResponse -------------------------- */

export function getTimestampFromTransactionResponse(
  data: HaneulExecuteTransactionResponse | HaneulTransactionResponse
): number | undefined {
  return 'timestamp_ms' in data ? data.timestamp_ms ?? undefined : undefined;
}

export function getParsedSplitCoinResponse(
  data: HaneulTransactionResponse
): HaneulParsedSplitCoinResponse | undefined {
  const parsed = data.parsed_data;
  return parsed && 'SplitCoin' in parsed ? parsed.SplitCoin : undefined;
}

export function getParsedMergeCoinResponse(
  data: HaneulTransactionResponse
): HaneulParsedMergeCoinResponse | undefined {
  const parsed = data.parsed_data;
  return parsed && 'MergeCoin' in parsed ? parsed.MergeCoin : undefined;
}

export function getParsedPublishResponse(
  data: HaneulTransactionResponse
): HaneulParsedPublishResponse | undefined {
  const parsed = data.parsed_data;
  return parsed && 'Publish' in parsed ? parsed.Publish : undefined;
}

/**
 * Get the updated coin after a merge.
 * @param data the response for executing a merge coin transaction
 * @returns the updated state of the primary coin after the merge
 */
export function getCoinAfterMerge(
  data: HaneulTransactionResponse
): HaneulObject | undefined {
  return getParsedMergeCoinResponse(data)?.updatedCoin;
}

/**
 * Get the updated coin after a split.
 * @param data the response for executing a Split coin transaction
 * @returns the updated state of the original coin object used for the split
 */
export function getCoinAfterSplit(
  data: HaneulTransactionResponse
): HaneulObject | undefined {
  return getParsedSplitCoinResponse(data)?.updatedCoin;
}

/**
 * Get the newly created coin after a split.
 * @param data the response for executing a Split coin transaction
 * @returns the updated state of the original coin object used for the split
 */
export function getNewlyCreatedCoinsAfterSplit(
  data: HaneulTransactionResponse
): HaneulObject[] | undefined {
  return getParsedSplitCoinResponse(data)?.newCoins;
}

/**
 * Get the newly created coin refs after a split.
 */
export function getNewlyCreatedCoinRefsAfterSplit(
  data: HaneulTransactionResponse | HaneulExecuteTransactionResponse
): HaneulObjectRef[] | undefined {
  if ('EffectsCert' in data) {
    const effects = data.EffectsCert.effects.effects;
    return effects.created?.map((c) => c.reference);
  }
  return undefined;
}
