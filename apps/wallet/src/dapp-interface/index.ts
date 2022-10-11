// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { DAppInterface } from './DAppInterface';
import { HaneulWallet } from './WalletStandardInterface';

import type { WalletsWindow } from '@haneullabs/wallet-standard';

declare const window: WalletsWindow;

window.navigator.wallets = window.navigator.wallets || [];
window.navigator.wallets.push(({ register }) => {
    register(new HaneulWallet());
});

Object.defineProperty(window, 'haneulWallet', {
    enumerable: false,
    configurable: false,
    value: new DAppInterface(),
});
