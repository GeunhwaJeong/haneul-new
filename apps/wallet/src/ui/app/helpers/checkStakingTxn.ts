// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulTransactionResponse } from '@haneullabs/haneul.js';

// TODO: Support programmable transactions:
export function checkStakingTxn(_txn: HaneulTransactionResponse) {
    return false;
}
