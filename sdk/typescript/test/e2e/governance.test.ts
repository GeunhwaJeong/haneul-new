// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { beforeAll, describe, expect, it } from 'vitest';

import { HaneulClient } from '../../src/client';
import { Keypair } from '../../src/cryptography';
import { HANEUL_TYPE_ARG, HaneulSystemStateUtil } from '../../src/framework';
import { setup, TestToolbox } from './utils/setup';

const DEFAULT_STAKE_AMOUNT = 1000000000;

describe('Governance API', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('test requestAddStake', async () => {
		const result = await addStake(toolbox.client, toolbox.keypair);
		expect(result.effects?.status.status).toEqual('success');
	});

	it('test getDelegatedStakes', async () => {
		await addStake(toolbox.client, toolbox.keypair);
		const stakes = await toolbox.client.getStakes({
			owner: toolbox.address(),
		});
		const stakesById = await toolbox.client.getStakesByIds({
			stakedHaneulIds: [stakes[0].stakes[0].stakedHaneulId],
		});
		expect(stakes.length).greaterThan(0);
		expect(stakesById[0].stakes[0]).toEqual(stakes[0].stakes[0]);
	});

	it('test requestWithdrawStake', async () => {
		// TODO: implement this
	});

	it('test getCommitteeInfo', async () => {
		const committeeInfo = await toolbox.client.getCommitteeInfo({
			epoch: '0',
		});
		expect(committeeInfo.validators?.length).greaterThan(0);
	});

	it('test getLatestHaneulSystemState', async () => {
		await toolbox.client.getLatestHaneulSystemState();
	});
});

async function addStake(client: HaneulClient, signer: Keypair) {
	const coins = await client.getCoins({
		owner: await signer.getPublicKey().toHaneulAddress(),
		coinType: HANEUL_TYPE_ARG,
	});

	const system = await client.getLatestHaneulSystemState();
	const validators = system.activeValidators;

	const tx = await HaneulSystemStateUtil.newRequestAddStakeTxn(
		client,
		[coins.data[0].coinObjectId],
		BigInt(DEFAULT_STAKE_AMOUNT),
		validators[0].haneulAddress,
	);

	return await client.signAndExecuteTransactionBlock({
		signer,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
}
