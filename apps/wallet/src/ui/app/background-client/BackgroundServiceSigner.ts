// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SerializedSignature, SignerWithProvider } from '@haneullabs/haneul.js';

import type { BackgroundClient } from '.';
import type { Provider, HaneulAddress, TxnDataSerializer } from '@haneullabs/haneul.js';

export class BackgroundServiceSigner extends SignerWithProvider {
    readonly #address: HaneulAddress;
    readonly #backgroundClient: BackgroundClient;

    constructor(
        address: HaneulAddress,
        backgroundClient: BackgroundClient,
        provider?: Provider,
        serializer?: TxnDataSerializer
    ) {
        super(provider, serializer);
        this.#address = address;
        this.#backgroundClient = backgroundClient;
    }

    async getAddress(): Promise<string> {
        return this.#address;
    }

    signData(data: Uint8Array): Promise<SerializedSignature> {
        return this.#backgroundClient.signData(this.#address, data);
    }

    connect(provider: Provider): SignerWithProvider {
        return new BackgroundServiceSigner(
            this.#address,
            this.#backgroundClient,
            provider
        );
    }
}
