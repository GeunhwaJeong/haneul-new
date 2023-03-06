// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    type HaneulSignTransactionInput,
    type HaneulSignAndExecuteTransactionOptions,
} from '@haneullabs/wallet-standard';

import { isBasePayload } from '_payloads';

import type {
    MoveCallTransaction,
    SignableTransaction,
    HaneulAddress,
} from '@haneullabs/haneul.js';
import type { BasePayload, Payload } from '_payloads';

export type TransactionDataType =
    | {
          type: 'v2';
          justSign?: boolean;
          //   TODO: Support transaciton builder string
          //   data: SignableTransaction | string;
          data: SignableTransaction;
          options?: HaneulSignAndExecuteTransactionOptions;
          account: HaneulAddress;
      }
    | { type: 'move-call'; data: MoveCallTransaction; account: HaneulAddress }
    | { type: 'serialized-move-call'; data: string; account: HaneulAddress };

export interface ExecuteTransactionRequest extends BasePayload {
    type: 'execute-transaction-request';
    transaction: TransactionDataType;
}

export function isExecuteTransactionRequest(
    payload: Payload
): payload is ExecuteTransactionRequest {
    return (
        isBasePayload(payload) && payload.type === 'execute-transaction-request'
    );
}

export type HaneulSignTransactionSerialized = Omit<
    HaneulSignTransactionInput,
    'transaction' | 'account'
> & {
    // TODO: Enable string for serialized `Transaction`
    // transaction: SignableTransaction | string;
    transaction: SignableTransaction;
    account: HaneulAddress;
};

export interface SignTransactionRequest extends BasePayload {
    type: 'sign-transaction-request';
    transaction: HaneulSignTransactionSerialized;
}

export function isSignTransactionRequest(
    payload: Payload
): payload is SignTransactionRequest {
    return (
        isBasePayload(payload) && payload.type === 'sign-transaction-request'
    );
}
