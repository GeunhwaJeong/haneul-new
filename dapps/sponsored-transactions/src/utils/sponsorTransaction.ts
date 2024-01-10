// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulObjectRef } from '@haneullabs/haneul.js/client';
import { getFaucetHost, requestHaneulFromFaucetV1 } from '@haneullabs/haneul.js/faucet';
import { Ed25519Keypair } from '@haneullabs/haneul.js/keypairs/ed25519';
import { TransactionBlock } from '@haneullabs/haneul.js/transactions';

import { client } from './rpc';

// This simulates what a server would do to sponsor a transaction
export async function sponsorTransaction(sender: string, transactionKindBytes: Uint8Array) {
	// Rather than do gas pool management, we just spin out a new keypair to sponsor the transaction with:
	const keypair = new Ed25519Keypair();
	const address = keypair.getPublicKey().toHaneulAddress();
	console.log(`Sponsor address: ${address}`);

	await requestHaneulFromFaucetV1({ recipient: address, host: getFaucetHost('testnet') });

	let payment: HaneulObjectRef[] = [];
	let retires = 50;
	while (retires !== 0) {
		const coins = await client.getCoins({ owner: address, limit: 1 });
		if (coins.data.length > 0) {
			payment = coins.data.map((coin) => ({
				objectId: coin.coinObjectId,
				version: coin.version,
				digest: coin.digest,
			}));
			break;
		}
		await new Promise((resolve) => setTimeout(resolve, 200)); // Sleep for 200ms
		retires -= 1;
	}

	const tx = TransactionBlock.fromKind(transactionKindBytes);
	tx.setSender(sender);
	tx.setGasOwner(address);
	tx.setGasPayment(payment);

	return keypair.signTransactionBlock(await tx.build({ client }));
}
