// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulMoveObject, HaneulObjectData, HaneulObjectResponse } from '@haneullabs/haneul.js/client';
import { normalizeHaneulAddress } from '@haneullabs/haneul.js/utils';

export const camelCase = (string: string) => string.replace(/(_\w)/g, (g) => g[1].toUpperCase());

export const parseObjectDataResponse = (response: HaneulObjectResponse | undefined) =>
	((response?.data as HaneulObjectData)?.content as HaneulMoveObject)?.fields as Record<string, any>;

export const parseRegistryResponse = (response: HaneulObjectResponse | undefined): any => {
	const fields = parseObjectDataResponse(response)?.value?.fields || {};

	const object = Object.fromEntries(
		Object.entries({ ...fields }).map(([key, val]) => [camelCase(key), val]),
	);

	if (response?.data?.objectId) {
		object.id = response.data.objectId;
	}

	delete object.data;

	const data = (fields.data?.fields.contents || []).reduce(
		(acc: Record<string, any>, c: Record<string, any>) => {
			const key = c.fields.key;
			const value = c.fields.value;

			return {
				...acc,
				[camelCase(key)]:
					c.type.includes('Address') || key === 'addr' ? normalizeHaneulAddress(value) : value,
			};
		},
		{},
	);

	return { ...object, ...data };
};
