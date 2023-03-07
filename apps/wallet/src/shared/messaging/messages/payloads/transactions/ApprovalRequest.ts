// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    type HaneulSignMessageOutput,
    type HaneulSignMessageOptions,
    type HaneulSignAndExecuteTransactionOptions,
} from '@haneullabs/wallet-standard';

import type {
    MoveCallTransaction,
    SignableTransaction,
    SignedTransaction,
    HaneulAddress,
    HaneulMoveNormalizedFunction,
    HaneulTransactionResponse,
    UnserializedSignableTransaction,
} from '@haneullabs/haneul.js';

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

export type SignMessageDataType = {
    type: 'sign-message';
    message: string;
    accountAddress: HaneulAddress;
    options?: HaneulSignMessageOptions;
};

export type ApprovalRequest = {
    id: string;
    approved: boolean | null;
    origin: string;
    originFavIcon?: string;
    txResult?: HaneulTransactionResponse | HaneulSignMessageOutput;
    txResultError?: string;
    txSigned?: SignedTransaction;
    metadata?: HaneulMoveNormalizedFunction;
    createdDate: string;
    tx: TransactionDataType | SignMessageDataType;
    unSerializedTxn?: UnserializedSignableTransaction | null;
};

export interface SignMessageApprovalRequest
    extends Omit<ApprovalRequest, 'txResult' | 'tx'> {
    tx: SignMessageDataType;
    txResult?: HaneulSignMessageOutput;
}

export interface TransactionApprovalRequest
    extends Omit<ApprovalRequest, 'txResult' | 'tx'> {
    tx: TransactionDataType;
    txResult?: HaneulTransactionResponse;
}

export function isSignMessageApprovalRequest(
    request: ApprovalRequest
): request is SignMessageApprovalRequest {
    return request.tx.type === 'sign-message';
}

export function isTransactionApprovalRequest(
    request: ApprovalRequest
): request is TransactionApprovalRequest {
    return request.tx.type !== 'sign-message';
}
