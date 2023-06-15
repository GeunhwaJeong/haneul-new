// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { CoinFormat, formatBalance } from '@haneullabs/core';
import { HANEUL_DECIMALS } from '@haneullabs/haneul.js';

import { type EpochGasInfo, type GraphDurationsType } from './types';

export const UNITS = ['GEUNHWA', 'HANEUL'] as const;
export const GRAPH_DURATIONS = ['7 Epochs', '30 Epochs'] as const;
export const GRAPH_DURATIONS_MAP: Record<GraphDurationsType, number> = {
	'7 Epochs': 7,
	'30 Epochs': 30,
};

export function useGasPriceFormat(gasPrice: bigint | null, unit: 'GEUNHWA' | 'HANEUL') {
	return gasPrice !== null
		? formatBalance(gasPrice, unit === 'GEUNHWA' ? 0 : HANEUL_DECIMALS, CoinFormat.FULL)
		: null;
}

export function isDefined(d: EpochGasInfo) {
	return d.date !== null && d.referenceGasPrice !== null;
}
