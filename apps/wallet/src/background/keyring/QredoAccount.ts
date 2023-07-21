// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulAddress } from '@haneullabs/haneul.js';
import { normalizeHaneulAddress } from '@haneullabs/haneul.js/utils';

import { type Account, AccountType } from './Account';
import { type Wallet } from '_src/shared/qredo-api';

export type SerializedQredoAccount = {
	type: AccountType.QREDO;
	address: HaneulAddress;
	qredoConnectionID: string;
	qredoWalletID: string;
	labels?: Wallet['labels'];
	derivationPath: null;
	publicKey: string;
};

export class QredoAccount implements Account {
	readonly type = AccountType.QREDO;
	readonly address: HaneulAddress;
	readonly qredoConnectionID: string;
	readonly qredoWalletID: string;
	readonly labels: Wallet['labels'];
	readonly publicKey: string;

	constructor({
		address,
		qredoConnectionID,
		qredoWalletID,
		labels = [],
		publicKey,
	}: Omit<SerializedQredoAccount, 'type' | 'derivationPath'>) {
		this.address = normalizeHaneulAddress(address);
		this.qredoConnectionID = qredoConnectionID;
		this.qredoWalletID = qredoWalletID;
		this.labels = labels;
		this.publicKey = publicKey;
	}

	toJSON(): SerializedQredoAccount {
		return {
			type: this.type,
			address: this.address,
			qredoConnectionID: this.qredoConnectionID,
			qredoWalletID: this.qredoWalletID,
			labels: this.labels,
			derivationPath: null,
			publicKey: this.publicKey,
		};
	}

	getPublicKey() {
		return this.publicKey;
	}
}
