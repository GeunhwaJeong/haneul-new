// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect, beforeAll } from 'vitest';
import { setup, TestToolbox } from './utils/setup';

describe('Invoke any RPC endpoint', () => {
  let toolbox: TestToolbox;

  beforeAll(async () => {
    toolbox = await setup();
  });

  it('haneul_getOwnedObjects', async () => {
    const gasObjectsExpected = await toolbox.provider.getOwnedObjects({
      owner: toolbox.address(),
    });
    const gasObjects = await toolbox.provider.call('haneul_getOwnedObjects', [
      toolbox.address(),
    ]);
    expect(gasObjects.data).toStrictEqual(gasObjectsExpected.data);
  });

  it('haneul_getObjectOwnedByAddress Error', async () => {
    expect(
      toolbox.provider.call('haneul_getOwnedObjects', []),
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
