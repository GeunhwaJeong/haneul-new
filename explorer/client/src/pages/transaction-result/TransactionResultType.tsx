// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
    CertifiedTransaction,
    ExecutionStatusType,
    HaneulObjectRef,
} from '@haneullabs/haneul.js';

export type DataType = CertifiedTransaction & {
    loadState: string;
    txId: string;
    status: ExecutionStatusType;
    gasFee: number;
    txError: string;
    mutated: HaneulObjectRef[];
    created: HaneulObjectRef[];
    timestamp_ms: number | null;
};

export type Category =
    | 'objects'
    | 'transactions'
    | 'addresses'
    | 'ethAddress'
    | 'unknown';
