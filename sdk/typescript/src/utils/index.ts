// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { formatAddress, formatDigest } from './format.js';
import {
	isValidHaneulAddress,
	isValidHaneulObjectId,
	isValidTransactionDigest,
	normalizeStructTag,
	normalizeHaneulAddress,
	normalizeHaneulObjectId,
	parseStructTag,
	HANEUL_ADDRESS_LENGTH,
} from './haneul-types.js';

export { fromB64, toB64, fromHEX, toHEX } from '@haneullabs/bcs';
export { is, assert } from 'superstruct';

export {
	formatAddress,
	formatDigest,
	isValidHaneulAddress,
	isValidHaneulObjectId,
	isValidTransactionDigest,
	normalizeStructTag,
	normalizeHaneulAddress,
	normalizeHaneulObjectId,
	parseStructTag,
	HANEUL_ADDRESS_LENGTH,
};

export { isValidHaneulNSName, normalizeHaneulNSName } from './haneulns.js';

export const HANEUL_DECIMALS = 9;
export const GEUNHWA_PER_HANEUL = BigInt(1000000000);

export const MOVE_STDLIB_ADDRESS = '0x1';
export const HANEUL_FRAMEWORK_ADDRESS = '0x2';
export const HANEUL_SYSTEM_ADDRESS = '0x3';
export const HANEUL_CLOCK_OBJECT_ID = normalizeHaneulObjectId('0x6');
export const HANEUL_SYSTEM_MODULE_NAME = 'haneul_system';
export const HANEUL_TYPE_ARG = `${HANEUL_FRAMEWORK_ADDRESS}::haneul::HANEUL`;
export const HANEUL_SYSTEM_STATE_OBJECT_ID: string = normalizeHaneulObjectId('0x5');
