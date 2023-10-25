// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { HANEUL_TYPE_ARG } from '@haneullabs/haneul.js/utils';

import { Coins, mainnetDeepBook, useDeepBookConfigs } from '.';

export function useRecognizedCoins() {
	const coinsMap = useDeepBookConfigs().coinsMap;
	return Object.values(coinsMap);
}

export const allowedSwapCoinsList = [HANEUL_TYPE_ARG, mainnetDeepBook.coinsMap[Coins.USDC]];
