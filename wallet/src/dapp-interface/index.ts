// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { DAppInterface } from './DAppInterface';

Object.defineProperty(window, 'haneulWallet', {
    enumerable: false,
    configurable: false,
    value: new DAppInterface(window),
});
