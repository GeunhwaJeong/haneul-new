// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    is,
    HaneulObject,
    type ValidatorsFields,
    type GetObjectDataResponse,
} from '@haneullabs/haneul.js';

export function validatorsFields(
    data?: GetObjectDataResponse
): ValidatorsFields | null {
    return data &&
        is(data.details, HaneulObject) &&
        data.details.data.dataType === 'moveObject'
        ? (data.details.data.fields as ValidatorsFields)
        : null;
}
