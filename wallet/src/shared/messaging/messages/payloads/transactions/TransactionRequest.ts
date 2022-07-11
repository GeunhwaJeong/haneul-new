// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { MoveCallTransaction, TransactionResponse } from '@haneullabs/haneul.js';

export type TransactionRequest = {
    id: string;
    approved: boolean | null;
    origin: string;
    originFavIcon?: string;
    txResult?: TransactionResponse;
    txResultError?: string;
    createdDate: string;
} & (
    | {
          type: 'move-call';
          tx: MoveCallTransaction;
      }
    | {
          type: 'serialized-move-call';
          txBytes: Uint8Array;
      }
);
