// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type Keypair, type HaneulAddress } from '@haneullabs/haneul.js';

import { type Account, AccountType } from './Account';
import { AccountKeypair } from './AccountKeypair';

export type SerializedDerivedAccount = {
	type: AccountType.DERIVED;
	address: HaneulAddress;
	derivationPath: string;
};

export class DerivedAccount implements Account {
	readonly accountKeypair: AccountKeypair;
	readonly type: AccountType;
	readonly address: HaneulAddress;
	readonly derivationPath: string;

	constructor({ derivationPath, keypair }: { derivationPath: string; keypair: Keypair }) {
		this.type = AccountType.DERIVED;
		this.derivationPath = derivationPath;
		this.accountKeypair = new AccountKeypair(keypair);
		this.address = this.accountKeypair.publicKey.toHaneulAddress();
	}

	toJSON(): SerializedDerivedAccount {
		return {
			type: AccountType.DERIVED,
			address: this.address,
			derivationPath: this.derivationPath,
		};
	}

	getPublicKey() {
		return this.accountKeypair.publicKey.toBase64();
	}
}
