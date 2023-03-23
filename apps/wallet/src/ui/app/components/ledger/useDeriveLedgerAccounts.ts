// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Ed25519PublicKey } from '@haneullabs/haneul.js';
import { useQuery, type UseQueryOptions } from '@tanstack/react-query';

import { useHaneulLedgerClient } from './HaneulLedgerClientProvider';
import { AccountType } from '_src/background/keyring/Account';
import { type SerializedLedgerAccount } from '_src/background/keyring/LedgerAccount';

import type HaneulLedgerClient from '@haneullabs/ledgerjs-hw-app-haneul';

type UseDeriveLedgerAccountOptions = {
    numAccountsToDerive: number;
} & Pick<
    UseQueryOptions<SerializedLedgerAccount[], unknown>,
    'select' | 'onSuccess' | 'onError'
>;

export function useDeriveLedgerAccounts(
    options: UseDeriveLedgerAccountOptions
) {
    const { numAccountsToDerive, ...useQueryOptions } = options;
    const { haneulLedgerClient } = useHaneulLedgerClient();

    return useQuery(
        ['derive-ledger-accounts'],
        () => {
            if (!haneulLedgerClient) {
                throw new Error(
                    "The Haneul application isn't open on a connected Ledger device"
                );
            }
            return deriveAccountsFromLedger(
                haneulLedgerClient,
                numAccountsToDerive
            );
        },
        {
            ...useQueryOptions,
            // NOTE: Unfortunately for security purposes, there's no way to uniquely identify a
            // Ledger device without making a request to the Haneul application and returning some
            // unique identifier (or a public key which should guarantee uniqueness). Since we
            // can't provide a unique query key, we'll disable caching entirely.
            cacheTime: 0,
        }
    );
}

async function deriveAccountsFromLedger(
    haneulLedgerClient: HaneulLedgerClient,
    numAccountsToDerive: number
) {
    const ledgerAccounts: SerializedLedgerAccount[] = [];
    const derivationPaths = getDerivationPathsForLedger(numAccountsToDerive);

    for (const derivationPath of derivationPaths) {
        const publicKeyResult = await haneulLedgerClient.getPublicKey(
            derivationPath
        );
        const publicKey = new Ed25519PublicKey(publicKeyResult.publicKey);
        const haneulAddress = publicKey.toHaneulAddress();
        ledgerAccounts.push({
            type: AccountType.LEDGER,
            address: haneulAddress,
            derivationPath,
        });
    }

    return ledgerAccounts;
}

function getDerivationPathsForLedger(numDerivations: number) {
    return Array.from({
        length: numDerivations,
    }).map((_, index) => `m/44'/8282'/${index}'/0'/0'`);
}
