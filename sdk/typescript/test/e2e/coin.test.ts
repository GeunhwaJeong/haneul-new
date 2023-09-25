// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, expect, it } from 'vitest';

import { Coin } from '../../src/framework';
import { normalizeHaneulObjectId } from '../../src/utils';
import { setup } from './utils/setup';

describe('Coin related API', () => {
	it('test Coin utility functions', async () => {
		const toolbox = await setup();
		const coins = await toolbox.getGasObjectsOwnedByAddress();
		coins.forEach((c) => {
			expect(Coin.isCoin(c)).toBeTruthy();
			expect(Coin.isHANEUL(c)).toBeTruthy();
		});
	});

	it('test getCoinStructTag', async () => {
		const toolbox = await setup();
		const exampleStructTag = {
			address: normalizeHaneulObjectId('0x2'),
			module: 'haneul',
			name: 'HANEUL',
			typeParams: [],
		};
		const coins = await toolbox.getGasObjectsOwnedByAddress();
		const coinTypeArg: string = Coin.getCoinTypeArg(coins[0])!;
		expect(Coin.getCoinStructTag(coinTypeArg)).toStrictEqual(exampleStructTag);
	});
});
