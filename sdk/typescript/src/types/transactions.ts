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
import { HaneulGasData, HaneulMovePackage, HaneulObjectRef } from './objects';
import {
  ObjectId,
  ObjectOwner,
  HaneulAddress,
  HaneulJsonValue,
  TransactionDigest,
  TransactionEventDigest,
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
  storage_rebate: number(),
  epoch_start_timestamp_ms: optional(number()),
});
export type HaneulChangeEpoch = Infer<typeof HaneulChangeEpoch>;

export const HaneulConsensusCommitPrologue = object({
  epoch: number(),
  round: number(),
  commit_timestamp_ms: number(),
});
export type HaneulConsensusCommitPrologue = Infer<
  typeof HaneulConsensusCommitPrologue
>;

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
  package: string(),
  module: string(),
  function: string(),
  typeArguments: optional(array(string())),
  arguments: optional(array(HaneulJsonValue)),
});
export type MoveCall = Infer<typeof MoveCall>;

export const Genesis = object({
  objects: array(ObjectId),
});
export type Genesis = Infer<typeof Genesis>;

export type ExecuteTransactionRequestType =
  | 'WaitForEffectsCert'
  | 'WaitForLocalExecution';

export type TransactionKindName =
  | 'TransferObject'
  | 'Publish'
  | 'Call'
  | 'TransferHaneul'
  | 'ChangeEpoch'
  | 'ConsensusCommitPrologue'
  | 'Pay'
  | 'PayHaneul'
  | 'PayAllHaneul'
  | 'Genesis';

export const HaneulTransactionKind = union([
  object({ TransferObject: TransferObject }),
  object({ Publish: HaneulMovePackage }),
  object({ Call: MoveCall }),
  object({ TransferHaneul: HaneulTransferHaneul }),
  object({ ChangeEpoch: HaneulChangeEpoch }),
  object({ ConsensusCommitPrologue: HaneulConsensusCommitPrologue }),
  object({ Pay: Pay }),
  object({ PayHaneul: PayHaneul }),
  object({ PayAllHaneul: PayAllHaneul }),
  object({ Genesis: Genesis }),
]);
export type HaneulTransactionKind = Infer<typeof HaneulTransactionKind>;

export const HaneulTransactionData = object({
  transactions: array(HaneulTransactionKind),
  sender: HaneulAddress,
  gasData: HaneulGasData,
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
  /** The epoch when this transaction was executed */
  executedEpoch: EpochId,
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
  /** Object Refs of objects now deleted (the old refs) */
  unwrapped_then_deleted: optional(array(HaneulObjectRef)),
  /** Object refs of objects now wrapped in other objects */
  wrapped: optional(array(HaneulObjectRef)),
  /**
   * The updated gas object reference. Have a dedicated field for convenient access.
   * It's also included in mutated.
   */
  gasObject: OwnedObjectRef,
  /** The events emitted during execution. Note that only successful transactions emit events */
  eventsDigest: optional(TransactionEventDigest),
  /** The set of transaction digests this transaction depends on */
  dependencies: optional(array(TransactionDigest)),
});
export type TransactionEffects = Infer<typeof TransactionEffects>;

export const TransactionEvents = array(HaneulEvent);
export type TransactionEvents = Infer<typeof TransactionEvents>;

export const DryRunTransactionResponse = object({
  effects: TransactionEffects,
  events: TransactionEvents,
});
export type DryRunTransactionResponse = Infer<typeof DryRunTransactionResponse>;

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
  events: TransactionEvents,
  results: DevInspectResultsType,
});
export type DevInspectResults = Infer<typeof DevInspectResults>;

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
export type AuthorityName = Infer<typeof AuthorityName>;
export const AuthorityName = string();

export const TransactionBytes = object({
  txBytes: string(),
  gas: array(HaneulObjectRef),
  // TODO: Type input_objects field
  inputObjects: unknown(),
});

export const HaneulTransaction = object({
  data: HaneulTransactionData,
  txSignatures: array(string()),
});
export type HaneulTransaction = Infer<typeof HaneulTransaction>;

export const HaneulTransactionResponse = object({
  transaction: HaneulTransaction,
  effects: TransactionEffects,
  events: TransactionEvents,
  timestampMs: optional(number()),
  checkpoint: optional(number()),
  confirmedLocalExecution: optional(boolean()),
});
export type HaneulTransactionResponse = Infer<typeof HaneulTransactionResponse>;

/* -------------------------------------------------------------------------- */
/*                              Helper functions                              */
/* -------------------------------------------------------------------------- */

export function getTransaction(tx: HaneulTransactionResponse): HaneulTransaction {
  return tx.transaction;
}

export function getTransactionDigest(
  tx: HaneulTransactionResponse,
): TransactionDigest {
  const effects = getTransactionEffects(tx)!;
  return effects.transactionDigest;
}

export function getTransactionSignature(tx: HaneulTransactionResponse): string[] {
  return tx.transaction.txSignatures;
}

/* ----------------------------- TransactionData ---------------------------- */

export function getTransactionSender(tx: HaneulTransactionResponse): HaneulAddress {
  return tx.transaction.data.sender;
}

export function getGasData(tx: HaneulTransactionResponse): HaneulGasData {
  return tx.transaction.data.gasData;
}

export function getTransactionGasObject(
  tx: HaneulTransactionResponse,
): HaneulObjectRef[] {
  return getGasData(tx).payment;
}

export function getTransactionGasPrice(tx: HaneulTransactionResponse) {
  return getGasData(tx).price;
}

export function getTransactionGasBudget(tx: HaneulTransactionResponse): number {
  return getGasData(tx).budget;
}

export function getTransferObjectTransaction(
  data: HaneulTransactionKind,
): TransferObject | undefined {
  return 'TransferObject' in data ? data.TransferObject : undefined;
}

export function getPublishTransaction(
  data: HaneulTransactionKind,
): HaneulMovePackage | undefined {
  return 'Publish' in data ? data.Publish : undefined;
}

export function getMoveCallTransaction(
  data: HaneulTransactionKind,
): MoveCall | undefined {
  return 'Call' in data ? data.Call : undefined;
}

export function getTransferHaneulTransaction(
  data: HaneulTransactionKind,
): HaneulTransferHaneul | undefined {
  return 'TransferHaneul' in data ? data.TransferHaneul : undefined;
}

export function getPayTransaction(data: HaneulTransactionKind): Pay | undefined {
  return 'Pay' in data ? data.Pay : undefined;
}

export function getPayHaneulTransaction(
  data: HaneulTransactionKind,
): PayHaneul | undefined {
  return 'PayHaneul' in data ? data.PayHaneul : undefined;
}

export function getPayAllHaneulTransaction(
  data: HaneulTransactionKind,
): PayAllHaneul | undefined {
  return 'PayAllHaneul' in data ? data.PayAllHaneul : undefined;
}

export function getChangeEpochTransaction(
  data: HaneulTransactionKind,
): HaneulChangeEpoch | undefined {
  return 'ChangeEpoch' in data ? data.ChangeEpoch : undefined;
}

export function getConsensusCommitPrologueTransaction(
  data: HaneulTransactionKind,
): HaneulConsensusCommitPrologue | undefined {
  return 'ConsensusCommitPrologue' in data
    ? data.ConsensusCommitPrologue
    : undefined;
}

export function getTransactions(
  data: HaneulTransactionResponse,
): HaneulTransactionKind[] {
  return data.transaction.data.transactions;
}

export function getTransferHaneulAmount(data: HaneulTransactionKind): bigint | null {
  return 'TransferHaneul' in data && data.TransferHaneul.amount
    ? BigInt(data.TransferHaneul.amount)
    : null;
}

export function getTransactionKindName(
  data: HaneulTransactionKind,
): TransactionKindName {
  return Object.keys(data)[0] as TransactionKindName;
}

/* ----------------------------- ExecutionStatus ---------------------------- */

export function getExecutionStatusType(
  data: HaneulTransactionResponse,
): ExecutionStatusType | undefined {
  return getExecutionStatus(data)?.status;
}

export function getExecutionStatus(
  data: HaneulTransactionResponse,
): ExecutionStatus | undefined {
  return getTransactionEffects(data)?.status;
}

export function getExecutionStatusError(
  data: HaneulTransactionResponse,
): string | undefined {
  return getExecutionStatus(data)?.error;
}

export function getExecutionStatusGasSummary(
  data: HaneulTransactionResponse | TransactionEffects,
): GasCostSummary | undefined {
  if (is(data, TransactionEffects)) {
    return data.gasUsed;
  }
  return getTransactionEffects(data)?.gasUsed;
}

export function getTotalGasUsed(
  data: HaneulTransactionResponse | TransactionEffects,
): number | undefined {
  const gasSummary = getExecutionStatusGasSummary(data);
  return gasSummary
    ? gasSummary.computationCost +
        gasSummary.storageCost -
        gasSummary.storageRebate
    : undefined;
}

export function getTotalGasUsedUpperBound(
  data: HaneulTransactionResponse | TransactionEffects,
): number | undefined {
  const gasSummary = getExecutionStatusGasSummary(data);
  return gasSummary
    ? gasSummary.computationCost + gasSummary.storageCost
    : undefined;
}

export function getTransactionEffects(
  data: HaneulTransactionResponse,
): TransactionEffects | undefined {
  return data.effects;
}

/* ---------------------------- Transaction Effects --------------------------- */

export function getEvents(
  data: HaneulTransactionResponse,
): HaneulEvent[] | undefined {
  return data.events;
}

export function getCreatedObjects(
  data: HaneulTransactionResponse,
): OwnedObjectRef[] | undefined {
  return getTransactionEffects(data)?.created;
}

/* --------------------------- TransactionResponse -------------------------- */

export function getTimestampFromTransactionResponse(
  data: HaneulTransactionResponse,
): number | undefined {
  return data.timestampMs ?? undefined;
}

/**
 * Get the newly created coin refs after a split.
 */
export function getNewlyCreatedCoinRefsAfterSplit(
  data: HaneulTransactionResponse,
): HaneulObjectRef[] | undefined {
  return getTransactionEffects(data)?.created?.map((c) => c.reference);
}
