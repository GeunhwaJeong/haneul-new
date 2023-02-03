// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    is,
    HaneulObject,
    type MoveHaneulSystemObjectFields,
    type GetObjectDataResponse,
} from '@haneullabs/haneul.js';

export function validatorsFields(
    data?: GetObjectDataResponse
): MoveHaneulSystemObjectFields | null {
    return data &&
        is(data.details, HaneulObject) &&
        data.details.data.dataType === 'moveObject'
        ? (data.details.data.fields as MoveHaneulSystemObjectFields)
        : null;
}
