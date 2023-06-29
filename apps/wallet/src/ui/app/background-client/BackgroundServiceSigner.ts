// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SerializedSignature, type JsonRpcProvider, type HaneulAddress } from '@haneullabs/haneul.js';

import { WalletSigner } from '../WalletSigner';

import type { BackgroundClient } from '.';

export class BackgroundServiceSigner extends WalletSigner {
	readonly #address: HaneulAddress;
	readonly #backgroundClient: BackgroundClient;

	constructor(address: HaneulAddress, backgroundClient: BackgroundClient, provider: JsonRpcProvider) {
		super(provider);
		this.#address = address;
		this.#backgroundClient = backgroundClient;
	}

	async getAddress(): Promise<string> {
		return this.#address;
	}

	signData(data: Uint8Array): Promise<SerializedSignature> {
		return this.#backgroundClient.signData(this.#address, data);
	}

	connect(provider: JsonRpcProvider) {
		return new BackgroundServiceSigner(this.#address, this.#backgroundClient, provider);
	}
}
