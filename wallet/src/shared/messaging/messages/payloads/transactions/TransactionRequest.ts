// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
    HaneulMoveNormalizedFunction,
    HaneulTransactionResponse,
} from '@haneullabs/haneul.js';
import type { TransactionDataType } from '_messages/payloads/transactions/ExecuteTransactionRequest';

export type TransactionRequest = {
    id: string;
    approved: boolean | null;
    origin: string;
    originFavIcon?: string;
    txResult?: HaneulTransactionResponse;
    txResultError?: string;
    metadata?: HaneulMoveNormalizedFunction;
    createdDate: string;
    tx: TransactionDataType;
};
