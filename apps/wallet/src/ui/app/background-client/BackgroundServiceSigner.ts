// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulClient } from '@haneullabs/haneul.js/client';
import { WalletSigner } from '../WalletSigner';

import type { BackgroundClient } from '.';
import type { SerializedSignature } from '@haneullabs/haneul.js/cryptography';

export class BackgroundServiceSigner extends WalletSigner {
	readonly #address: string;
	readonly #backgroundClient: BackgroundClient;

	constructor(address: string, backgroundClient: BackgroundClient, client: HaneulClient) {
		super(client);
		this.#address = address;
		this.#backgroundClient = backgroundClient;
	}

	async getAddress(): Promise<string> {
		return this.#address;
	}

	signData(data: Uint8Array): Promise<SerializedSignature> {
		return this.#backgroundClient.signData(this.#address, data);
	}

	connect(client: HaneulClient) {
		return new BackgroundServiceSigner(this.#address, this.#backgroundClient, client);
	}
}
