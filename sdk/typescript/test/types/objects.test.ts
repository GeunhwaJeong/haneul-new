// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect } from 'vitest';
import mockObjectData from '@haneullabs/haneul-open-rpc/samples/objects.json';

import { isGetObjectDataResponse } from '../../src/index.guard';

describe('Test Objects Definition', () => {
  it('Test against different object definitions', () => {
    validate('coin');
    validate('example_nft');
    validate('move_package');
    validate('hero');
  });
});

function validate(key: 'coin' | 'example_nft' | 'move_package' | 'hero') {
  const data = mockObjectData[key];
  expect(isGetObjectDataResponse(data)).toBeTruthy();
}
