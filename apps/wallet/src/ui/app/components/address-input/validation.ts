// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulNSName, useRpcClient, useHaneulNSEnabled } from '@haneullabs/core';
import { isValidHaneulAddress } from '@haneullabs/haneul.js';
import { type HaneulClient } from '@haneullabs/haneul.js/client';
import { useMemo } from 'react';
import * as Yup from 'yup';

export function createHaneulAddressValidation(client: HaneulClient, haneulNSEnabled: boolean) {
	const resolveCache = new Map<string, boolean>();

	return Yup.string()
		.ensure()
		.trim()
		.required()
		.test('is-haneul-address', 'Invalid address. Please check again.', async (value) => {
			if (haneulNSEnabled && isHaneulNSName(value)) {
				if (resolveCache.has(value)) {
					return resolveCache.get(value)!;
				}

				const address = await client.resolveNameServiceAddress({
					name: value,
				});

				resolveCache.set(value, !!address);

				return !!address;
			}

			return isValidHaneulAddress(value);
		})
		.label("Recipient's address");
}

export function useHaneulAddressValidation() {
	const client = useRpcClient();
	const haneulNSEnabled = useHaneulNSEnabled();

	return useMemo(() => {
		return createHaneulAddressValidation(client, haneulNSEnabled);
	}, [client, haneulNSEnabled]);
}
