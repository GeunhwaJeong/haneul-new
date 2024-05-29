// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signPersonalMessage API. */
export type HaneulSignPersonalMessageVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a personal message, and returning the
 * message bytes that were signed, and message signature.
 */
export type HaneulSignPersonalMessageFeature = {
	/** Namespace for the feature. */
	'haneul:signPersonalMessage': {
		/** Version of the feature API. */
		version: HaneulSignPersonalMessageVersion;
		signPersonalMessage: HaneulSignPersonalMessageMethod;
	};
};

export type HaneulSignPersonalMessageMethod = (
	input: HaneulSignPersonalMessageInput,
) => Promise<HaneulSignPersonalMessageOutput>;

/** Input for signing personal messages. */
export interface HaneulSignPersonalMessageInput {
	message: Uint8Array;
	account: WalletAccount;
}

/** Output of signing personal messages. */
export interface HaneulSignPersonalMessageOutput extends SignedPersonalMessage {}

export interface SignedPersonalMessage {
	/** Base64 encoded message bytes */
	bytes: string;
	/** Base64 encoded signature */
	signature: string;
}
