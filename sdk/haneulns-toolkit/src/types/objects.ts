// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulAddress } from '@haneullabs/haneul.js';

export type HaneulNSContract = {
    packageId: HaneulAddress;
    haneulns: HaneulAddress;
    registry: HaneulAddress;
    reverseRegistry: HaneulAddress;
};

export type NameObject = {
    id: HaneulAddress;
    owner: HaneulAddress;
    targetAddress: HaneulAddress | '';
    avatar?: HaneulAddress;
    contentHash?: HaneulAddress;
};

export type DataFields = 'avatar' | 'contentHash';

export type NetworkType = 'devnet' | 'testnet';
