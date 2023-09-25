// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulTransactionBlockResponseOptions } from '@haneullabs/haneul.js/client';
import { TransactionBlock } from '@haneullabs/haneul.js/transactions';
import { useWalletKit } from '@haneullabs/wallet-kit';

import { useRpc } from '../context/RpcClientContext';

// A helper to execute transactions by:
// 1. Signing them using the wallet
// 2. Executing them using the rpc provider
export function useTransactionExecution() {
	const provider = useRpc();

	// sign transaction from the wallet
	const { signTransactionBlock } = useWalletKit();

	// tx: TransactionBlock
	const signAndExecute = async ({
		tx,
		options = { showEffects: true },
	}: {
		tx: TransactionBlock;
		options?: HaneulTransactionBlockResponseOptions | undefined;
	}) => {
		const signedTx = await signTransactionBlock({ transactionBlock: tx });

		const res = await provider.executeTransactionBlock({
			transactionBlock: signedTx.transactionBlockBytes,
			signature: signedTx.signature,
			options,
		});

		const status = res.effects?.status?.status === 'success';

		if (status) return true;
		else throw new Error('Transaction execution failed.');
	};

	return { signAndExecute };
}
