// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {ObjectOwner, HaneulAddress, TransactionDigest} from './common';
import {ObjectId, SequenceNumber} from './objects';
import {HaneulJsonValue} from './transactions';

// event types mirror those in "haneul-json-rpc-types/lib.rs"
export type MoveEvent = {
  packageId: ObjectId;
  transactionModule: string;
  sender: HaneulAddress;
  type: string;
  fields?: { [key: string]: any };
  bcs: string;
};

export type PublishEvent = {
  sender: HaneulAddress;
  packageId: ObjectId;
};

export type CoinBalanceChangeEvent = {
  packageId: ObjectId,
  transactionModule: string,
  sender: HaneulAddress,
  owner: ObjectOwner,
  changeType: BalanceChangeType,
  coinType: string,
  coinObjectId: ObjectId,
  version: SequenceNumber,
  amount: number,
};

export type TransferObjectEvent = {
  packageId: ObjectId;
  transactionModule: string;
  sender: HaneulAddress;
  recipient: ObjectOwner;
  objectType: string,
  objectId: ObjectId;
  version: SequenceNumber;
};

export type MutateObjectEvent = {
  packageId: ObjectId;
  transactionModule: string;
  sender: HaneulAddress;
  objectType: string,
  objectId: ObjectId;
  version: SequenceNumber;
};

export type DeleteObjectEvent = {
  packageId: ObjectId;
  transactionModule: string;
  sender: HaneulAddress;
  objectId: ObjectId;
  version: SequenceNumber;
};

export type NewObjectEvent = {
  packageId: ObjectId;
  transactionModule: string;
  sender: HaneulAddress;
  recipient: ObjectOwner;
  objectType: string,
  objectId: ObjectId;
  version: SequenceNumber;
};

export type HaneulEvent =
  | { moveEvent: MoveEvent }
  | { publish: PublishEvent }
  | { coinBalanceChange: CoinBalanceChangeEvent }
  | { transferObject: TransferObjectEvent }
  | { mutateObject: MutateObjectEvent }
  | { deleteObject: DeleteObjectEvent }
  | { newObject: NewObjectEvent }
  | { epochChange: bigint }
  | { checkpoint: bigint };

export type MoveEventField = {
  path: string;
  value: HaneulJsonValue;
};

export type EventQuery =
    | "All"
    | { "Transaction": TransactionDigest }
    | { "MoveModule": { package: ObjectId, module: string } }
    | { "MoveEvent": string }
    | { "EventType": EventType }
    | { "Sender": HaneulAddress }
    | { "Recipient": ObjectOwner }
    | { "Object": ObjectId }
    | { "TimeRange": { "start_time": number, "end_time": number } };

export type EventId = string

export type PaginatedEvents = {
  data: HaneulEvents;
  nextCursor: EventId | null;
};

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

export type BalanceChangeType = "Gas" | "Pay" | "Receive"

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

export type HaneulEventEnvelope = {
  timestamp: number;
  txDigest: TransactionDigest;
  id: EventId;  // tx_seq_num:event_seq
  event: HaneulEvent;
};

export type HaneulEvents = HaneulEventEnvelope[];

export type SubscriptionId = number;

export type SubscriptionEvent = {
  subscription: SubscriptionId;
  result: HaneulEventEnvelope;
};

// mirrors the value defined in https://github.com/GeunhwaJeong/haneul/blob/e12f8c58ef7ba17205c4caf5ad2c350cbb01656c/crates/haneul-json-rpc/src/api.rs#L27
export const EVENT_QUERY_MAX_LIMIT = 100;
export const DEFAULT_START_TIME = 0;
export const DEFAULT_END_TIME = Number.MAX_SAFE_INTEGER;
