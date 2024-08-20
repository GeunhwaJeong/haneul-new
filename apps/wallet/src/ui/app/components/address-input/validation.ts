// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useHaneulNSEnabled } from '@haneullabs/core';
import { useHaneulClient } from '@haneullabs/dapp-kit';
import { type HaneulClient } from '@haneullabs/haneul/client';
import { isValidHaneulAddress, isValidHaneulNSName } from '@haneullabs/haneul/utils';
import { useMemo } from 'react';
import * as Yup from 'yup';

const CACHE_EXPIRY_TIME = 60 * 1000; // 1 minute in milliseconds

export function createHaneulAddressValidation(client: HaneulClient, haneulNSEnabled: boolean) {
	const resolveCache = new Map<string, { valid: boolean; expiry: number }>();

	const currentTime = Date.now();
	return Yup.string()
		.ensure()
		.trim()
		.required()
		.test('is-haneul-address', 'Invalid address. Please check again.', async (value) => {
			if (haneulNSEnabled && isValidHaneulNSName(value)) {
				if (resolveCache.has(value)) {
					const cachedEntry = resolveCache.get(value)!;
					if (currentTime < cachedEntry.expiry) {
						return cachedEntry.valid;
					} else {
						resolveCache.delete(value); // Remove expired entry
					}
				}

				const address = await client.resolveNameServiceAddress({
					name: value,
				});

				resolveCache.set(value, {
					valid: !!address,
					expiry: currentTime + CACHE_EXPIRY_TIME,
				});

				return !!address;
			}

			return isValidHaneulAddress(value);
		})
		.label("Recipient's address");
}

export function useHaneulAddressValidation() {
	const client = useHaneulClient();
	const haneulNSEnabled = useHaneulNSEnabled();

	return useMemo(() => {
		return createHaneulAddressValidation(client, haneulNSEnabled);
	}, [client, haneulNSEnabled]);
}
