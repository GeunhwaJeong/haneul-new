// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Coin, type CoinBalance } from '@haneullabs/haneul.js';

// Sort tokens by symbol and total balance
// Move this to the API backend
export function sortGetAllBalancesToken(tokens: CoinBalance[]) {
    return [...tokens].sort((a, b) =>
        (Coin.getCoinSymbol(a.coinType) + Number(a.totalBalance)).localeCompare(
            Coin.getCoinSymbol(b.coinType) + Number(b.totalBalance)
        )
    );
}
