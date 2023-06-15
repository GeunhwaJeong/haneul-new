// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { normalizeHaneulAddress, type HaneulAddress } from '@haneullabs/haneul.js';

import { type Account, AccountType } from './Account';

export type SerializedLedgerAccount = {
	type: AccountType.LEDGER;
	address: HaneulAddress;
	derivationPath: string;
};

export class LedgerAccount implements Account {
	readonly type: AccountType;
	readonly address: HaneulAddress;
	readonly derivationPath: string;

	constructor({ address, derivationPath }: { address: HaneulAddress; derivationPath: string }) {
		this.type = AccountType.LEDGER;
		this.address = normalizeHaneulAddress(address);
		this.derivationPath = derivationPath;
	}

	toJSON(): SerializedLedgerAccount {
		return {
			type: AccountType.LEDGER,
			address: this.address,
			derivationPath: this.derivationPath,
		};
	}
}
