// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    type HaneulSignAndExecuteTransactionBlockInput,
    type HaneulSignMessageOutput,
} from '@haneullabs/wallet-standard';

import type {
    SignedTransaction,
    HaneulAddress,
    HaneulTransactionResponse,
} from '@haneullabs/haneul.js';

export type TransactionDataType = {
    type: 'transaction';
    data: string;
    account: HaneulAddress;
    justSign?: boolean;
    requestType?: HaneulSignAndExecuteTransactionBlockInput['requestType'];
    options?: HaneulSignAndExecuteTransactionBlockInput['options'];
};

export type SignMessageDataType = {
    type: 'sign-message';
    message: string;
    accountAddress: HaneulAddress;
};

export type ApprovalRequest = {
    id: string;
    approved: boolean | null;
    origin: string;
    originFavIcon?: string;
    txResult?: HaneulTransactionResponse | HaneulSignMessageOutput;
    txResultError?: string;
    txSigned?: SignedTransaction;
    createdDate: string;
    tx: TransactionDataType | SignMessageDataType;
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
