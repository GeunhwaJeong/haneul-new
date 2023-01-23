// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  is,
  array,
  Infer,
  literal,
  number,
  object,
  optional,
  string,
  union,
  unknown,
  boolean,
  tuple,
} from 'superstruct';
import { HaneulEvent } from './events';
import { HaneulMovePackage, HaneulObject, HaneulObjectRef } from './objects';
import {
  ObjectId,
  ObjectOwner,
  HaneulAddress,
  HaneulJsonValue,
  TransactionDigest,
} from './common';

// TODO: support u64
export const EpochId = number();

export const TransferObject = object({
  recipient: HaneulAddress,
  objectRef: HaneulObjectRef,
});
export type TransferObject = Infer<typeof TransferObject>;

export const HaneulTransferHaneul = object({
  recipient: HaneulAddress,
  amount: union([number(), literal(null)]),
});
export type HaneulTransferHaneul = Infer<typeof HaneulTransferHaneul>;

export const HaneulChangeEpoch = object({
  epoch: EpochId,
  storage_charge: number(),
  computation_charge: number(),
});
export type HaneulChangeEpoch = Infer<typeof HaneulChangeEpoch>;

export const Pay = object({
  coins: array(HaneulObjectRef),
  recipients: array(HaneulAddress),
  amounts: array(number()),
});
export type Pay = Infer<typeof Pay>;

export const PayHaneul = object({
  coins: array(HaneulObjectRef),
  recipients: array(HaneulAddress),
  amounts: array(number()),
});
export type PayHaneul = Infer<typeof PayHaneul>;

export const PayAllHaneul = object({
  coins: array(HaneulObjectRef),
  recipient: HaneulAddress,
});
export type PayAllHaneul = Infer<typeof PayAllHaneul>;

export const MoveCall = object({
  package: HaneulObjectRef,
  module: string(),
  function: string(),
  typeArguments: optional(array(string())),
  arguments: array(HaneulJsonValue),
});
export type MoveCall = Infer<typeof MoveCall>;

export type ExecuteTransactionRequestType =
  | 'WaitForEffectsCert'
  | 'WaitForLocalExecution';

export type TransactionKindName =
  | 'TransferObject'
  | 'Publish'
  | 'Call'
  | 'TransferHaneul'
  | 'ChangeEpoch'
  | 'Pay'
  | 'PayHaneul'
  | 'PayAllHaneul';

export const HaneulTransactionKind = union([
  object({ TransferObject: TransferObject }),
  object({ Publish: HaneulMovePackage }),
  object({ Call: MoveCall }),
  object({ TransferHaneul: HaneulTransferHaneul }),
  object({ ChangeEpoch: HaneulChangeEpoch }),
  object({ Pay: Pay }),
  object({ PayHaneul: PayHaneul }),
  object({ PayAllHaneul: PayAllHaneul }),
]);
export type HaneulTransactionKind = Infer<typeof HaneulTransactionKind>;

export const HaneulTransactionData = object({
  transactions: array(HaneulTransactionKind),
  sender: HaneulAddress,
  gasPayment: HaneulObjectRef,
  // TODO: remove optional after 0.21.0 is released
  gasPrice: optional(number()),
  gasBudget: number(),
});
export type HaneulTransactionData = Infer<typeof HaneulTransactionData>;

export const AuthoritySignature = string();
export const GenericAuthoritySignature = union([
  AuthoritySignature,
  array(AuthoritySignature),
]);

export const AuthorityQuorumSignInfo = object({
  epoch: EpochId,
  signature: GenericAuthoritySignature,
  signers_map: array(number()),
});
export type AuthorityQuorumSignInfo = Infer<typeof AuthorityQuorumSignInfo>;

export const CertifiedTransaction = object({
  transactionDigest: TransactionDigest,
  data: HaneulTransactionData,
  txSignature: string(),
  authSignInfo: AuthorityQuorumSignInfo,
});
export type CertifiedTransaction = Infer<typeof CertifiedTransaction>;

export const GasCostSummary = object({
  computationCost: number(),
  storageCost: number(),
  storageRebate: number(),
});
export type GasCostSummary = Infer<typeof GasCostSummary>;

export const ExecutionStatusType = union([
  literal('success'),
  literal('failure'),
]);
export type ExecutionStatusType = Infer<typeof ExecutionStatusType>;

export const ExecutionStatus = object({
  status: ExecutionStatusType,
  error: optional(string()),
});
export type ExecutionStatus = Infer<typeof ExecutionStatus>;

// TODO: change the tuple to struct from the server end
export const OwnedObjectRef = object({
  owner: ObjectOwner,
  reference: HaneulObjectRef,
});
export type OwnedObjectRef = Infer<typeof OwnedObjectRef>;

export const TransactionEffects = object({
  /** The status of the execution */
  status: ExecutionStatus,
  gasUsed: GasCostSummary,
  /** The object references of the shared objects used in this transaction. Empty if no shared objects were used. */
  sharedObjects: optional(array(HaneulObjectRef)),
  /** The transaction digest */
  transactionDigest: TransactionDigest,
  /** ObjectRef and owner of new objects created */
  created: optional(array(OwnedObjectRef)),
  /** ObjectRef and owner of mutated objects, including gas object */
  mutated: optional(array(OwnedObjectRef)),
  /**
   * ObjectRef and owner of objects that are unwrapped in this transaction.
   * Unwrapped objects are objects that were wrapped into other objects in the past,
   * and just got extracted out.
   */
  unwrapped: optional(array(OwnedObjectRef)),
  /** Object Refs of objects now deleted (the old refs) */
  deleted: optional(array(HaneulObjectRef)),
  /** Object refs of objects now wrapped in other objects */
  wrapped: optional(array(HaneulObjectRef)),
  /**
   * The updated gas object reference. Have a dedicated field for convenient access.
   * It's also included in mutated.
   */
  gasObject: OwnedObjectRef,
  /** The events emitted during execution. Note that only successful transactions emit events */
  events: optional(array(HaneulEvent)),
  /** The set of transaction digests this transaction depends on */
  dependencies: optional(array(TransactionDigest)),
});
export type TransactionEffects = Infer<typeof TransactionEffects>;

const ReturnValueType = tuple([array(number()), string()]);
const MutableReferenceOutputType = tuple([number(), array(number()), string()]);
const ExecutionResultType = object({
  mutableReferenceOutputs: optional(array(MutableReferenceOutputType)),
  returnValues: optional(array(ReturnValueType)),
});
const DevInspectResultTupleType = tuple([number(), ExecutionResultType]);

const DevInspectResultsType = union([
  object({ Ok: array(DevInspectResultTupleType) }),
  object({ Err: string() }),
]);

export const DevInspectResults = object({
  effects: TransactionEffects,
  results: DevInspectResultsType,
});
export type DevInspectResults = Infer<typeof DevInspectResults>;

export const HaneulTransactionAuthSignersResponse = object({
  signers: array(string()),
});
export type HaneulTransactionAuthSignersResponse = Infer<
  typeof HaneulTransactionAuthSignersResponse
>;

// TODO: this is likely to go away after https://github.com/GeunhwaJeong/haneul/issues/4207
export const HaneulCertifiedTransactionEffects = object({
  transactionEffectsDigest: string(),
  authSignInfo: AuthorityQuorumSignInfo,
  effects: TransactionEffects,
});

export const HaneulExecuteTransactionResponse = union([
  object({ TxCert: object({ certificate: CertifiedTransaction }) }),
  object({
    EffectsCert: object({
      certificate: CertifiedTransaction,
      effects: HaneulCertifiedTransactionEffects,
      confirmed_local_execution: boolean(),
    }),
  }),
]);
export type HaneulExecuteTransactionResponse = Infer<
  typeof HaneulExecuteTransactionResponse
>;

export type GatewayTxSeqNumber = number;

export const GetTxnDigestsResponse = array(TransactionDigest);
export type GetTxnDigestsResponse = Infer<typeof GetTxnDigestsResponse>;

export const PaginatedTransactionDigests = object({
  data: array(TransactionDigest),
  nextCursor: union([TransactionDigest, literal(null)]),
});
export type PaginatedTransactionDigests = Infer<
  typeof PaginatedTransactionDigests
>;

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

export type EmptySignInfo = object;
export type AuthorityName = string;

export const TransactionBytes = object({
  txBytes: string(),
  gas: HaneulObjectRef,
  // TODO: Type input_objects field
  inputObjects: unknown(),
});

export const HaneulParsedMergeCoinResponse = object({
  updatedCoin: HaneulObject,
  updatedGas: HaneulObject,
});
export type HaneulParsedMergeCoinResponse = Infer<
  typeof HaneulParsedMergeCoinResponse
>;

export const HaneulParsedSplitCoinResponse = object({
  updatedCoin: HaneulObject,
  newCoins: array(HaneulObject),
  updatedGas: HaneulObject,
});
export type HaneulParsedSplitCoinResponse = Infer<
  typeof HaneulParsedSplitCoinResponse
>;

export const HaneulPackage = object({
  digest: string(),
  objectId: string(),
  version: number(),
});

export const HaneulParsedPublishResponse = object({
  createdObjects: array(HaneulObject),
  package: HaneulPackage,
  updatedGas: HaneulObject,
});
export type HaneulParsedPublishResponse = Infer<typeof HaneulParsedPublishResponse>;

export const HaneulParsedTransactionResponse = union([
  object({ SplitCoin: HaneulParsedSplitCoinResponse }),
  object({ MergeCoin: HaneulParsedMergeCoinResponse }),
  object({ Publish: HaneulParsedPublishResponse }),
]);
export type HaneulParsedTransactionResponse = Infer<
  typeof HaneulParsedTransactionResponse
>;

export const HaneulTransactionResponse = object({
  certificate: CertifiedTransaction,
  effects: TransactionEffects,
  timestamp_ms: union([number(), literal(null)]),
  parsed_data: union([HaneulParsedTransactionResponse, literal(null)]),
});
export type HaneulTransactionResponse = Infer<typeof HaneulTransactionResponse>;

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

export function getTransactionGasPrice(tx: CertifiedTransaction) {
  return tx.data.gasPrice;
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

export function getPayHaneulTransaction(
  data: HaneulTransactionKind
): PayHaneul | undefined {
  return 'PayHaneul' in data ? data.PayHaneul : undefined;
}

export function getPayAllHaneulTransaction(
  data: HaneulTransactionKind
): PayAllHaneul | undefined {
  return 'PayAllHaneul' in data ? data.PayAllHaneul : undefined;
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
  data:
    | HaneulTransactionResponse
    | HaneulExecuteTransactionResponse
    | TransactionEffects
): GasCostSummary | undefined {
  if (is(data, TransactionEffects)) {
    return data.gasUsed;
  }
  return getTransactionEffects(data)?.gasUsed;
}

export function getTotalGasUsed(
  data:
    | HaneulTransactionResponse
    | HaneulExecuteTransactionResponse
    | TransactionEffects
): number | undefined {
  const gasSummary = getExecutionStatusGasSummary(data);
  return gasSummary
    ? gasSummary.computationCost +
        gasSummary.storageCost -
        gasSummary.storageRebate
    : undefined;
}

export function getTotalGasUsedUpperBound(
  data:
    | HaneulTransactionResponse
    | HaneulExecuteTransactionResponse
    | TransactionEffects
): number | undefined {
  const gasSummary = getExecutionStatusGasSummary(data);
  return gasSummary
    ? gasSummary.computationCost +
        gasSummary.storageCost
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

/* ---------------------------- Transaction Effects --------------------------- */

export function getEvents(
  data: HaneulExecuteTransactionResponse | HaneulTransactionResponse
): HaneulEvent[] | undefined {
  return getTransactionEffects(data)?.events;
}

export function getCreatedObjects(
  data: HaneulExecuteTransactionResponse | HaneulTransactionResponse
): OwnedObjectRef[] | undefined {
  return getTransactionEffects(data)?.created;
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
