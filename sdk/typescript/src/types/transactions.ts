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
  boolean,
  tuple,
  assign,
  nullable,
} from 'superstruct';

import {
  ObjectId,
  ObjectOwner,
  SequenceNumber,
  HaneulAddress,
  HaneulJsonValue,
  TransactionDigest,
  TransactionEventDigest,
} from './common';
import { HaneulEvent } from './events';
import {
  ObjectDigest,
  HaneulGasData,
  HaneulMovePackage,
  HaneulObjectRef,
} from './objects';

export const EpochId = string();

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

export const Genesis = object({
  objects: array(ObjectId),
});
export type Genesis = Infer<typeof Genesis>;

export const HaneulArgument = union([
  literal('GasCoin'),
  object({ Input: number() }),
  object({ Result: number() }),
  object({ NestedResult: tuple([number(), number()]) }),
]);
export type HaneulArgument = Infer<typeof HaneulArgument>;

export const MoveCallHaneulCommand = object({
  arguments: optional(array(HaneulArgument)),
  type_arguments: optional(array(string())),
  package: ObjectId,
  module: string(),
  function: string(),
});
export type MoveCallHaneulCommand = Infer<typeof MoveCallHaneulCommand>;

export const HaneulCommand = union([
  object({ MoveCall: MoveCallHaneulCommand }),
  object({ TransferObjects: tuple([array(HaneulArgument), HaneulArgument]) }),
  object({ SplitCoins: tuple([HaneulArgument, array(HaneulArgument)]) }),
  object({ MergeCoins: tuple([HaneulArgument, array(HaneulArgument)]) }),
  object({ Publish: HaneulMovePackage }),
  object({ MakeMoveVec: tuple([nullable(string()), array(HaneulArgument)]) }),
]);

export const HaneulCallArg = union([
  object({
    type: literal('pure'),
    valueType: optional(string()),
    value: HaneulJsonValue,
  }),
  object({
    type: literal('object'),
    objectType: literal('immOrOwnedObject'),
    objectId: ObjectId,
    version: SequenceNumber,
    digest: ObjectDigest,
  }),
  object({
    type: literal('object'),
    objectType: literal('sharedObject'),
    objectId: ObjectId,
    initialSharedVersion: SequenceNumber,
    mutable: boolean(),
  }),
]);
export type HaneulCallArg = Infer<typeof HaneulCallArg>;

export const ProgrammableTransaction = object({
  commands: array(HaneulCommand),
  inputs: array(HaneulCallArg),
});
export type ProgrammableTransaction = Infer<typeof ProgrammableTransaction>;
export type ProgrammableTransactionCommand = Infer<typeof HaneulCommand>;

/**
 * 1. WaitForEffectsCert: waits for TransactionEffectsCert and then returns to the client.
 *    This mode is a proxy for transaction finality.
 * 2. WaitForLocalExecution: waits for TransactionEffectsCert and makes sure the node
 *    executed the transaction locally before returning to the client. The local execution
 *    makes sure this node is aware of this transaction when the client fires subsequent queries.
 *    However, if the node fails to execute the transaction locally in a timely manner,
 *    a bool type in the response is set to false to indicate the case.
 */
export type ExecuteTransactionRequestType =
  | 'WaitForEffectsCert'
  | 'WaitForLocalExecution';

export type TransactionKindName =
  | 'ChangeEpoch'
  | 'ConsensusCommitPrologue'
  | 'Genesis'
  | 'ProgrammableTransaction';

export const HaneulTransactionKind = union([
  assign(HaneulChangeEpoch, object({ kind: literal('ChangeEpoch') })),
  assign(
    HaneulConsensusCommitPrologue,
    object({
      kind: literal('ConsensusCommitPrologue'),
    }),
  ),
  assign(Genesis, object({ kind: literal('Genesis') })),
  assign(
    ProgrammableTransaction,
    object({ kind: literal('ProgrammableTransaction') }),
  ),
]);
export type HaneulTransactionKind = Infer<typeof HaneulTransactionKind>;

export const HaneulTransactionData = object({
  // Eventually this will become union(literal('v1'), literal('v2'), ...)
  messageVersion: literal('v1'),
  transaction: HaneulTransactionKind,
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
  computationCost: string(),
  storageCost: string(),
  storageRebate: string(),
  nonRefundableStorageFee: string(),
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

export const OwnedObjectRef = object({
  owner: ObjectOwner,
  reference: HaneulObjectRef,
});
export type OwnedObjectRef = Infer<typeof OwnedObjectRef>;
export const TransactionEffectsModifiedAtVersions = object({
  objectId: ObjectId,
  sequenceNumber: SequenceNumber,
});

export const TransactionEffects = object({
  // Eventually this will become union(literal('v1'), literal('v2'), ...)
  messageVersion: literal('v1'),

  /** The status of the execution */
  status: ExecutionStatus,
  /** The epoch when this transaction was executed */
  executedEpoch: EpochId,
  /** The version that every modified (mutated or deleted) object had before it was modified by this transaction. **/
  modifiedAtVersions: optional(array(TransactionEffectsModifiedAtVersions)),
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

const ReturnValueType = tuple([array(number()), string()]);
const MutableReferenceOutputType = tuple([
  HaneulArgument,
  array(number()),
  string(),
]);
const ExecutionResultType = object({
  mutableReferenceOutputs: optional(array(MutableReferenceOutputType)),
  returnValues: optional(array(ReturnValueType)),
});

export const DevInspectResults = object({
  effects: TransactionEffects,
  events: TransactionEvents,
  results: optional(array(ExecutionResultType)),
  error: optional(string()),
});
export type DevInspectResults = Infer<typeof DevInspectResults>;

export const GetTxnDigestsResponse = array(TransactionDigest);
export type GetTxnDigestsResponse = Infer<typeof GetTxnDigestsResponse>;

export type HaneulTransactionResponseQuery = {
  filter?: TransactionFilter;
  options?: HaneulTransactionResponseOptions;
};

export type TransactionFilter =
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

export const HaneulTransaction = object({
  data: HaneulTransactionData,
  txSignatures: array(string()),
});
export type HaneulTransaction = Infer<typeof HaneulTransaction>;

export const HaneulObjectChangePublished = object({
  type: literal('published'),
  packageId: ObjectId,
  version: SequenceNumber,
  digest: ObjectDigest,
  modules: array(string()),
});
export type HaneulObjectChangePublished = Infer<typeof HaneulObjectChangePublished>;

export const HaneulObjectChangeTransferred = object({
  type: literal('transferred'),
  sender: HaneulAddress,
  recipient: ObjectOwner,
  objectType: string(),
  objectId: ObjectId,
  version: SequenceNumber,
  digest: ObjectDigest,
});
export type HaneulObjectChangeTransferred = Infer<
  typeof HaneulObjectChangeTransferred
>;

export const HaneulObjectChangeMutated = object({
  type: literal('mutated'),
  sender: HaneulAddress,
  owner: ObjectOwner,
  objectType: string(),
  objectId: ObjectId,
  version: SequenceNumber,
  previousVersion: SequenceNumber,
  digest: ObjectDigest,
});
export type HaneulObjectChangeMutated = Infer<typeof HaneulObjectChangeMutated>;

export const HaneulObjectChangeDeleted = object({
  type: literal('deleted'),
  sender: HaneulAddress,
  objectType: string(),
  objectId: ObjectId,
  version: SequenceNumber,
});
export type HaneulObjectChangeDeleted = Infer<typeof HaneulObjectChangeDeleted>;

export const HaneulObjectChangeWrapped = object({
  type: literal('wrapped'),
  sender: HaneulAddress,
  objectType: string(),
  objectId: ObjectId,
  version: SequenceNumber,
});
export type HaneulObjectChangeWrapped = Infer<typeof HaneulObjectChangeWrapped>;

export const HaneulObjectChangeCreated = object({
  type: literal('created'),
  sender: HaneulAddress,
  owner: ObjectOwner,
  objectType: string(),
  objectId: ObjectId,
  version: SequenceNumber,
  digest: ObjectDigest,
});
export type HaneulObjectChangeCreated = Infer<typeof HaneulObjectChangeCreated>;

export const HaneulObjectChange = union([
  HaneulObjectChangePublished,
  HaneulObjectChangeTransferred,
  HaneulObjectChangeMutated,
  HaneulObjectChangeDeleted,
  HaneulObjectChangeWrapped,
  HaneulObjectChangeCreated,
]);
export type HaneulObjectChange = Infer<typeof HaneulObjectChange>;

export const BalanceChange = object({
  owner: ObjectOwner,
  coinType: string(),
  /* Coin balance change(positive means receive, negative means send) */
  amount: string(),
});

export const HaneulTransactionResponse = object({
  digest: TransactionDigest,
  transaction: optional(HaneulTransaction),
  effects: optional(TransactionEffects),
  events: optional(TransactionEvents),
  timestampMs: optional(number()),
  checkpoint: optional(number()),
  confirmedLocalExecution: optional(boolean()),
  objectChanges: optional(array(HaneulObjectChange)),
  balanceChanges: optional(array(BalanceChange)),
  /* Errors that occurred in fetching/serializing the transaction. */
  errors: optional(array(string())),
});
export type HaneulTransactionResponse = Infer<typeof HaneulTransactionResponse>;

export const HaneulTransactionResponseOptions = object({
  /* Whether to show transaction input data. Default to be false. */
  showInput: optional(boolean()),
  /* Whether to show transaction effects. Default to be false. */
  showEffects: optional(boolean()),
  /* Whether to show transaction events. Default to be false. */
  showEvents: optional(boolean()),
  /* Whether to show object changes. Default to be false. */
  showObjectChanges: optional(boolean()),
  /* Whether to show coin balance changes. Default to be false. */
  showBalanceChanges: optional(boolean()),
});

export type HaneulTransactionResponseOptions = Infer<
  typeof HaneulTransactionResponseOptions
>;

export const PaginatedTransactionResponse = object({
  data: array(HaneulTransactionResponse),
  nextCursor: union([TransactionDigest, literal(null)]),
  hasNextPage: boolean(),
});
export type PaginatedTransactionResponse = Infer<
  typeof PaginatedTransactionResponse
>;
export const DryRunTransactionResponse = object({
  effects: TransactionEffects,
  events: TransactionEvents,
  objectChanges: array(HaneulObjectChange),
  balanceChanges: array(BalanceChange),
});
export type DryRunTransactionResponse = Infer<typeof DryRunTransactionResponse>;

/* -------------------------------------------------------------------------- */
/*                              Helper functions                              */
/* -------------------------------------------------------------------------- */

export function getTransaction(
  tx: HaneulTransactionResponse,
): HaneulTransaction | undefined {
  return tx.transaction;
}

export function getTransactionDigest(
  tx: HaneulTransactionResponse,
): TransactionDigest {
  return tx.digest;
}

export function getTransactionSignature(
  tx: HaneulTransactionResponse,
): string[] | undefined {
  return tx.transaction?.txSignatures;
}

/* ----------------------------- TransactionData ---------------------------- */

export function getTransactionSender(
  tx: HaneulTransactionResponse,
): HaneulAddress | undefined {
  return tx.transaction?.data.sender;
}

export function getGasData(tx: HaneulTransactionResponse): HaneulGasData | undefined {
  return tx.transaction?.data.gasData;
}

export function getTransactionGasObject(
  tx: HaneulTransactionResponse,
): HaneulObjectRef[] | undefined {
  return getGasData(tx)?.payment;
}

export function getTransactionGasPrice(tx: HaneulTransactionResponse) {
  return getGasData(tx)?.price;
}

export function getTransactionGasBudget(tx: HaneulTransactionResponse) {
  return getGasData(tx)?.budget;
}

export function getChangeEpochTransaction(
  data: HaneulTransactionKind,
): HaneulChangeEpoch | undefined {
  return data.kind === 'ChangeEpoch' ? data : undefined;
}

export function getConsensusCommitPrologueTransaction(
  data: HaneulTransactionKind,
): HaneulConsensusCommitPrologue | undefined {
  return data.kind === 'ConsensusCommitPrologue' ? data : undefined;
}

export function getTransactionKind(
  data: HaneulTransactionResponse,
): HaneulTransactionKind | undefined {
  return data.transaction?.data.transaction;
}

export function getTransactionKindName(
  data: HaneulTransactionKind,
): TransactionKindName {
  return data.kind;
}

export function getProgrammableTransaction(
  data: HaneulTransactionKind,
): ProgrammableTransaction | undefined {
  return data.kind === 'ProgrammableTransaction' ? data : undefined;
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
): bigint | undefined {
  const gasSummary = getExecutionStatusGasSummary(data);
  return gasSummary
    ? BigInt(gasSummary.computationCost) +
        BigInt(gasSummary.storageCost) -
        BigInt(gasSummary.storageRebate)
    : undefined;
}

export function getTotalGasUsedUpperBound(
  data: HaneulTransactionResponse | TransactionEffects,
): bigint | undefined {
  const gasSummary = getExecutionStatusGasSummary(data);
  return gasSummary
    ? BigInt(gasSummary.computationCost) + BigInt(gasSummary.storageCost)
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

export function getObjectChanges(
  data: HaneulTransactionResponse,
): HaneulObjectChange[] | undefined {
  return data.objectChanges;
}

export function getPublishedObjectChanges(
  data: HaneulTransactionResponse,
): HaneulObjectChangePublished[] {
  return (
    (data.objectChanges?.filter((a) =>
      is(a, HaneulObjectChangePublished),
    ) as HaneulObjectChangePublished[]) ?? []
  );
}
