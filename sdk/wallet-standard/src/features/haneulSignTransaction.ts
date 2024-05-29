// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signTransaction API. */
export type HaneulSignTransactionVersion = '2.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type HaneulSignTransactionFeature = {
	/** Namespace for the feature. */
	'haneul:signTransaction': {
		/** Version of the feature API. */
		version: HaneulSignTransactionVersion;
		signTransaction: HaneulSignTransactionMethod;
	};
};

export type HaneulSignTransactionMethod = (
	input: HaneulSignTransactionInput,
) => Promise<SignedTransaction>;

/** Input for signing transactions. */
export interface HaneulSignTransactionInput {
	transaction: { toJSON: () => Promise<string> };
	account: WalletAccount;
	chain: IdentifierString;
	signal?: AbortSignal;
}

/** Output of signing transactions. */

export interface SignedTransaction {
	/** Transaction as base64 encoded bcs. */
	bytes: string;
	/** Base64 encoded signature */
	signature: string;
}
