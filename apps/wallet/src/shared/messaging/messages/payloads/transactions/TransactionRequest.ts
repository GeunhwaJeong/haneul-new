// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
    SignedTransaction,
    HaneulMoveNormalizedFunction,
    HaneulTransactionResponse,
    UnserializedSignableTransaction,
} from '@haneullabs/haneul.js';
import type { TransactionDataType } from '_messages/payloads/transactions/ExecuteTransactionRequest';

export type TransactionRequest = {
    id: string;
    approved: boolean | null;
    origin: string;
    originFavIcon?: string;
    txResult?: HaneulTransactionResponse;
    txResultError?: string;
    txSigned?: SignedTransaction;
    metadata?: HaneulMoveNormalizedFunction;
    createdDate: string;
    tx: TransactionDataType;
    unSerializedTxn?: UnserializedSignableTransaction | null;
};
