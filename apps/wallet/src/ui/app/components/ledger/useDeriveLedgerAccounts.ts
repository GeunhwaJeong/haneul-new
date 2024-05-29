// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type LedgerAccountSerializedUI } from '_src/background/accounts/LedgerAccount';
import type HaneulLedgerClient from '@haneullabs/ledgerjs-hw-app-haneul';
import { Ed25519PublicKey } from '@haneullabs/haneul/keypairs/ed25519';
import { useQuery, type UseQueryOptions } from '@tanstack/react-query';

import { useHaneulLedgerClient } from './HaneulLedgerClientProvider';

export type DerivedLedgerAccount = Pick<
	LedgerAccountSerializedUI,
	'address' | 'publicKey' | 'type' | 'derivationPath'
>;
type UseDeriveLedgerAccountOptions = {
	numAccountsToDerive: number;
} & Pick<UseQueryOptions<DerivedLedgerAccount[], unknown>, 'select'>;

export function useDeriveLedgerAccounts(options: UseDeriveLedgerAccountOptions) {
	const { numAccountsToDerive, ...useQueryOptions } = options;
	const { haneulLedgerClient } = useHaneulLedgerClient();

	return useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: ['derive-ledger-accounts'],
		queryFn: () => {
			if (!haneulLedgerClient) {
				throw new Error("The Haneul application isn't open on a connected Ledger device");
			}
			return deriveAccountsFromLedger(haneulLedgerClient, numAccountsToDerive);
		},
		...useQueryOptions,
		gcTime: 0,
	});
}

async function deriveAccountsFromLedger(
	haneulLedgerClient: HaneulLedgerClient,
	numAccountsToDerive: number,
) {
	const ledgerAccounts: DerivedLedgerAccount[] = [];
	const derivationPaths = getDerivationPathsForLedger(numAccountsToDerive);

	for (const derivationPath of derivationPaths) {
		const publicKeyResult = await haneulLedgerClient.getPublicKey(derivationPath);
		const publicKey = new Ed25519PublicKey(publicKeyResult.publicKey);
		const haneulAddress = publicKey.toHaneulAddress();
		ledgerAccounts.push({
			type: 'ledger',
			address: haneulAddress,
			derivationPath,
			publicKey: publicKey.toBase64(),
		});
	}

	return ledgerAccounts;
}

function getDerivationPathsForLedger(numDerivations: number) {
	return Array.from({
		length: numDerivations,
	}).map((_, index) => `m/44'/8282'/${index}'/0'/0'`);
}
