// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulNSName, useHaneulNSEnabled } from '@haneullabs/core';
import { useHaneulClientQuery, useHaneulClient } from '@haneullabs/dapp-kit';
import { type HaneulClient, type HaneulSystemStateSummary } from '@haneullabs/haneul.js/client';
import {
	isValidTransactionDigest,
	isValidHaneulAddress,
	isValidHaneulObjectId,
	normalizeHaneulObjectId,
} from '@haneullabs/haneul.js/utils';
import { useQuery } from '@tanstack/react-query';

const isGenesisLibAddress = (value: string): boolean => /^(0x|0X)0{0,39}[12]$/.test(value);

type Results = { id: string; label: string; type: string }[];

const getResultsForTransaction = async (client: HaneulClient, query: string) => {
	if (!isValidTransactionDigest(query)) return null;
	const txdata = await client.getTransactionBlock({ digest: query });
	return [
		{
			id: txdata.digest,
			label: txdata.digest,
			type: 'transaction',
		},
	];
};

const getResultsForObject = async (client: HaneulClient, query: string) => {
	const normalized = normalizeHaneulObjectId(query);
	if (!isValidHaneulObjectId(normalized)) return null;

	const { data, error } = await client.getObject({ id: normalized });
	if (!data || error) return null;

	return [
		{
			id: data.objectId,
			label: data.objectId,
			type: 'object',
		},
	];
};

const getResultsForCheckpoint = async (client: HaneulClient, query: string) => {
	// Checkpoint digests have the same format as transaction digests:
	if (!isValidTransactionDigest(query)) return null;

	const { digest } = await client.getCheckpoint({ id: query });
	if (!digest) return null;

	return [
		{
			id: digest,
			label: digest,
			type: 'checkpoint',
		},
	];
};

const getResultsForAddress = async (client: HaneulClient, query: string, haneulNSEnabled: boolean) => {
	if (haneulNSEnabled && isHaneulNSName(query)) {
		const resolved = await client.resolveNameServiceAddress({ name: query.toLowerCase() });
		if (!resolved) return null;
		return [
			{
				id: resolved,
				label: resolved,
				type: 'address',
			},
		];
	}

	const normalized = normalizeHaneulObjectId(query);
	if (!isValidHaneulAddress(normalized) || isGenesisLibAddress(normalized)) return null;

	const [from, to] = await Promise.all([
		client.queryTransactionBlocks({
			filter: { FromAddress: normalized },
			limit: 1,
		}),
		client.queryTransactionBlocks({
			filter: { ToAddress: normalized },
			limit: 1,
		}),
	]);

	if (!from.data?.length && !to.data?.length) return null;

	return [
		{
			id: normalized,
			label: normalized,
			type: 'address',
		},
	];
};

// Query for validator by pool id or haneul address.
const getResultsForValidatorByPoolIdOrHaneulAddress = async (
	systemStateSummery: HaneulSystemStateSummary | null,
	query: string,
) => {
	const normalized = normalizeHaneulObjectId(query);
	if ((!isValidHaneulAddress(normalized) && !isValidHaneulObjectId(normalized)) || !systemStateSummery)
		return null;

	// find validator by pool id or haneul address
	const validator = systemStateSummery.activeValidators?.find(
		({ stakingPoolId, haneulAddress }) => stakingPoolId === normalized || haneulAddress === query,
	);

	if (!validator) return null;

	return [
		{
			id: validator.haneulAddress || validator.stakingPoolId,
			label: normalized,
			type: 'validator',
		},
	];
};

export function useSearch(query: string) {
	const client = useHaneulClient();
	const { data: systemStateSummery } = useHaneulClientQuery('getLatestHaneulSystemState');
	const haneulNSEnabled = useHaneulNSEnabled();

	return useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: ['search', query],
		queryFn: async () => {
			const results = (
				await Promise.allSettled([
					getResultsForTransaction(client, query),
					getResultsForCheckpoint(client, query),
					getResultsForAddress(client, query, haneulNSEnabled),
					getResultsForObject(client, query),
					getResultsForValidatorByPoolIdOrHaneulAddress(systemStateSummery || null, query),
				])
			).filter((r) => r.status === 'fulfilled' && r.value) as PromiseFulfilledResult<Results>[];

			return results.map(({ value }) => value).flat();
		},
		enabled: !!query,
		cacheTime: 10000,
	});
}
