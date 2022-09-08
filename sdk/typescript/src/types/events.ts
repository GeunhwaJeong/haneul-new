// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulAddress, ObjectOwner, TransactionDigest } from "./common";
import { ObjectId, SequenceNumber } from "./objects";
import { HaneulJsonValue } from "./transactions";


// event types mirror those in "haneul-json-rpc-types/lib.rs"
export type MoveEvent = {
    packageId: ObjectId;
    transactionModule: string;
    sender: HaneulAddress;
    type: string;
    fields: { [key: string]: any; };
    bcs: string;
};

export type PublishEvent = {
    sender: HaneulAddress;
    packageId: ObjectId;
};

export type TransferObjectEvent = {
    packageId: ObjectId;
    transactionModule: string;
    sender: HaneulAddress;
    recipient: ObjectOwner;
    objectId: ObjectId;
    version: SequenceNumber;
    type: string; // TODO - better type
    amount: number | null;
};

export type DeleteObjectEvent = {
    packageId: ObjectId;
    transactionModule: string;
    sender: HaneulAddress;
    objectId: ObjectId;
};

export type NewObjectEvent = {
    packageId: ObjectId;
    transactionModule: string;
    sender: HaneulAddress;
    recipient: ObjectOwner;
    objectId: ObjectId;
};

export type HaneulEvent =
    | { moveEvent: MoveEvent }
    | { publish: PublishEvent }
    | { transferObject: TransferObjectEvent }
    | { deleteObject: DeleteObjectEvent }
    | { newObject: NewObjectEvent }
    | { epochChange: bigint }
    | { checkpoint: bigint };

export type MoveEventField = {
    path: string,
    value: HaneulJsonValue
}

export type EventType =
    | "MoveEvent"
    | "Publish"
    | "TransferObject"
    | "DeleteObject"
    | "NewObject"
    | "EpochChange"
    | "Checkpoint";

// mirrors haneul_json_rpc_types::HaneulEventFilter
export type HaneulEventFilter =
    | { "Package" : ObjectId }
    | { "Module" : string }
    | { "MoveEventType" : string }
    | { "MoveEventField" : MoveEventField }
    | { "SenderAddress" : HaneulAddress }
    | { "EventType" : EventType }
    | { "All" : HaneulEventFilter[] }
    | { "Any" : HaneulEventFilter[] }
    | { "And" : [HaneulEventFilter, HaneulEventFilter] }
    | { "Or" : [HaneulEventFilter, HaneulEventFilter] };

export type HaneulEventEnvelope = {
    timestamp:  number,
    txDigest: TransactionDigest,
    event: HaneulEvent
}

export type SubscriptionId = number;

export type SubscriptionEvent = { subscription: SubscriptionId, result: HaneulEventEnvelope };