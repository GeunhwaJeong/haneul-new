// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isBasePayload } from '_payloads';

import type { TransactionResponse } from '@haneullabs/haneul.js';
import type { BasePayload, Payload } from '_payloads';

export interface ExecuteTransactionResponse extends BasePayload {
    type: 'execute-transaction-response';
    result: TransactionResponse;
}

export function isExecuteTransactionResponse(
    payload: Payload
): payload is ExecuteTransactionResponse {
    return (
        isBasePayload(payload) &&
        payload.type === 'execute-transaction-response'
    );
}
