// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  object,
  number,
  string,
  Infer,
  array,
  record,
  any,
  optional,
  boolean,
  nullable,
} from 'superstruct';
import {
  ObjectId,
  HaneulAddress,
  TransactionDigest,
  HaneulJsonValue,
  SequenceNumber,
} from './common';

export const EventId = object({
  txDigest: TransactionDigest,
  eventSeq: SequenceNumber,
});

// event types mirror those in "haneul-json-rpc-types/src/haneul_event.rs"

export const HaneulEvent = object({
  id: EventId,
  // Move package where this event was emitted.
  packageId: ObjectId,
  // Move module where this event was emitted.
  transactionModule: string(),
  // Sender's Haneul address.
  sender: HaneulAddress,
  // Move event type.
  type: string(),
  // Parsed json value of the event
  parsedJson: optional(record(string(), any())),
  // Base 58 encoded bcs bytes of the move event
  bcs: optional(string()),
  timestampMs: optional(string()),
});

export type HaneulEvent = Infer<typeof HaneulEvent>;

export type MoveEventField = {
  path: string;
  value: HaneulJsonValue;
};

/**
 * Sequential event ID, ie (transaction seq number, event seq number).
 * 1) Serves as a unique event ID for each fullnode
 * 2) Also serves to sequence events for the purposes of pagination and querying.
 *    A higher id is an event seen later by that fullnode.
 * This ID is the "cursor" for event querying.
 */
export type EventId = Infer<typeof EventId>;

// mirrors haneul_json_rpc_types::HaneulEventFilter
export type HaneulEventFilter =
  | { Package: ObjectId }
  | { MoveModule: { package: ObjectId; module: string } }
  | { MoveEventType: string }
  | { MoveEventField: MoveEventField }
  | { Transaction: TransactionDigest }
  | {
      TimeRange: {
        // left endpoint of time interval, milliseconds since epoch, inclusive
        start_time: number;
        // right endpoint of time interval, milliseconds since epoch, exclusive
        end_time: number;
      };
    }
  | { Sender: HaneulAddress }
  | { All: HaneulEventFilter[] }
  | { Any: HaneulEventFilter[] }
  | { And: [HaneulEventFilter, HaneulEventFilter] }
  | { Or: [HaneulEventFilter, HaneulEventFilter] };

export const PaginatedEvents = object({
  data: array(HaneulEvent),
  nextCursor: nullable(EventId),
  hasNextPage: boolean(),
});
export type PaginatedEvents = Infer<typeof PaginatedEvents>;

export const SubscriptionId = number();

export type SubscriptionId = Infer<typeof SubscriptionId>;

export const SubscriptionEvent = object({
  subscription: SubscriptionId,
  result: HaneulEvent,
});

export type SubscriptionEvent = Infer<typeof SubscriptionEvent>;

/* ------------------------------- EventData ------------------------------ */

export function getEventSender(event: HaneulEvent): HaneulAddress {
  return event.sender;
}

export function getEventPackage(event: HaneulEvent): ObjectId {
  return event.packageId;
}
