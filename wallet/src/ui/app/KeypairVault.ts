// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Ed25519Keypair } from '@haneullabs/haneul.js';

import { getKeypairFromMnemonics } from '_shared/cryptography/mnemonics';

export default class KeypairVault {
    private _keypair: Ed25519Keypair | null = null;

    public set mnemonic(mnemonic: string) {
        this._keypair = new Ed25519Keypair(getKeypairFromMnemonics(mnemonic));
    }

    public getAccount(): string | null {
        return this._keypair?.getPublicKey().toHaneulAddress() || null;
    }

    public getKeyPair() {
        if (!this._keypair) {
            throw new Error('Account keypair is not set');
        }
        return this._keypair;
    }
}
