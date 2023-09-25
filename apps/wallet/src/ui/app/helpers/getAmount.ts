// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulEvent, HaneulTransactionBlockKind, TransactionEffects } from '@haneullabs/haneul.js/client';

type FormattedBalance = {
	amount?: number | null;
	coinType?: string | null;
	recipientAddress: string;
}[];

export function getAmount(
	_txnData: HaneulTransactionBlockKind,
	_txnEffect: TransactionEffects,
	_events: HaneulEvent[],
): FormattedBalance | null {
	// TODO: Support programmable transactions:
	return null;
}
