// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { SignedTransaction, HaneulSignTransactionInput } from './haneulSignTransaction.js';

/** The latest API version of the signAndExecuteTransactionBlock API. */
export type HaneulSignAndExecuteTransactionVersion = '2.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type HaneulSignAndExecuteTransactionFeature = {
	/** Namespace for the feature. */
	'haneul:signAndExecuteTransaction': {
		/** Version of the feature API. */
		version: HaneulSignAndExecuteTransactionVersion;
		signAndExecuteTransaction: HaneulSignAndExecuteTransactionMethod;
	};
};

export type HaneulSignAndExecuteTransactionMethod = (
	input: HaneulSignAndExecuteTransactionInput,
) => Promise<HaneulSignAndExecuteTransactionOutput>;

/** Input for signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionInput extends HaneulSignTransactionInput {}

/** Output of signing and sending transactions. */
export interface HaneulSignAndExecuteTransactionOutput extends SignedTransaction {
	digest: string;
	/** Transaction effects as base64 encoded bcs. */
	effects: string;
}
