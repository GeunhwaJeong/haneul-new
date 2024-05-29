// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
	IdentifierRecord,
	StandardConnectFeature,
	StandardDisconnectFeature,
	StandardEventsFeature,
	WalletWithFeatures,
} from '@wallet-standard/core';

import type { HaneulReportTransactionEffectsFeature } from './haneulReportTransactionEffects.js';
import type { HaneulSignAndExecuteTransactionFeature } from './haneulSignAndExecuteTransaction.js';
import type { HaneulSignAndExecuteTransactionBlockFeature } from './haneulSignAndExecuteTransactionBlock.js';
import type { HaneulSignMessageFeature } from './haneulSignMessage.js';
import type { HaneulSignPersonalMessageFeature } from './haneulSignPersonalMessage.js';
import type { HaneulSignTransactionFeature } from './haneulSignTransaction.js';
import type { HaneulSignTransactionBlockFeature } from './haneulSignTransactionBlock.js';

/**
 * Wallet Standard features that are unique to Haneul, and that all Haneul wallets are expected to implement.
 */
export type HaneulFeatures = Partial<HaneulSignTransactionBlockFeature> &
	Partial<HaneulSignAndExecuteTransactionBlockFeature> &
	HaneulSignPersonalMessageFeature &
	HaneulSignAndExecuteTransactionFeature &
	HaneulSignTransactionFeature &
	// This deprecated feature should be removed once wallets update to the new method:
	Partial<HaneulSignMessageFeature> &
	Partial<HaneulReportTransactionEffectsFeature>;

export type HaneulWalletFeatures = StandardConnectFeature &
	StandardEventsFeature &
	HaneulFeatures &
	// Disconnect is an optional feature:
	Partial<StandardDisconnectFeature>;

export type WalletWithHaneulFeatures = WalletWithFeatures<HaneulWalletFeatures>;

/**
 * Represents a wallet with the absolute minimum feature set required to function in the Haneul ecosystem.
 */
export type WalletWithRequiredFeatures = WalletWithFeatures<
	MinimallyRequiredFeatures &
		Partial<HaneulFeatures> &
		Partial<StandardDisconnectFeature> &
		IdentifierRecord<unknown>
>;

export type MinimallyRequiredFeatures = StandardConnectFeature & StandardEventsFeature;

export * from './haneulSignMessage.js';
export * from './haneulSignTransactionBlock.js';
export * from './haneulSignTransaction.js';
export * from './haneulSignAndExecuteTransactionBlock.js';
export * from './haneulSignAndExecuteTransaction.js';
export * from './haneulSignPersonalMessage.js';
export * from './haneulReportTransactionEffects.js';
