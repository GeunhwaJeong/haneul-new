// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IdentifierRecord, HaneulFeatures } from '@haneullabs/wallet-standard';

export const haneulFeatures: HaneulFeatures = {
	'haneul:signPersonalMessage': {
		version: '1.0.0',
		signPersonalMessage: vi.fn(),
	},
	'haneul:signTransactionBlock': {
		version: '1.0.0',
		signTransactionBlock: vi.fn(),
	},
	'haneul:signAndExecuteTransactionBlock': {
		version: '1.0.0',
		signAndExecuteTransactionBlock: vi.fn(),
	},
};

export const superCoolFeature: IdentifierRecord<unknown> = {
	'my-dapp:super-cool-feature': {
		version: '1.0.0',
		superCoolFeature: vi.fn(),
	},
};
