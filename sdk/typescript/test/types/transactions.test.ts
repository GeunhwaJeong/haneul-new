// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect } from 'vitest';
import mockTransactionData from '@haneullabs/haneul-open-rpc/samples/transactions.json';

import { isHaneulTransactionResponse } from '../../src/index.guard';

describe('Test Transaction Definition', () => {
  it('Test against different transaction definitions', () => {
    const txns = mockTransactionData;

    expect(isHaneulTransactionResponse(txns['move_call'])).toBeTruthy();
    expect(isHaneulTransactionResponse(txns['transfer'])).toBeTruthy();
    expect(isHaneulTransactionResponse(txns['coin_split'])).toBeTruthy();
    expect(isHaneulTransactionResponse(txns['transfer_haneul'])).toBeTruthy();
    // TODO: add mock data for failed transaction
    // expect(
    //   isTransactionEffectsResponse(txns['fail'])
    // ).toBeTruthy();
  });
});
