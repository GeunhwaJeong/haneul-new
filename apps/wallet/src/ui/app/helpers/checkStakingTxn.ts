// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulTransactionBlockResponse } from '@haneullabs/haneul/client';

// TODO: Support programmable transactions:
export function checkStakingTxn(_txn: HaneulTransactionBlockResponse) {
	return false;
}
