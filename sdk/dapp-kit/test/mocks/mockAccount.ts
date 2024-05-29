// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Ed25519Keypair } from '@haneullabs/haneul/keypairs/ed25519';
import type { WalletAccount } from '@haneullabs/wallet-standard';
import { ReadonlyWalletAccount } from '@haneullabs/wallet-standard';

export function createMockAccount(accountOverrides: Partial<WalletAccount> = {}) {
	const keypair = new Ed25519Keypair();
	return new ReadonlyWalletAccount({
		address: keypair.getPublicKey().toHaneulAddress(),
		publicKey: keypair.getPublicKey().toHaneulBytes(),
		chains: ['haneul:unknown'],
		features: [
			'haneul:signAndExecuteTransactionBlock',
			'haneul:signTransactionBlock',
			'haneul:signAndExecuteTransaction',
			'haneul:signTransaction',
		],
		...accountOverrides,
	});
}
