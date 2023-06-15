// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulNSName, useRpcClient, useHaneulNSEnabled } from '@haneullabs/core';
import { type JsonRpcProvider, isValidHaneulAddress } from '@haneullabs/haneul.js';
import { useMemo } from 'react';
import * as Yup from 'yup';

export function createHaneulAddressValidation(rpc: JsonRpcProvider, haneulNSEnabled: boolean) {
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

				const address = await rpc.resolveNameServiceAddress({
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
	const rpc = useRpcClient();
	const haneulNSEnabled = useHaneulNSEnabled();

	return useMemo(() => {
		return createHaneulAddressValidation(rpc, haneulNSEnabled);
	}, [rpc, haneulNSEnabled]);
}
