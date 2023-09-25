// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
	ExecuteTransactionRequestType,
	HaneulTransactionBlockResponse,
	HaneulTransactionBlockResponseOptions,
} from '@haneullabs/haneul.js/client';

import type { HaneulSignTransactionBlockInput } from './haneulSignTransactionBlock';

/** The latest API version of the signAndExecuteTransactionBlock API. */
export type HaneulSignAndExecuteTransactionBlockVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type HaneulSignAndExecuteTransactionBlockFeature = {
	/** Namespace for the feature. */
	'haneul:signAndExecuteTransactionBlock': {
		/** Version of the feature API. */
		version: HaneulSignAndExecuteTransactionBlockVersion;
		signAndExecuteTransactionBlock: HaneulSignAndExecuteTransactionBlockMethod;
	};
};

export type HaneulSignAndExecuteTransactionBlockMethod = (
	input: HaneulSignAndExecuteTransactionBlockInput,
) => Promise<HaneulSignAndExecuteTransactionBlockOutput>;

/** Input for signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionBlockInput extends HaneulSignTransactionBlockInput {
	/**
	 * `WaitForEffectsCert` or `WaitForLocalExecution`, see details in `ExecuteTransactionRequestType`.
	 * Defaults to `WaitForLocalExecution` if options.showEffects or options.showEvents is true
	 */
	requestType?: ExecuteTransactionRequestType;
	/** specify which fields to return (e.g., transaction, effects, events, etc). By default, only the transaction digest will be returned. */
	options?: HaneulTransactionBlockResponseOptions;
}

/** Output of signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionBlockOutput extends HaneulTransactionBlockResponse {}
