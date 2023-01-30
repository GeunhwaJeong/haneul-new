// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { normalizeHaneulAddress } from '@haneullabs/haneul.js';

import type {
    SignaturePubkeyPair,
    Keypair,
    HaneulAddress,
    Base64DataBuffer,
} from '@haneullabs/haneul.js';

export type AccountType = 'derived' | 'imported';
export type AccountSerialized = {
    type: AccountType;
    address: HaneulAddress;
    derivationPath: string | null;
};

export class Account {
    #keypair: Keypair;
    public readonly type: AccountType;
    public readonly derivationPath: string | null;
    public readonly address: HaneulAddress;

    constructor(
        options:
            | { type: 'derived'; derivationPath: string; keypair: Keypair }
            | { type: 'imported'; keypair: Keypair }
    ) {
        this.type = options.type;
        this.derivationPath =
            options.type === 'derived' ? options.derivationPath : null;
        this.#keypair = options.keypair;
        this.address = normalizeHaneulAddress(
            this.#keypair.getPublicKey().toHaneulAddress()
        );
    }

    exportKeypair() {
        return this.#keypair.export();
    }

    async sign(data: Base64DataBuffer): Promise<SignaturePubkeyPair> {
        return {
            signatureScheme: this.#keypair.getKeyScheme(),
            // TODO(joyqvq): Remove once 0.25.0 is released.
            // This is fine to hardcode useRecoverable = false because wallet does not support Secp256k1. Ed25519 does not use this parameter.
            signature: this.#keypair.signData(data, false),
            pubKey: this.#keypair.getPublicKey(),
        };
    }

    toJSON(): AccountSerialized {
        return {
            type: this.type,
            address: this.address,
            derivationPath: this.derivationPath,
        };
    }
}
