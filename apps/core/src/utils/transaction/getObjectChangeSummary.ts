// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import {
	DisplayFieldsResponse,
	HaneulObjectChange,
	HaneulObjectChangeCreated,
	HaneulObjectChangeDeleted,
	HaneulObjectChangeMutated,
	HaneulObjectChangePublished,
	HaneulObjectChangeTransferred,
	HaneulObjectChangeWrapped,
} from '@haneullabs/haneul.js/client';

import { groupByOwner } from './groupByOwner';
import { HaneulObjectChangeTypes } from './types';

export type WithDisplayFields<T> = T & { display?: DisplayFieldsResponse };
export type HaneulObjectChangeWithDisplay = WithDisplayFields<HaneulObjectChange>;

export type ObjectChanges = {
	changesWithDisplay: HaneulObjectChangeWithDisplay[];
	changes: HaneulObjectChange[];
	ownerType: string;
};
export type ObjectChangesByOwner = Record<string, ObjectChanges>;

export type ObjectChangeSummary = {
	[K in HaneulObjectChangeTypes]: ObjectChangesByOwner;
};

export const getObjectChangeSummary = (objectChanges: HaneulObjectChangeWithDisplay[]) => {
	if (!objectChanges) return null;

	const mutated = objectChanges.filter(
		(change) => change.type === 'mutated',
	) as HaneulObjectChangeMutated[];

	const created = objectChanges.filter(
		(change) => change.type === 'created',
	) as HaneulObjectChangeCreated[];

	const transferred = objectChanges.filter(
		(change) => change.type === 'transferred',
	) as HaneulObjectChangeTransferred[];

	const published = objectChanges.filter(
		(change) => change.type === 'published',
	) as HaneulObjectChangePublished[];

	const wrapped = objectChanges.filter(
		(change) => change.type === 'wrapped',
	) as HaneulObjectChangeWrapped[];

	const deleted = objectChanges.filter(
		(change) => change.type === 'deleted',
	) as HaneulObjectChangeDeleted[];

	return {
		transferred: groupByOwner(transferred),
		created: groupByOwner(created),
		mutated: groupByOwner(mutated),
		published: groupByOwner(published),
		wrapped: groupByOwner(wrapped),
		deleted: groupByOwner(deleted),
	};
};
