// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { type HaneulAddress } from '@haneullabs/haneul.js';

import { BalanceChangeSummary } from './getBalanceChangeSummary';
import { GasSummaryType } from './getGasSummary';
import { ObjectChangeSummary } from './getObjectChangeSummary';

export type TransactionSummary = {
	digest?: string;
	sender?: HaneulAddress;
	timestamp?: string;
	balanceChanges: BalanceChangeSummary;
	gas?: GasSummaryType;
	objectSummary: ObjectChangeSummary | null;
} | null;

export type HaneulObjectChangeTypes =
	| 'published'
	| 'transferred'
	| 'mutated'
	| 'deleted'
	| 'wrapped'
	| 'created';
