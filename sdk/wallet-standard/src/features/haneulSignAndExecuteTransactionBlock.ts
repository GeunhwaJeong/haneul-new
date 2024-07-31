// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
	ExecuteTransactionRequestType,
	HaneulTransactionBlockResponse,
	HaneulTransactionBlockResponseOptions,
} from '@haneullabs/haneul/client';

import type { HaneulSignTransactionBlockInput } from './haneulSignTransactionBlock.js';

/** The latest API version of the signAndExecuteTransactionBlock API. */
export type HaneulSignAndExecuteTransactionBlockVersion = '1.0.0';

/**
 * @deprecated Use `haneul:signAndExecuteTransaction` instead.
 *
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type HaneulSignAndExecuteTransactionBlockFeature = {
	/** Namespace for the feature. */
	'haneul:signAndExecuteTransactionBlock': {
		/** Version of the feature API. */
		version: HaneulSignAndExecuteTransactionBlockVersion;
		/** @deprecated Use `haneul:signAndExecuteTransaction` instead. */
		signAndExecuteTransactionBlock: HaneulSignAndExecuteTransactionBlockMethod;
	};
};

/** @deprecated Use `haneul:signAndExecuteTransaction` instead. */
export type HaneulSignAndExecuteTransactionBlockMethod = (
	input: HaneulSignAndExecuteTransactionBlockInput,
) => Promise<HaneulSignAndExecuteTransactionBlockOutput>;

/** Input for signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionBlockInput extends HaneulSignTransactionBlockInput {
	/**
	 * @deprecated requestType will be ignored by JSON RPC in the future
	 */
	requestType?: ExecuteTransactionRequestType;
	/** specify which fields to return (e.g., transaction, effects, events, etc). By default, only the transaction digest will be returned. */
	options?: HaneulTransactionBlockResponseOptions;
}

/** Output of signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionBlockOutput extends HaneulTransactionBlockResponse {}
