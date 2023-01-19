// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// This file implements `haneul_getRawObject` RPC call to
// speed up data processing and lessen network load by using BCS

import { ObjectOwner, ObjectStatus, Provider, HaneulObjectRef } from "@haneullabs/haneul.js";

/**
 * Filling in the missing piece in TS SDK.
 */
export type RawObjectResponse = {
    status: ObjectStatus;
    details: {
        reference: HaneulObjectRef;
        owner: ObjectOwner;
        data: {
            /* ... some other fields */
            bcs_bytes: string
        },
    }
};

/**
 * Object data fetching result.
 * Contains both the reference to use in txs and the data.
 */
export type ObjectData<T> = {
    reference: HaneulObjectRef;
    data: T;
};

/**
 * Wraps the `haneul_getRawObject` method.
 */
export function getRawObject(provider: Provider, objectId: string): Promise<RawObjectResponse> {
    return provider.call('haneul_getRawObject', [ objectId ]);
}
