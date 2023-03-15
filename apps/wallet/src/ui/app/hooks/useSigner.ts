// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useHaneulLedgerClient } from '../components/ledger/HaneulLedgerClientProvider';
import { useAccounts } from './useAccounts';
import { useActiveAccount } from './useActiveAccount';
import { thunkExtras } from '_redux/store/thunk-extras';
import { AccountType } from '_src/background/keyring/Account';

import type { HaneulAddress } from '@haneullabs/haneul.js';

export function useSigner(address?: HaneulAddress) {
    const activeAccount = useActiveAccount();
    const existingAccounts = useAccounts();
    const signerAccount = address
        ? existingAccounts.find((account) => account.address === address)
        : activeAccount;

    const { initializeLedgerSignerInstance } = useHaneulLedgerClient();
    const { api, background } = thunkExtras;

    if (!signerAccount) {
        throw new Error("Can't find account for the signer address");
    }

    return async () => {
        if (signerAccount.type === AccountType.LEDGER) {
            return await initializeLedgerSignerInstance(
                signerAccount.derivationPath
            );
        }
        return api.getSignerInstance(signerAccount, background);
    };
}
