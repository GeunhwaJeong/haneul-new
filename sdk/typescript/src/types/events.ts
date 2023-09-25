// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { Infer } from 'superstruct';
import { any, array, boolean, nullable, object, optional, record, string } from 'superstruct';

import type { HaneulJsonValue } from './common.js';

export const EventId = object({
	txDigest: string(),
	eventSeq: string(),
});

// event types mirror those in "haneul-json-rpc-types/src/haneul_event.rs"

export const HaneulEvent = object({
	id: EventId,
	// Move package where this event was emitted.
	packageId: string(),
	// Move module where this event was emitted.
	transactionModule: string(),
	// Sender's Haneul address.
	sender: string(),
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
	| { Package: string }
	| { MoveModule: { package: string; module: string } }
	| { MoveEventType: string }
	| { MoveEventField: MoveEventField }
	| { Transaction: string }
	| {
			TimeRange: {
				// left endpoint of time interval, milliseconds since epoch, inclusive
				startTime: string;
				// right endpoint of time interval, milliseconds since epoch, exclusive
				endTime: string;
			};
	  }
	| { Sender: string }
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

/* ------------------------------- EventData ------------------------------ */

export function getEventSender(event: HaneulEvent): string {
	return event.sender;
}

export function getEventPackage(event: HaneulEvent): string {
	return event.packageId;
}
