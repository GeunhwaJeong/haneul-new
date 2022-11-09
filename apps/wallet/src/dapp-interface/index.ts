// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { registerWallet } from '@haneullabs/wallet-standard';

import { DAppInterface } from './DAppInterface';
import { HaneulWallet } from './WalletStandardInterface';

registerWallet(new HaneulWallet());

try {
    Object.defineProperty(window, 'haneulWallet', {
        enumerable: false,
        configurable: false,
        value: new DAppInterface(),
    });
} catch (e) {
    // eslint-disable-next-line no-console
    console.warn(
        '[haneul-wallet] Unable to attach to window.haneulWallet. There are likely multiple copies of the Haneul Wallet installed.'
    );
}
