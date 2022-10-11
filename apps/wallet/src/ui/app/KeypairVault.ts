// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Ed25519Keypair } from '@haneullabs/haneul.js';

import type { Keypair } from '@haneullabs/haneul.js';

export default class KeypairVault {
    private _keypair: Keypair | null = null;

    public set mnemonic(mnemonic: string) {
        this._keypair = Ed25519Keypair.deriveKeypair(mnemonic);
    }

    public getAccount(): string | null {
        let address = this._keypair?.getPublicKey().toHaneulAddress() || null;
        if (address && !address.startsWith('0x')) {
            address = `0x${address}`;
        }
        return address;
    }

    public getKeyPair() {
        if (!this._keypair) {
            throw new Error('Account keypair is not set');
        }
        return this._keypair;
    }
}
