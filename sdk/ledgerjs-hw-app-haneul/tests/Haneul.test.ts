// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { openTransportReplayer, RecordStore } from '@ledgerhq/hw-transport-mocker';
import { expect, test } from 'vitest';

import Haneul from '../src/Haneul';

test('Haneul init', async () => {
	const transport = await openTransportReplayer(RecordStore.fromString(''));
	const pkt = new Haneul(transport);
	expect(pkt).not.toBe(undefined);
});
