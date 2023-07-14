// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { toB64 } from '@haneullabs/bcs';

/**
 * Value to be converted into public key.
 */
export type PublicKeyInitData = string | Uint8Array | Iterable<number>;

export function bytesEqual(a: Uint8Array, b: Uint8Array) {
	if (a === b) return true;

	if (a.length !== b.length) {
		return false;
	}

	for (let i = 0; i < a.length; i++) {
		if (a[i] !== b[i]) {
			return false;
		}
	}
	return true;
}

/**
 * A public key
 */
export abstract class PublicKey {
	/**
	 * Checks if two public keys are equal
	 */
	equals(publicKey: PublicKey) {
		return bytesEqual(this.toBytes(), publicKey.toBytes());
	}

	/**
	 * Return the base-64 representation of the public key
	 */
	toBase64() {
		return toB64(this.toBytes());
	}

	/**
	 * @deprecated use toBase64 instead.
	 *
	 * Return the base-64 representation of the public key
	 */
	toString() {
		return this.toBase64();
	}

	/**
	 * Return the Haneul representation of the public key encoded in
	 * base-64. A Haneul public key is formed by the concatenation
	 * of the scheme flag with the raw bytes of the public key
	 */
	toHaneulPublicKey(): string {
		const bytes = this.toBytes();
		const haneulPublicKey = new Uint8Array(bytes.length + 1);
		haneulPublicKey.set([this.flag()]);
		haneulPublicKey.set(bytes, 1);
		return toB64(haneulPublicKey);
	}

	/**
	 * Return the byte array representation of the public key
	 */
	abstract toBytes(): Uint8Array;

	/**
	 * Return the Haneul address associated with this public key
	 */
	abstract toHaneulAddress(): string;

	/**
	 * Return signature scheme flag of the public key
	 */
	abstract flag(): number;
}
