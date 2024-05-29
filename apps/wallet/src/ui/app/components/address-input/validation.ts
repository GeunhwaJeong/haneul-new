// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulNSName, useHaneulNSEnabled } from '@haneullabs/core';
import { useHaneulClient } from '@haneullabs/dapp-kit';
import { type HaneulClient } from '@haneullabs/haneul/client';
import { isValidHaneulAddress } from '@haneullabs/haneul/utils';
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
	const client = useHaneulClient();
	const haneulNSEnabled = useHaneulNSEnabled();

	return useMemo(() => {
		return createHaneulAddressValidation(client, haneulNSEnabled);
	}, [client, haneulNSEnabled]);
}
