// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulAddress } from '@haneullabs/haneul.js';
import { normalizeHaneulAddress } from '@haneullabs/haneul.js/utils';

import { type Account, AccountType } from './Account';

export type SerializedLedgerAccount = {
	type: AccountType.LEDGER;
	address: HaneulAddress;
	derivationPath: string;
	publicKey: string | null;
};

export class LedgerAccount implements Account {
	readonly type: AccountType;
	readonly address: HaneulAddress;
	readonly derivationPath: string;
	#publicKey: string | null;

	constructor({
		address,
		derivationPath,
		publicKey,
	}: {
		address: HaneulAddress;
		derivationPath: string;
		publicKey: string | null;
	}) {
		this.type = AccountType.LEDGER;
		this.address = normalizeHaneulAddress(address);
		this.derivationPath = derivationPath;
		this.#publicKey = publicKey;
	}

	toJSON(): SerializedLedgerAccount {
		return {
			type: AccountType.LEDGER,
			address: this.address,
			derivationPath: this.derivationPath,
			publicKey: this.#publicKey,
		};
	}

	getPublicKey() {
		return this.#publicKey;
	}

	setPublicKey(publicKey: string) {
		this.#publicKey = publicKey;
	}
}
