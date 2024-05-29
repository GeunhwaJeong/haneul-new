// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { getFullnodeUrl, HaneulClient } from '@haneullabs/haneul/client';
import { renderHook } from '@testing-library/react';

import { useHaneulClient } from '../../src/index.js';
import { createHaneulClientContextWrapper } from '../test-utils.js';

describe('useHaneulClient', () => {
	test('throws without a HaneulClientContext', () => {
		expect(() => renderHook(() => useHaneulClient())).toThrowError(
			'Could not find HaneulClientContext. Ensure that you have set up the HaneulClientProvider',
		);
	});

	test('returns a HaneulClient', () => {
		const haneulClient = new HaneulClient({ url: getFullnodeUrl('localnet') });
		const wrapper = createHaneulClientContextWrapper(haneulClient);
		const { result } = renderHook(() => useHaneulClient(), { wrapper });

		expect(result.current).toBe(haneulClient);
	});
});
