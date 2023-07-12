// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { blake2b } from '@noble/hashes/blake2b';
import { fromB64, toB64 } from '@haneullabs/bcs';
import type { PublicKeyInitData } from '../../cryptography/publickey.js';
import { bytesEqual } from '../../cryptography/publickey.js';
import { SIGNATURE_SCHEME_TO_FLAG } from '../../cryptography/signature.js';
import { bytesToHex } from '@noble/hashes/utils';
import { HANEUL_ADDRESS_LENGTH, normalizeHaneulAddress } from '../../utils/haneul-types.js';

const PUBLIC_KEY_SIZE = 32;

/**
 * An Ed25519 public key
 */
export class Ed25519PublicKey {
	static SIZE = PUBLIC_KEY_SIZE;
	private data: Uint8Array;

	/**
	 * Create a new Ed25519PublicKey object
	 * @param value ed25519 public key as buffer or base-64 encoded string
	 */
	constructor(value: PublicKeyInitData) {
		if (typeof value === 'string') {
			this.data = fromB64(value);
		} else if (value instanceof Uint8Array) {
			this.data = value;
		} else {
			this.data = Uint8Array.from(value);
		}

		if (this.data.length !== PUBLIC_KEY_SIZE) {
			throw new Error(
				`Invalid public key input. Expected ${PUBLIC_KEY_SIZE} bytes, got ${this.data.length}`,
			);
		}
	}

	/**
	 * Checks if two Ed25519 public keys are equal
	 */
	equals(publicKey: Ed25519PublicKey): boolean {
		return bytesEqual(this.toBytes(), publicKey.toBytes());
	}

	/**
	 * Return the base-64 representation of the Ed25519 public key
	 */
	toBase64(): string {
		return toB64(this.toBytes());
	}

	/**
	 * Return the byte array representation of the Ed25519 public key
	 */
	toBytes(): Uint8Array {
		return this.data;
	}

	/**
	 * Return the base-64 representation of the Ed25519 public key
	 */
	toString(): string {
		return this.toBase64();
	}

	/**
	 * Return the Haneul address associated with this Ed25519 public key
	 */
	toHaneulAddress(): string {
		let tmp = new Uint8Array(PUBLIC_KEY_SIZE + 1);
		tmp.set([SIGNATURE_SCHEME_TO_FLAG['ED25519']]);
		tmp.set(this.toBytes(), 1);
		// Each hex char represents half a byte, hence hex address doubles the length
		return normalizeHaneulAddress(
			bytesToHex(blake2b(tmp, { dkLen: 32 })).slice(0, HANEUL_ADDRESS_LENGTH * 2),
		);
	}

	/**
	 * Return the Haneul representation of the public key encoded in
	 * base-64. A Haneul public key is formed by the concatenation
	 * of the scheme flag with the raw bytes of the public key
	 */
	toHaneulPublicKey(): string {
		const haneulPublicKey = new Uint8Array(this.data.length + 1);
		haneulPublicKey.set([this.flag()]);
		haneulPublicKey.set(this.data, 1);
		return toB64(haneulPublicKey);
	}

	/**
	 * Return the Haneul address associated with this Ed25519 public key
	 */
	flag(): number {
		return SIGNATURE_SCHEME_TO_FLAG['ED25519'];
	}
}
