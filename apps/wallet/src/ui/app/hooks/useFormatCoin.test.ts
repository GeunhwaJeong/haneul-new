// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import BigNumber from 'bignumber.js';
import { describe, it, expect } from 'vitest';

import { formatBalance } from './useFormatCoin';

const HANEUL_DECIMALS = 9;

function toMist(haneul: string) {
    return new BigNumber(haneul).shiftedBy(HANEUL_DECIMALS).toString();
}

describe('formatBalance', () => {
    it('formats zero amounts correctly', () => {
        expect(formatBalance('0', 0)).toEqual('0');
        expect(formatBalance('0', HANEUL_DECIMALS)).toEqual('0');
    });

    it('formats decimal amounts correctly', () => {
        expect(formatBalance('0', HANEUL_DECIMALS)).toEqual('0');
        expect(formatBalance('0.000', HANEUL_DECIMALS)).toEqual('0');
    });

    it('formats integer amounts correctly', () => {
        expect(formatBalance(toMist('1'), HANEUL_DECIMALS)).toEqual('1');
        expect(formatBalance(toMist('1.0001'), HANEUL_DECIMALS)).toEqual('1');
        expect(formatBalance(toMist('1.1201'), HANEUL_DECIMALS)).toEqual('1.12');
        expect(formatBalance(toMist('1.1234'), HANEUL_DECIMALS)).toEqual('1.123');
        expect(formatBalance(toMist('1.1239'), HANEUL_DECIMALS)).toEqual('1.123');

        expect(formatBalance(toMist('9999.9999'), HANEUL_DECIMALS)).toEqual(
            '9,999.999'
        );
        // 10k + handling:
        expect(formatBalance(toMist('10000'), HANEUL_DECIMALS)).toEqual('10 K');
        expect(formatBalance(toMist('12345'), HANEUL_DECIMALS)).toEqual(
            '12.345 K'
        );
        // Millions:
        expect(formatBalance(toMist('1234000'), HANEUL_DECIMALS)).toEqual(
            '1.234 M'
        );
        // Billions:
        expect(formatBalance(toMist('1234000000'), HANEUL_DECIMALS)).toEqual(
            '1.234 B'
        );
    });
});
