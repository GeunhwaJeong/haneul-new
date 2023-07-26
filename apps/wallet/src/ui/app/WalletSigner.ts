// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SignedMessage, type SignedTransaction, SignerWithProvider } from '@haneullabs/haneul.js';
import {
	type ExecuteTransactionRequestType,
	type HaneulTransactionBlockResponse,
	type HaneulTransactionBlockResponseOptions,
} from '@haneullabs/haneul.js/client';
import { type SerializedSignature } from '@haneullabs/haneul.js/cryptography';
import { type TransactionBlock } from '@haneullabs/haneul.js/transactions';

export abstract class WalletSigner extends SignerWithProvider {
	abstract signData(data: Uint8Array, clientIdentifier?: string): Promise<SerializedSignature>;

	async signMessage(
		input: { message: Uint8Array },
		clientIdentifier?: string,
	): Promise<SignedMessage> {
		return super.signMessage(input);
	}
	async signTransactionBlock(
		input: {
			transactionBlock: Uint8Array | TransactionBlock;
		},
		clientIdentifier?: string,
	): Promise<SignedTransaction> {
		return super.signTransactionBlock(input);
	}
	async signAndExecuteTransactionBlock(
		input: {
			transactionBlock: Uint8Array | TransactionBlock;
			options?: HaneulTransactionBlockResponseOptions;
			requestType?: ExecuteTransactionRequestType;
		},
		clientIdentifier?: string,
	): Promise<HaneulTransactionBlockResponse> {
		return super.signAndExecuteTransactionBlock(input);
	}
}
