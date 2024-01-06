// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { TransactionBlock } from '@haneullabs/haneul.js/transactions';
import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signTransactionBlock API. */
export type HaneulSignTransactionBlockVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type HaneulSignTransactionBlockFeature = {
	/** Namespace for the feature. */
	'haneul:signTransactionBlock': {
		/** Version of the feature API. */
		version: HaneulSignTransactionBlockVersion;
		signTransactionBlock: HaneulSignTransactionBlockMethod;
	};
};

export type HaneulSignTransactionBlockMethod = (
	input: HaneulSignTransactionBlockInput,
) => Promise<HaneulSignTransactionBlockOutput>;

/** Input for signing transactions. */
export interface HaneulSignTransactionBlockInput {
	transactionBlock: TransactionBlock;
	account: WalletAccount;
	chain: IdentifierString;
}

/** Output of signing transactions. */
export interface HaneulSignTransactionBlockOutput extends SignedTransactionBlock {}

export interface SignedTransactionBlock {
	transactionBlockBytes: string;
	signature: string;
}
