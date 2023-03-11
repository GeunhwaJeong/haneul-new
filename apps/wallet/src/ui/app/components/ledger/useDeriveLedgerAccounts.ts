// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Ed25519PublicKey } from '@haneullabs/haneul.js';
import { useEffect, useState } from 'react';

import { type LedgerAccount } from './LedgerAccountItem';
import { useHaneulLedgerClient } from './HaneulLedgerClientProvider';

import type HaneulLedgerClient from '@haneullabs/ledgerjs-hw-app-haneul';

type UseDeriveLedgerAccountOptions = {
    numAccountsToDerive: number;
    onError: (error: unknown) => void;
};

type UseDeriveLedgerAccountResult = [
    LedgerAccount[],
    React.Dispatch<React.SetStateAction<LedgerAccount[]>>,
    boolean
];

export function useDeriveLedgerAccounts(
    options: UseDeriveLedgerAccountOptions
): UseDeriveLedgerAccountResult {
    const { numAccountsToDerive, onError } = options;
    const [ledgerAccounts, setLedgerAccounts] = useState<LedgerAccount[]>([]);
    const [haneulLedgerClient] = useHaneulLedgerClient();
    const [isLoading, setLoading] = useState(false);

    useEffect(() => {
        const generateLedgerAccounts = async () => {
            setLoading(true);

            try {
                if (!haneulLedgerClient) {
                    throw new Error(
                        "The Haneul application isn't open on a connected Ledger device"
                    );
                }

                // We have to do this sequentially since Ledger uses a device lock to
                // enure that only one operation is being executed at a time
                const accounts = await deriveAccountsFromLedger(
                    haneulLedgerClient,
                    numAccountsToDerive
                );
                setLedgerAccounts(accounts);
            } catch (error) {
                if (onError) {
                    onError(error);
                }
            } finally {
                setLoading(false);
            }
        };
        generateLedgerAccounts();
    }, [numAccountsToDerive, onError, haneulLedgerClient]);

    return [ledgerAccounts, setLedgerAccounts, isLoading];
}

async function deriveAccountsFromLedger(
    haneulLedgerClient: HaneulLedgerClient,
    numAccountsToDerive: number
) {
    const ledgerAccounts: LedgerAccount[] = [];
    const derivationPaths = getDerivationPathsForLedger(numAccountsToDerive);

    for (const derivationPath of derivationPaths) {
        const publicKeyResult = await haneulLedgerClient.getPublicKey(
            derivationPath
        );
        const publicKey = new Ed25519PublicKey(publicKeyResult.publicKey);
        ledgerAccounts.push({
            isSelected: false,
            address: publicKey.toHaneulAddress(),
        });
    }

    return ledgerAccounts;
}

function getDerivationPathsForLedger(numDerivations: number) {
    return Array.from({
        length: numDerivations,
    }).map((_, index) => `m/44'/8282'/${index}'/0'/0'`);
}
