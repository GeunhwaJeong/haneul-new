// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Ed25519Keypair } from '@haneullabs/haneul.js/keypairs/ed25519';
import type {
	IdentifierRecord,
	StandardConnectFeature,
	StandardEventsFeature,
	HaneulFeatures,
} from '@haneullabs/wallet-standard';
import { ReadonlyWalletAccount, HANEUL_CHAINS } from '@haneullabs/wallet-standard';
import type { Wallet } from '@haneullabs/wallet-standard';

export class MockWallet implements Wallet {
	version = '1.0.0' as const;
	icon = `data:image/png;base64,` as const;
	chains = HANEUL_CHAINS;
	#walletName: string;
	#additionalFeatures: IdentifierRecord<unknown>;

	#connect = vi.fn().mockReturnValue({ accounts: this.accounts });
	#disconnect = vi.fn();
	#on = vi.fn();
	#signPersonalMessage = vi.fn();
	#signTransactionBlock = vi.fn();
	#signAndExecuteTransactionBlock = vi.fn();

	constructor(name: string, additionalFeatures: IdentifierRecord<unknown>) {
		this.#walletName = name;
		this.#additionalFeatures = additionalFeatures;
	}

	get name() {
		return this.#walletName;
	}

	get accounts() {
		const keypair = new Ed25519Keypair();
		const account = new ReadonlyWalletAccount({
			address: keypair.getPublicKey().toHaneulAddress(),
			publicKey: keypair.getPublicKey().toHaneulBytes(),
			chains: ['haneul:unknown'],
			features: ['haneul:signAndExecuteTransactionBlock', 'haneul:signTransactionBlock'],
		});
		return [account];
	}

	get features(): StandardConnectFeature &
		StandardEventsFeature &
		HaneulFeatures &
		IdentifierRecord<unknown> {
		return {
			'standard:connect': {
				version: '1.0.0',
				connect: this.#connect,
			},
			'standard:disconnect': {
				version: '1.0.0',
				disconnect: this.#disconnect,
			},
			'standard:events': {
				version: '1.0.0',
				on: this.#on,
			},
			'haneul:signPersonalMessage': {
				version: '1.0.0',
				signPersonalMessage: this.#signPersonalMessage,
			},
			'haneul:signTransactionBlock': {
				version: '1.0.0',
				signTransactionBlock: this.#signTransactionBlock,
			},
			'haneul:signAndExecuteTransactionBlock': {
				version: '1.0.0',
				signAndExecuteTransactionBlock: this.#signAndExecuteTransactionBlock,
			},
			...this.#additionalFeatures,
		};
	}
}
