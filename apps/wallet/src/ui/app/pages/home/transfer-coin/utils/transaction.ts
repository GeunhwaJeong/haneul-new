// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type CoinStruct, HANEUL_TYPE_ARG, Transaction } from '@haneullabs/haneul.js';

import { parseAmount } from '_src/ui/app/helpers';

interface Options {
    coinType: string;
    to: string;
    amount: string;
    coinDecimals: number;
    isPayAllHaneul: boolean;
    coins: CoinStruct[];
}

export function createTokenTransferTransaction({
    to,
    amount,
    coins,
    coinType,
    coinDecimals,
    isPayAllHaneul,
}: Options) {
    const tx = new Transaction();

    if (isPayAllHaneul && coinType === HANEUL_TYPE_ARG) {
        tx.transferObjects([tx.gas], tx.pure(to));
        tx.setGasPayment(
            coins
                .filter((coin) => coin.coinType === coinType)
                .map((coin) => ({
                    objectId: coin.coinObjectId,
                    digest: coin.digest,
                    version: coin.version,
                }))
        );

        return tx;
    }

    const bigIntAmount = parseAmount(amount, coinDecimals);
    const [primaryCoin, ...mergeCoins] = coins.filter(
        (coin) => coin.coinType === coinType
    );

    if (coinType === HANEUL_TYPE_ARG) {
        const coin = tx.splitCoin(tx.gas, tx.pure(bigIntAmount));
        tx.transferObjects([coin], tx.pure(to));
    } else {
        const primaryCoinInput = tx.object(primaryCoin.coinObjectId);
        if (mergeCoins.length) {
            // TODO: This could just merge a subset of coins that meet the balance requirements instead of all of them.
            tx.mergeCoins(
                primaryCoinInput,
                mergeCoins.map((coin) => tx.object(coin.coinObjectId))
            );
        }
        const coin = tx.splitCoin(primaryCoinInput, tx.pure(bigIntAmount));
        tx.transferObjects([coin], tx.pure(to));
    }

    return tx;
}
