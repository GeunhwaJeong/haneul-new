// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulClient } from '@haneullabs/haneul/client';
import { useContext } from 'react';

import { HaneulClientContext } from '../components/HaneulClientProvider.js';

export function useHaneulClientContext() {
	const haneulClient = useContext(HaneulClientContext);

	if (!haneulClient) {
		throw new Error(
			'Could not find HaneulClientContext. Ensure that you have set up the HaneulClientProvider',
		);
	}

	return haneulClient;
}

export function useHaneulClient(): HaneulClient {
	return useHaneulClientContext().client;
}
