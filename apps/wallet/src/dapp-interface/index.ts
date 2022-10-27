// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { registerWallet } from '@haneullabs/wallet-standard';

import { DAppInterface } from './DAppInterface';
import { HaneulWallet } from './WalletStandardInterface';

registerWallet(new HaneulWallet());

Object.defineProperty(window, 'haneulWallet', {
    enumerable: false,
    configurable: false,
    value: new DAppInterface(),
});
