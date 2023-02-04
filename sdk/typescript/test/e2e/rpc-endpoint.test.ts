// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect, beforeAll } from 'vitest';
import { setup, TestToolbox } from './utils/setup';

describe('Invoke any RPC endpoint', () => {
  let toolbox: TestToolbox;

  beforeAll(async () => {
    toolbox = await setup();
  });

  it('haneul_getObjectsOwnedByAddress', async () => {
    const gasObjectsExpected = await toolbox.provider.getObjectsOwnedByAddress(
      toolbox.address(),
    );
    const gasObjects = await toolbox.provider.call(
      'haneul_getObjectsOwnedByAddress',
      [toolbox.address()],
    );
    expect(gasObjects).toStrictEqual(gasObjectsExpected);
  });

  it('haneul_getObjectOwnedByAddress Error', async () => {
    expect(
      toolbox.provider.call('haneul_getObjectsOwnedByAddress', []),
    ).rejects.toThrowError();
  });

  it('haneul_getCommitteeInfo', async () => {
    const committeeInfoExpected = await toolbox.provider.getCommitteeInfo();

    const committeeInfo = await toolbox.provider.call(
      'haneul_getCommitteeInfo',
      [],
    );

    expect(committeeInfo).toStrictEqual(committeeInfoExpected);
  });
});
