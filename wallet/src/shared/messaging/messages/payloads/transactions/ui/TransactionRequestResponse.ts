// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isBasePayload } from '_payloads';

import type { HaneulTransactionResponse } from '@haneullabs/haneul.js';
import type { BasePayload, Payload } from '_payloads';

export interface TransactionRequestResponse extends BasePayload {
    type: 'transaction-request-response';
    txID: string;
    approved: boolean;
    txResult?: HaneulTransactionResponse;
    tsResultError?: string;
}

export function isTransactionRequestResponse(
    payload: Payload
): payload is TransactionRequestResponse {
    return (
        isBasePayload(payload) &&
        payload.type === 'transaction-request-response'
    );
}
