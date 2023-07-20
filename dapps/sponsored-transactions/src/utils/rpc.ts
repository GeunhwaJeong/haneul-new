// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulClient, getFullnodeUrl } from '@haneullabs/haneul.js/client';

export const provider = new HaneulClient({ url: getFullnodeUrl('testnet') });
