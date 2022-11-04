// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
    CertifiedTransaction,
    ExecutionStatusType,
    HaneulObjectRef,
    HaneulEvent,
    HaneulTransactionResponse,
} from '@haneullabs/haneul.js';

export type DataType = CertifiedTransaction & {
    transaction: HaneulTransactionResponse | null;
    loadState: string;
    txId: string;
    status: ExecutionStatusType;
    gasFee: number;
    txError: string;
    mutated: HaneulObjectRef[];
    created: HaneulObjectRef[];
    events?: HaneulEvent[];
    timestamp_ms: number | null;
};

export type Category =
    | 'objects'
    | 'transactions'
    | 'addresses'
    | 'ethAddress'
    | 'unknown';
