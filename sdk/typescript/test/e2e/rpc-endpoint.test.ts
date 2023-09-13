// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect, beforeAll } from 'vitest';
import { setup, TestToolbox } from './utils/setup';
import { HaneulGasData } from '../../src/client';

describe('Invoke any RPC endpoint', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('haneulx_getOwnedObjects', async () => {
		const gasObjectsExpected = await toolbox.client.getOwnedObjects({
			owner: toolbox.address(),
		});
		const gasObjects = await toolbox.client.call<{ data: HaneulGasData }>('haneulx_getOwnedObjects', [
			toolbox.address(),
		]);
		expect(gasObjects.data).toStrictEqual(gasObjectsExpected.data);
	});

	it('haneul_getObjectOwnedByAddress Error', async () => {
		expect(toolbox.client.call('haneulx_getOwnedObjects', [])).rejects.toThrowError();
	});

	it('haneulx_getCommitteeInfo', async () => {
		const committeeInfoExpected = await toolbox.client.getCommitteeInfo();

		const committeeInfo = await toolbox.client.call('haneulx_getCommitteeInfo', []);

		expect(committeeInfo).toStrictEqual(committeeInfoExpected);
	});
});
