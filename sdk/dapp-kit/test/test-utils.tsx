// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulClient } from '@haneullabs/haneul.js/client';
import { HaneulClientProvider } from 'dapp-kit/src';

export function createHaneulClientContextWrapper(client: HaneulClient) {
	return function HaneulClientContextWrapper({ children }: { children: React.ReactNode }) {
		return <HaneulClientProvider networks={{ test: client }}>{children}</HaneulClientProvider>;
	};
}
