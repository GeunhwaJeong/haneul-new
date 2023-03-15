// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    Ed25519PublicKey,
    type SerializedSignature,
    type SignatureScheme,
    SignerWithProvider,
    type HaneulAddress,
    toSerializedSignature,
    type JsonRpcProvider,
} from '@haneullabs/haneul.js';

import type HaneulLedgerClient from '@haneullabs/ledgerjs-hw-app-haneul';

export class LedgerSigner extends SignerWithProvider {
    readonly #haneulLedgerClient: HaneulLedgerClient;
    readonly #derivationPath: string;
    readonly #signatureScheme: SignatureScheme = 'ED25519';

    constructor(
        haneulLedgerClient: HaneulLedgerClient,
        derivationPath: string,
        provider: JsonRpcProvider
    ) {
        super(provider);
        this.#haneulLedgerClient = haneulLedgerClient;
        this.#derivationPath = derivationPath;
    }

    async getAddress(): Promise<HaneulAddress> {
        const publicKeyResult = await this.#haneulLedgerClient.getPublicKey(
            this.#derivationPath
        );
        const publicKey = new Ed25519PublicKey(publicKeyResult.publicKey);
        return publicKey.toHaneulAddress();
    }

    async getPublicKey(): Promise<Ed25519PublicKey> {
        const { publicKey } = await this.#haneulLedgerClient.getPublicKey(
            this.#derivationPath
        );
        return new Ed25519PublicKey(publicKey);
    }

    async signData(data: Uint8Array): Promise<SerializedSignature> {
        const { signature } = await this.#haneulLedgerClient.signTransaction(
            this.#derivationPath,
            data
        );
        const pubKey = await this.getPublicKey();
        return toSerializedSignature({
            signature,
            signatureScheme: this.#signatureScheme,
            pubKey,
        });
    }

    connect(provider: JsonRpcProvider): SignerWithProvider {
        return new LedgerSigner(
            this.#haneulLedgerClient,
            this.#derivationPath,
            provider
        );
    }
}
