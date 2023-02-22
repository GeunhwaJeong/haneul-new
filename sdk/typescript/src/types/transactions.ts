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
import { HaneulGasData, HaneulMovePackage, HaneulObject, HaneulObjectRef } from './objects';
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
  // TODO: Make non-optional after v0.26.0 lands everywhere
  storage_rebate: optional(number()),
  epoch_start_timestamp_ms: optional(number()),
});
export type HaneulChangeEpoch = Infer<typeof HaneulChangeEpoch>;

export const HaneulConsensusCommitPrologue = object({
  checkpoint_start_timestamp_ms: number(),
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
  // TODO: Simplify once 0.24.0 lands
  package: union([string(), HaneulObjectRef]),
  module: string(),
  function: string(),
  typeArguments: optional(array(string())),
  arguments: array(HaneulJsonValue),
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

export const CertifiedTransaction = object({
  transactionDigest: TransactionDigest,
  data: HaneulTransactionData,
  txSignatures: array(string()),
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
  /**
   * The epoch when this transaction was executed
   * TODO: Changed it to non-optional once this is stable.
   * */
  executedEpoch: optional(EpochId),
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

export const HaneulEffectsFinalityInfo = union([
  object({ certified: AuthorityQuorumSignInfo }),
  object({ checkpointed: tuple([number(), number()]) }),
]);
export type HaneulEffectsFinalityInfo = Infer<typeof HaneulEffectsFinalityInfo>;

export const HaneulFinalizedEffects = object({
  transactionEffectsDigest: string(),
  effects: TransactionEffects,
  finalityInfo: HaneulEffectsFinalityInfo,
});
export type HaneulFinalizedEffects = Infer<typeof HaneulFinalizedEffects>;

export const HaneulTransactionData_v26 = object({
  transactions: array(HaneulTransactionKind),
  sender: HaneulAddress,
  gasPayment: HaneulObjectRef,
  // TODO: remove optional after 0.21.0 is released
  gasPrice: optional(number()),
  gasBudget: number(),
});
export type HaneulTransactionData_v26 = Infer<typeof HaneulTransactionData_v26>;

export function toHaneulTransactionData(
  tx_data: HaneulTransactionData_v26,
): HaneulTransactionData {
  return {
    transactions: tx_data.transactions,
    sender: tx_data.sender,
    gasData: {
      payment: tx_data.gasPayment,
      owner: tx_data.sender,
      budget: tx_data.gasBudget,
      price: tx_data.gasPrice!,
    },
  };
}

export const CertifiedTransaction_v26 = object({
  transactionDigest: TransactionDigest,
  data: HaneulTransactionData_v26,
  txSignature: string(),
  authSignInfo: AuthorityQuorumSignInfo,
});
export type CertifiedTransaction_v26 = Infer<typeof CertifiedTransaction_v26>;

export const HaneulExecuteTransactionResponse_v26 = object({
  certificate: optional(CertifiedTransaction_v26),
  effects: HaneulFinalizedEffects,
  confirmed_local_execution: boolean(),
});

export type HaneulExecuteTransactionResponse_v26 = Infer<
  typeof HaneulExecuteTransactionResponse_v26
>;

// TODO: Remove after devnet 0.28.0

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

export const HaneulTransaction = object({
  data: HaneulTransactionData,
  txSignatures: array(string()),
});
export type HaneulTransaction = Infer<typeof HaneulTransaction>;

export const HaneulTransactionResponse = object({
  // TODO: Remove optional after devnet 0.28.0
  transaction: optional(HaneulTransaction),
  // TODO: Remove after devnet 0.28.0
  certificate: optional(
    union([CertifiedTransaction, CertifiedTransaction_v26]),
  ),
  effects: TransactionEffects,
  // TODO: Remove after devnet 0.28.0
  timestamp_ms: optional(union([number(), literal(null)])),
  // TODO: Remove optioanl after devnet 0.28.0
  timestampMs: optional(union([number(), literal(null)])),
  // TODO: remove optional after 0.27.0 is released
  checkpoint: optional(union([number(), literal(null)])),
  // TODO: Remove optioanl after devnet 0.28.0
  confirmedLocalExecution: optional(boolean()),
  // TODO: Remove after devnet 0.28.0
  parsed_data: optional(union([HaneulParsedTransactionResponse, literal(null)])),
});
export type HaneulTransactionResponse = Infer<typeof HaneulTransactionResponse>;

// TODO: Remove after devnet 0.28.0
export const HaneulExecuteTransactionResponse = union([
  object({
    EffectsCert: object({
      certificate: CertifiedTransaction,
      effects: HaneulCertifiedTransactionEffects,
      confirmed_local_execution: boolean(),
    }),
  }),
  object({
    certificate: optional(CertifiedTransaction),
    effects: HaneulFinalizedEffects,
    confirmed_local_execution: boolean(),
  }),
  HaneulTransactionResponse,
]);
export type HaneulExecuteTransactionResponse = Infer<
  typeof HaneulExecuteTransactionResponse
>;

/* -------------------------------------------------------------------------- */
/*                              Helper functions                              */
/* -------------------------------------------------------------------------- */

/* ---------------------------------- CertifiedTransaction --------------------------------- */

export function getCertifiedTransaction(
  tx: HaneulTransactionResponse | HaneulExecuteTransactionResponse,
): CertifiedTransaction | CertifiedTransaction_v26 | undefined {
  if ('certificate' in tx) {
    return tx.certificate;
  } else if ('EffectsCert' in tx) {
    return tx.EffectsCert.certificate;
  }
  return undefined;
}

export function getTransactionDigest(
  tx:
    | CertifiedTransaction
    | CertifiedTransaction_v26
    | HaneulTransactionResponse
    | HaneulExecuteTransactionResponse,
): TransactionDigest {
  if ('transactionDigest' in tx) {
    return tx.transactionDigest;
  }
  const effects = getTransactionEffects(tx)!;
  return effects.transactionDigest;
}

export function getTransactionSignature(
  tx: HaneulTransactionResponse | CertifiedTransaction | CertifiedTransaction_v26,
): string[] {
  const certificateOrTx =
    'certificate' in tx
      ? tx.certificate!
      : 'transaction' in tx
      ? tx.transaction!
      : tx;

  if ('txSignatures' in certificateOrTx) {
    return certificateOrTx.txSignatures;
  }

  if ('txSignature' in certificateOrTx) {
    return [certificateOrTx.txSignature];
  }

  return [];
}

export function getTransactionData(
  tx: CertifiedTransaction,
): HaneulTransactionData {
  return tx.data;
}

/* ----------------------------- TransactionData ---------------------------- */

export function getTransactionSender(tx: HaneulTransactionResponse): HaneulAddress {
  return tx.certificate
    ? tx.certificate.data.sender
    : tx.transaction!.data.sender;
}

export function getGasData(
  tx: CertifiedTransaction | HaneulTransactionResponse,
): HaneulGasData {
  if ('data' in tx) {
    return tx.data.gasData;
  }

  if ('certificate' in tx) {
    const data = tx.certificate!.data;
    if ('gasData' in data) {
      return data.gasData;
    } else {
      return {
        payment: data.gasPayment,
        budget: data.gasBudget,
        owner: data.sender,
        price: data.gasPrice!,
      };
    }
  }

  return tx.transaction!.data.gasData;
}

export function getTransactionGasObject(
  tx: HaneulTransactionResponse | CertifiedTransaction,
): HaneulObjectRef {
  return getGasData(tx).payment;
}

export function getTransactionGasPrice(
  tx: HaneulTransactionResponse | CertifiedTransaction,
) {
  return getGasData(tx).price;
}

export function getTransactionGasBudget(
  tx: HaneulTransactionResponse | CertifiedTransaction,
): number {
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
  return data.certificate
    ? data.certificate.data.transactions
    : data.transaction!.data.transactions;
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
  data: HaneulTransactionResponse | HaneulExecuteTransactionResponse,
): ExecutionStatusType | undefined {
  return getExecutionStatus(data)?.status;
}

export function getExecutionStatus(
  data: HaneulTransactionResponse | HaneulExecuteTransactionResponse,
): ExecutionStatus | undefined {
  return getTransactionEffects(data)?.status;
}

export function getExecutionStatusError(
  data: HaneulTransactionResponse | HaneulExecuteTransactionResponse,
): string | undefined {
  return getExecutionStatus(data)?.error;
}

export function getExecutionStatusGasSummary(
  data:
    | HaneulTransactionResponse
    | HaneulExecuteTransactionResponse
    | TransactionEffects,
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
    | TransactionEffects,
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
    | TransactionEffects,
): number | undefined {
  const gasSummary = getExecutionStatusGasSummary(data);
  return gasSummary
    ? gasSummary.computationCost + gasSummary.storageCost
    : undefined;
}

export function getTransactionEffects(
  data: HaneulExecuteTransactionResponse | HaneulTransactionResponse,
): TransactionEffects | undefined {
  if ('effects' in data) {
    return `effects` in data.effects ? data.effects.effects : data.effects;
  }
  return 'EffectsCert' in data ? data.EffectsCert.effects.effects : undefined;
}

/* ---------------------------- Transaction Effects --------------------------- */

export function getEvents(
  data: HaneulExecuteTransactionResponse | HaneulTransactionResponse,
): HaneulEvent[] | undefined {
  return getTransactionEffects(data)?.events;
}

export function getCreatedObjects(
  data: HaneulExecuteTransactionResponse | HaneulTransactionResponse,
): OwnedObjectRef[] | undefined {
  return getTransactionEffects(data)?.created;
}

/* --------------------------- TransactionResponse -------------------------- */

export function getTimestampFromTransactionResponse(
  data: HaneulExecuteTransactionResponse | HaneulTransactionResponse,
): number | undefined {
  return 'timestamp_ms' in data || 'timestampMs' in data
    ? (data.timestamp_ms || data.timestampMs) ?? undefined
    : undefined;
}

export function getParsedSplitCoinResponse(
  data: HaneulTransactionResponse,
): HaneulParsedSplitCoinResponse | undefined {
  const parsed = data.parsed_data;
  return parsed && 'SplitCoin' in parsed ? parsed.SplitCoin : undefined;
}

export function getParsedMergeCoinResponse(
  data: HaneulTransactionResponse,
): HaneulParsedMergeCoinResponse | undefined {
  const parsed = data.parsed_data;
  return parsed && 'MergeCoin' in parsed ? parsed.MergeCoin : undefined;
}

export function getParsedPublishResponse(
  data: HaneulTransactionResponse,
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
  data: HaneulTransactionResponse,
): HaneulObject | undefined {
  return getParsedMergeCoinResponse(data)?.updatedCoin;
}

/**
 * Get the updated coin after a split.
 * @param data the response for executing a Split coin transaction
 * @returns the updated state of the original coin object used for the split
 */
export function getCoinAfterSplit(
  data: HaneulTransactionResponse,
): HaneulObject | undefined {
  return getParsedSplitCoinResponse(data)?.updatedCoin;
}

/**
 * Get the newly created coin after a split.
 * @param data the response for executing a Split coin transaction
 * @returns the updated state of the original coin object used for the split
 */
export function getNewlyCreatedCoinsAfterSplit(
  data: HaneulTransactionResponse,
): HaneulObject[] | undefined {
  return getParsedSplitCoinResponse(data)?.newCoins;
}

/**
 * Get the newly created coin refs after a split.
 */
export function getNewlyCreatedCoinRefsAfterSplit(
  data: HaneulTransactionResponse | HaneulExecuteTransactionResponse,
): HaneulObjectRef[] | undefined {
  if ('EffectsCert' in data) {
    const effects = data.EffectsCert.effects.effects;
    return effects.created?.map((c) => c.reference);
  }
  if ('effects' in data) {
    const effects =
      'effects' in data.effects ? data.effects.effects : data.effects;
    return effects.created?.map((c) => c.reference);
  }
  return undefined;
}
