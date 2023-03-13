// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Keypair } from '../cryptography/keypair';
import {
  SerializedSignature,
  toSerializedSignature,
} from '../cryptography/signature';
import { JsonRpcProvider } from '../providers/json-rpc-provider';
import { HaneulAddress } from '../types';
import { SignerWithProvider } from './signer-with-provider';

export class RawSigner extends SignerWithProvider {
  private readonly keypair: Keypair;

  constructor(keypair: Keypair, provider: JsonRpcProvider) {
    super(provider);
    this.keypair = keypair;
  }

  async getAddress(): Promise<HaneulAddress> {
    return this.keypair.getPublicKey().toHaneulAddress();
  }

  async signData(data: Uint8Array): Promise<SerializedSignature> {
    const pubkey = this.keypair.getPublicKey();
    const signature = this.keypair.signData(data, false);
    const signatureScheme = this.keypair.getKeyScheme();

    return toSerializedSignature({
      signatureScheme,
      signature,
      pubKey: pubkey,
    });
  }

  connect(provider: JsonRpcProvider): SignerWithProvider {
    return new RawSigner(this.keypair, provider);
  }
}
