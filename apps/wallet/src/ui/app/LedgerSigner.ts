// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type HaneulLedgerClient from '@haneullabs/ledgerjs-hw-app-haneul';
import { type HaneulClient } from '@haneullabs/haneul/client';
import { toSerializedSignature, type SignatureScheme } from '@haneullabs/haneul/cryptography';
import { Ed25519PublicKey } from '@haneullabs/haneul/keypairs/ed25519';

import { WalletSigner } from './WalletSigner';

export class LedgerSigner extends WalletSigner {
	#haneulLedgerClient: HaneulLedgerClient | null;
	readonly #connectToLedger: () => Promise<HaneulLedgerClient>;
	readonly #derivationPath: string;
	readonly #signatureScheme: SignatureScheme = 'ED25519';

	constructor(
		connectToLedger: () => Promise<HaneulLedgerClient>,
		derivationPath: string,
		client: HaneulClient,
	) {
		super(client);
		this.#connectToLedger = connectToLedger;
		this.#haneulLedgerClient = null;
		this.#derivationPath = derivationPath;
	}

	async #initializeHaneulLedgerClient() {
		if (!this.#haneulLedgerClient) {
			// We want to make sure that there's only one connection established per Ledger signer
			// instance since some methods make multiple calls like getAddress and signData
			this.#haneulLedgerClient = await this.#connectToLedger();
		}
		return this.#haneulLedgerClient;
	}

	async getAddress(): Promise<string> {
		const ledgerClient = await this.#initializeHaneulLedgerClient();
		const publicKeyResult = await ledgerClient.getPublicKey(this.#derivationPath);
		const publicKey = new Ed25519PublicKey(publicKeyResult.publicKey);
		return publicKey.toHaneulAddress();
	}

	async getPublicKey(): Promise<Ed25519PublicKey> {
		const ledgerClient = await this.#initializeHaneulLedgerClient();
		const { publicKey } = await ledgerClient.getPublicKey(this.#derivationPath);
		return new Ed25519PublicKey(publicKey);
	}

	async signData(data: Uint8Array): Promise<string> {
		const ledgerClient = await this.#initializeHaneulLedgerClient();
		const { signature } = await ledgerClient.signTransaction(this.#derivationPath, data);
		const publicKey = await this.getPublicKey();
		return toSerializedSignature({
			signature,
			signatureScheme: this.#signatureScheme,
			publicKey,
		});
	}

	connect(client: HaneulClient) {
		return new LedgerSigner(this.#connectToLedger, this.#derivationPath, client);
	}
}
