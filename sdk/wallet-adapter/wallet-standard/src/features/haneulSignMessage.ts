// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { SignedMessage } from '@haneullabs/haneul.js';
import type { WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signMessage API. */
export type HaneulSignMessageVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a personal message, and returning the
 * message bytes that were signed, and message signature.
 */
export type HaneulSignMessageFeature = {
	/** Namespace for the feature. */
	'haneul:signMessage': {
		/** Version of the feature API. */
		version: HaneulSignMessageVersion;
		signMessage: HaneulSignMessageMethod;
	};
};

export type HaneulSignMessageMethod = (input: HaneulSignMessageInput) => Promise<HaneulSignMessageOutput>;

/** Input for signing messages. */
export interface HaneulSignMessageInput {
	message: Uint8Array;
	account: WalletAccount;
}

/** Output of signing messages. */
export interface HaneulSignMessageOutput extends SignedMessage {}
