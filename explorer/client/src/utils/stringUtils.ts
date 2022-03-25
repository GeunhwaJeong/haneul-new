// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

export const asciiFromNumberBytes = (bytes: number[]): string =>
    bytes.map((n) => String.fromCharCode(n)).join('');

export const trimStdLibPrefix = (str: string): string =>
    str.replace(/^0x2::/, '');

export const isValidHttpUrl = (url: string) => {
    try {
        new URL(url);
    } catch (e) {
        return false;
    }
    return /^https?/.test(url);
};

export const isHaneulAddressHex = (str: string) =>
    /^(0x)?[0-9a-fA-F]{40}$/.test(str);
