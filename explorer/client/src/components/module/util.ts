// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulAddress } from '@haneullabs/haneul.js';

export const HANEUL_ADDRESS_LENGTH = 20;

// TODO: Use version of this function from the SDK when it is exposed.
export function normalizeHaneulAddress(
    value: string,
    forceAdd0x: boolean = false
): HaneulAddress {
    let address = value.toLowerCase();
    if (!forceAdd0x && address.startsWith('0x')) {
        address = address.slice(2);
    }
    const numMissingZeros =
        (HANEUL_ADDRESS_LENGTH - getHexByteLength(address)) * 2;
    if (numMissingZeros <= 0) {
        return '0x' + address;
    }
    return '0x' + '0'.repeat(numMissingZeros) + address;
}

function getHexByteLength(value: string): number {
    return /^(0x|0X)/.test(value) ? (value.length - 2) / 2 : value.length / 2;
}
