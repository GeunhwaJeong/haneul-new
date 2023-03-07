// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { registerWallet } from '@haneullabs/wallet-standard';

import { HaneulWallet } from './WalletStandardInterface';

registerWallet(new HaneulWallet());
