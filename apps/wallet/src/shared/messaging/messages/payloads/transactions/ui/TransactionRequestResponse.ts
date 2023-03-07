// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulSignMessageOutput } from '@haneullabs/wallet-standard';

import { isBasePayload } from '_payloads';

import type { SignedTransaction, HaneulTransactionResponse } from '@haneullabs/haneul.js';
import type { BasePayload, Payload } from '_payloads';

export interface TransactionRequestResponse extends BasePayload {
    type: 'transaction-request-response';
    txID: string;
    approved: boolean;
    txResult?: HaneulTransactionResponse | HaneulSignMessageOutput;
    txResultError?: string;
    txSigned?: SignedTransaction;
}

export function isTransactionRequestResponse(
    payload: Payload
): payload is TransactionRequestResponse {
    return (
        isBasePayload(payload) &&
        payload.type === 'transaction-request-response'
    );
}
