// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { Transaction } from '@haneullabs/haneul/transactions';
import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signTransactionBlock API. */
export type HaneulSignTransactionBlockVersion = '1.0.0';

/**
 * @deprecated Use `haneul:signTransaction` instead.
 *
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type HaneulSignTransactionBlockFeature = {
	/** Namespace for the feature. */
	'haneul:signTransactionBlock': {
		/** Version of the feature API. */
		version: HaneulSignTransactionBlockVersion;
		/** @deprecated Use `haneul:signTransaction` instead. */
		signTransactionBlock: HaneulSignTransactionBlockMethod;
	};
};

/** @deprecated Use `haneul:signTransaction` instead. */
export type HaneulSignTransactionBlockMethod = (
	input: HaneulSignTransactionBlockInput,
) => Promise<HaneulSignTransactionBlockOutput>;

/** Input for signing transactions. */
export interface HaneulSignTransactionBlockInput {
	transactionBlock: Transaction;
	account: WalletAccount;
	chain: IdentifierString;
}

/** Output of signing transactions. */
export interface HaneulSignTransactionBlockOutput extends SignedTransactionBlock {}

export interface SignedTransactionBlock {
	/** Transaction as base64 encoded bcs. */
	transactionBlockBytes: string;
	/** Base64 encoded signature */
	signature: string;
}
