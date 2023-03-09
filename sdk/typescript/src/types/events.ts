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
  object({ moveEvent: MoveEvent }),
  object({ publish: PublishEvent }),
  object({ coinBalanceChange: CoinBalanceChangeEvent }),
  object({ transferObject: TransferObjectEvent }),
  object({ mutateObject: MutateObjectEvent }),
  object({ deleteObject: DeleteObjectEvent }),
  object({ newObject: NewObjectEvent }),
  object({ epochChange: EpochChangeEvent }),
  object({ checkpoint: CheckpointEvent }),
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
