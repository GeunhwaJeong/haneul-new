// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  object,
  number,
  string,
  bigint,
  union,
  literal,
  Infer,
  array,
  record,
  any,
  optional,
  boolean,
} from 'superstruct';
import {
  ObjectId,
  ObjectOwner,
  HaneulAddress,
  TransactionDigest,
  HaneulJsonValue,
  SequenceNumber,
} from './common';

export const BalanceChangeType = union([
  literal('Gas'),
  literal('Pay'),
  literal('Receive'),
]);

export type BalanceChangeType = Infer<typeof BalanceChangeType>;

// event types mirror those in "haneul-json-rpc-types/lib.rs"
export const MoveEvent = object({
  packageId: ObjectId,
  transactionModule: string(),
  sender: HaneulAddress,
  type: string(),
  fields: record(string(), any()),
  bcs: string(),
});

export type MoveEvent = Infer<typeof MoveEvent>;

export const PublishEvent = object({
  sender: HaneulAddress,
  packageId: ObjectId,
  version: optional(number()),
  digest: optional(string()),
});

export type PublishEvent = Infer<typeof PublishEvent>;

export const CoinBalanceChangeEvent = object({
  packageId: ObjectId,
  transactionModule: string(),
  sender: HaneulAddress,
  owner: ObjectOwner,
  changeType: BalanceChangeType,
  coinType: string(),
  coinObjectId: ObjectId,
  version: SequenceNumber,
  amount: number(),
});

export type CoinBalanceChangeEvent = Infer<typeof CoinBalanceChangeEvent>;

export const TransferObjectEvent = object({
  packageId: ObjectId,
  transactionModule: string(),
  sender: HaneulAddress,
  recipient: ObjectOwner,
  objectType: string(),
  objectId: ObjectId,
  version: SequenceNumber,
});

export type TransferObjectEvent = Infer<typeof TransferObjectEvent>;

export const MutateObjectEvent = object({
  packageId: ObjectId,
  transactionModule: string(),
  sender: HaneulAddress,
  objectType: string(),
  objectId: ObjectId,
  version: SequenceNumber,
});

export type MutateObjectEvent = Infer<typeof MutateObjectEvent>;

export const DeleteObjectEvent = object({
  packageId: ObjectId,
  transactionModule: string(),
  sender: HaneulAddress,
  objectId: ObjectId,
  version: SequenceNumber,
});

export type DeleteObjectEvent = Infer<typeof DeleteObjectEvent>;

export const NewObjectEvent = object({
  packageId: ObjectId,
  transactionModule: string(),
  sender: HaneulAddress,
  recipient: ObjectOwner,
  objectType: string(),
  objectId: ObjectId,
  version: SequenceNumber,
});

export type NewObjectEvent = Infer<typeof NewObjectEvent>;

// TODO: Figure out if these actually can be bigint:
export const EpochChangeEvent = union([bigint(), number()]);
export type EpochChangeEvent = Infer<typeof EpochChangeEvent>;

export const CheckpointEvent = union([bigint(), number()]);
export type CheckpointEvent = Infer<typeof EpochChangeEvent>;

export const HaneulEvent = union([
  object({ type: literal('moveEvent'), content: MoveEvent }),
  object({ type: literal('publish'), content: PublishEvent }),
  object({
    type: literal('coinBalanceChange'),
    content: CoinBalanceChangeEvent,
  }),
  object({ type: literal('transferObject'), content: TransferObjectEvent }),
  object({ type: literal('mutateObject'), content: MutateObjectEvent }),
  object({ type: literal('deleteObject'), content: DeleteObjectEvent }),
  object({ type: literal('newObject'), content: NewObjectEvent }),
  object({ type: literal('epochChange'), content: EpochChangeEvent }),
  object({ type: literal('checkpoint'), content: CheckpointEvent }),
]);
export type HaneulEvent = Infer<typeof HaneulEvent>;

export type MoveEventField = {
  path: string;
  value: HaneulJsonValue;
};

export type EventQuery =
  | 'All'
  | { Transaction: TransactionDigest }
  | { MoveModule: { package: ObjectId; module: string } }
  | { MoveEvent: string }
  | { EventType: EventType }
  | { Sender: HaneulAddress }
  | { Recipient: ObjectOwner }
  | { Object: ObjectId }
  | { TimeRange: { start_time: number; end_time: number } };

export const EventId = object({
  txDigest: TransactionDigest,
  eventSeq: number(),
});

export type EventId = Infer<typeof EventId>;

export type EventType =
  | 'MoveEvent'
  | 'Publish'
  | 'TransferObject'
  | 'MutateObject'
  | 'CoinBalanceChange'
  | 'DeleteObject'
  | 'NewObject'
  | 'EpochChange'
  | 'Checkpoint';

// mirrors haneul_json_rpc_types::HaneulEventFilter
export type HaneulEventFilter =
  | { Package: ObjectId }
  | { Module: string }
  | { MoveEventType: string }
  | { MoveEventField: MoveEventField }
  | { SenderAddress: HaneulAddress }
  | { EventType: EventType }
  | { All: HaneulEventFilter[] }
  | { Any: HaneulEventFilter[] }
  | { And: [HaneulEventFilter, HaneulEventFilter] }
  | { Or: [HaneulEventFilter, HaneulEventFilter] };

export const HaneulEventEnvelope = object({
  timestamp: number(),
  txDigest: TransactionDigest,
  id: EventId, // tx_digest:event_seq
  event: HaneulEvent,
});

export type HaneulEventEnvelope = Infer<typeof HaneulEventEnvelope>;

export type HaneulEvents = HaneulEventEnvelope[];

export const PaginatedEvents = object({
  data: array(HaneulEventEnvelope),
  nextCursor: union([EventId, literal(null)]),
  hasNextPage: boolean(),
});
export type PaginatedEvents = Infer<typeof PaginatedEvents>;

export const SubscriptionId = number();

export type SubscriptionId = Infer<typeof SubscriptionId>;

export const SubscriptionEvent = object({
  subscription: SubscriptionId,
  result: HaneulEventEnvelope,
});

export type SubscriptionEvent = Infer<typeof SubscriptionEvent>;

/* ------------------------------- EventData ------------------------------ */

export function getMoveEvent(event: HaneulEvent): MoveEvent | undefined {
  return event.type === 'moveEvent' ? event.content : undefined;
}

export function getPublishEvent(event: HaneulEvent): PublishEvent | undefined {
  return event.type === 'publish' ? event.content : undefined;
}

export function getCoinBalanceChangeEvent(
  event: HaneulEvent,
): CoinBalanceChangeEvent | undefined {
  return event.type === 'coinBalanceChange' ? event.content : undefined;
}

export function getTransferObjectEvent(
  event: HaneulEvent,
): TransferObjectEvent | undefined {
  return event.type === 'transferObject' ? event.content : undefined;
}

export function getMutateObjectEvent(
  event: HaneulEvent,
): MutateObjectEvent | undefined {
  return event.type === 'mutateObject' ? event.content : undefined;
}

export function getDeletObjectEvent(
  event: HaneulEvent,
): DeleteObjectEvent | undefined {
  return event.type === 'deleteObject' ? event.content : undefined;
}

export function getNewObjectEvent(event: HaneulEvent): NewObjectEvent | undefined {
  return event.type === 'newObject' ? event.content : undefined;
}

export function getEpochChangeEvent(
  event: HaneulEvent,
): EpochChangeEvent | undefined {
  return event.type === 'epochChange' ? event.content : undefined;
}

export function getCheckpointEvent(
  event: HaneulEvent,
): CheckpointEvent | undefined {
  return event.type === 'checkpoint' ? event.content : undefined;
}

export function getEventSender(event: HaneulEvent): HaneulAddress | undefined {
  return event.type !== 'epochChange' && event.type !== 'checkpoint'
    ? event.content.sender
    : undefined;
}

export function getEventPackage(event: HaneulEvent): ObjectId | undefined {
  return event.type !== 'epochChange' && event.type !== 'checkpoint'
    ? event.content.packageId
    : undefined;
}

export function isEventType(
  e: HaneulEvent,
  type:
    | 'moveEvent'
    | 'publish'
    | 'transferObject'
    | 'mutateObject'
    | 'coinBalanceChange'
    | 'deleteObject'
    | 'newObject'
    | 'epochChange'
    | 'checkpoint',
): boolean {
  return e.type === type;
}
