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
  assign,
  nullable,
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
  amounts: array(string()),
});
export type Pay = Infer<typeof Pay>;

export const PayHaneul = object({
  coins: array(HaneulObjectRef),
  recipients: array(HaneulAddress),
  amounts: array(string()),
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

export const HaneulArgument = unknown();

export const HaneulCommand = union([
  object({
    MoveCall: object({
      arguments: array(HaneulArgument),
      type_arguments: array(string()),
      package: ObjectId,
      module: string(),
      function: string(),
    }),
  }),
  object({ TransferObjects: tuple([array(HaneulArgument), HaneulArgument]) }),
  object({ SplitCoin: tuple([HaneulArgument, HaneulAddress]) }),
  object({ MergeCoins: tuple([HaneulArgument, array(HaneulArgument)]) }),
  object({ Publish: HaneulMovePackage }),
  object({ MakeMoveVec: tuple([nullable(string()), array(HaneulArgument)]) }),
]);

export const ProgrammableTransaction = object({
  commands: array(),
  inputs: array(HaneulJsonValue),
});
export type ProgrammableTransaction = Infer<typeof ProgrammableTransaction>;

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
  | 'Genesis'
  | 'ProgrammableTransaction';

export const HaneulTransactionKind = union([
  assign(TransferObject, object({ kind: literal('TransferObject') })),
  assign(HaneulMovePackage, object({ kind: literal('Publish') })),
  assign(MoveCall, object({ kind: literal('Call') })),
  assign(HaneulTransferHaneul, object({ kind: literal('TransferHaneul') })),
  assign(HaneulChangeEpoch, object({ kind: literal('ChangeEpoch') })),
  assign(
    HaneulConsensusCommitPrologue,
    object({
      kind: literal('ConsensusCommitPrologue'),
    }),
  ),
  assign(Pay, object({ kind: literal('Pay') })),
  assign(PayHaneul, object({ kind: literal('PayHaneul') })),
  assign(PayAllHaneul, object({ kind: literal('PayAllHaneul') })),
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
  // Eventually this will become union(literal('v1'), literal('v2'), ...)
  messageVersion: literal('v1'),

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
  hasNextPage: boolean(),
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

export const HaneulTransaction = object({
  data: HaneulTransactionData,
  txSignatures: array(string()),
});
export type HaneulTransaction = Infer<typeof HaneulTransaction>;

export const HaneulTransactionResponse = object({
  digest: TransactionDigest,
  transaction: optional(HaneulTransaction),
  effects: optional(TransactionEffects),
  events: optional(TransactionEvents),
  timestampMs: optional(number()),
  checkpoint: optional(number()),
  confirmedLocalExecution: optional(boolean()),
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
});

export type HaneulTransactionResponseOptions = Infer<
  typeof HaneulTransactionResponseOptions
>;

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

export function getTransferObjectTransaction(
  data: HaneulTransactionKind,
): TransferObject | undefined {
  return data.kind === 'TransferObject' ? data : undefined;
}

export function getPublishTransaction(
  data: HaneulTransactionKind,
): HaneulMovePackage | undefined {
  return data.kind === 'Publish' ? data : undefined;
}

export function getMoveCallTransaction(
  data: HaneulTransactionKind,
): MoveCall | undefined {
  return data.kind === 'Call' ? data : undefined;
}

export function getTransferHaneulTransaction(
  data: HaneulTransactionKind,
): HaneulTransferHaneul | undefined {
  return data.kind === 'TransferHaneul' ? data : undefined;
}

export function getPayTransaction(data: HaneulTransactionKind): Pay | undefined {
  return data.kind === 'Pay' ? data : undefined;
}

export function getPayHaneulTransaction(
  data: HaneulTransactionKind,
): PayHaneul | undefined {
  return data.kind === 'PayHaneul' ? data : undefined;
}

export function getPayAllHaneulTransaction(
  data: HaneulTransactionKind,
): PayAllHaneul | undefined {
  return data.kind === 'PayAllHaneul' ? data : undefined;
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

export function getTransactionKinds(
  data: HaneulTransactionResponse,
): HaneulTransactionKind[] | undefined {
  return data.transaction?.data.transactions;
}

export function getTransferHaneulAmount(data: HaneulTransactionKind): bigint | null {
  return data.kind === 'TransferHaneul' && data.amount
    ? BigInt(data.amount)
    : null;
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
