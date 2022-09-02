// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulAddress, ObjectOwner } from "./common";
import { ObjectId, SequenceNumber } from "./objects";


// event types mirror those in "haneul-json-rpc-types/lib.rs"
export type MoveEvent = {
    packageId: ObjectId;
    transactionModule: string;
    sender: HaneulAddress;
    type: string;
    fields: { [key: string]: any; }; // TODO - better type
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
