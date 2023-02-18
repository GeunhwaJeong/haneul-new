// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulSignTransactionOutput } from '@haneullabs/wallet-standard';

import { isBasePayload } from '_payloads';

import type { HaneulTransactionResponse } from '@haneullabs/haneul.js';
import type { BasePayload, Payload } from '_payloads';

export interface ExecuteTransactionResponse extends BasePayload {
    type: 'execute-transaction-response';
    result: HaneulTransactionResponse;
}

export function isExecuteTransactionResponse(
    payload: Payload
): payload is ExecuteTransactionResponse {
    return (
        isBasePayload(payload) &&
        payload.type === 'execute-transaction-response'
    );
}

export interface SignTransactionResponse extends BasePayload {
    type: 'sign-transaction-response';
    result: HaneulSignTransactionOutput;
}

export function isSignTransactionResponse(
    payload: Payload
): payload is SignTransactionResponse {
    return (
        isBasePayload(payload) && payload.type === 'sign-transaction-response'
    );
}
