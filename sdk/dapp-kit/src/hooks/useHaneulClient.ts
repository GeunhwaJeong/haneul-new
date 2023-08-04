// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulClient } from '@haneullabs/haneul.js/client';
import { createContext, useContext } from 'react';

export const HaneulClientContext = createContext<HaneulClient | undefined>(undefined);

export function useHaneulClient() {
	const haneulClient = useContext(HaneulClientContext);

	if (!haneulClient) {
		throw new Error(
			'Could not find HaneulClientContext. Ensure that you have set up the HaneulClientProvider',
		);
	}

	return haneulClient;
}
