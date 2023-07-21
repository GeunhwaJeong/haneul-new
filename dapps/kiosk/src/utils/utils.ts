// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { KioskListing, KioskOwnerCap } from '@haneullabs/kiosk';
import {
	GEUNHWA_PER_HANEUL,
	ObjectId,
	HaneulObjectResponse,
	getObjectDisplay,
	getObjectId,
} from '@haneullabs/haneul.js';
import { normalizeHaneulAddress } from '@haneullabs/haneul.js/utils';
// Parse the display of a list of objects into a simple {object_id: display} map
// to use throughout the app.
export const parseObjectDisplays = (
	data: HaneulObjectResponse[],
): Record<ObjectId, Record<string, string> | undefined> => {
	return data.reduce<Record<ObjectId, Record<string, string> | undefined>>(
		(acc, item: HaneulObjectResponse) => {
			const display = getObjectDisplay(item)?.data;
			const id = getObjectId(item);
			acc[id] = display || undefined;
			return acc;
		},
		{},
	);
};

export const processKioskListings = (data: KioskListing[]): Record<ObjectId, KioskListing> => {
	const results: Record<ObjectId, KioskListing> = {};

	data
		.filter((x) => !!x)
		.map((x: KioskListing) => {
			results[x.objectId || ''] = x;
			return x;
		});
	return results;
};

export const geunhwaToHaneul = (geunhwa: bigint | string | undefined) => {
	if (!geunhwa) return 0;
	return Number(geunhwa || 0) / Number(GEUNHWA_PER_HANEUL);
};

export const formatHaneul = (amount: number) => {
	return new Intl.NumberFormat('en-US', {
		minimumFractionDigits: 2,
		maximumFractionDigits: 5,
	}).format(amount);
};

/**
 * Finds an active owner cap for a kioskId based on the
 * address owned kiosks.
 */
export const findActiveCap = (
	caps: KioskOwnerCap[] = [],
	kioskId: ObjectId,
): KioskOwnerCap | undefined => {
	return caps.find((x) => normalizeHaneulAddress(x.kioskId) === normalizeHaneulAddress(kioskId));
};
