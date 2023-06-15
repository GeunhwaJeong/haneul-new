// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
	HaneulTransactionBlockKind,
	TransactionEffects,
	TransactionEvents,
} from '@haneullabs/haneul.js';

type FormattedBalance = {
	amount?: number | null;
	coinType?: string | null;
	recipientAddress: string;
}[];

export function getAmount(
	_txnData: HaneulTransactionBlockKind,
	_txnEffect: TransactionEffects,
	_events: TransactionEvents,
): FormattedBalance | null {
	// TODO: Support programmable transactions:
	return null;
}
