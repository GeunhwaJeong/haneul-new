// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject } from '@haneullabs/haneul.js';

import type {
    ObjectId,
    HaneulObject,
    HaneulMoveObject,
    TransactionResponse,
    RawSigner,
    HaneulAddress,
} from '@haneullabs/haneul.js';

const COIN_TYPE = '0x2::Coin::Coin';
const COIN_TYPE_ARG_REGEX = /^0x2::Coin::Coin<(.+)>$/;
export const GAS_TYPE_ARG = '0x2::HANEUL::HANEUL';
export const GAS_SYMBOL = 'HANEUL';

// TODO use sdk
export class Coin {
    public static isCoin(obj: HaneulObject) {
        return isHaneulMoveObject(obj.data) && obj.data.type.startsWith(COIN_TYPE);
    }

    public static getCoinTypeArg(obj: HaneulMoveObject) {
        const res = obj.type.match(COIN_TYPE_ARG_REGEX);
        return res ? res[1] : null;
    }

    public static getCoinSymbol(coinTypeArg: string) {
        return coinTypeArg.substring(coinTypeArg.lastIndexOf(':') + 1);
    }

    public static getBalance(obj: HaneulMoveObject) {
        return BigInt(obj.fields.balance);
    }

    public static getID(obj: HaneulMoveObject): ObjectId {
        return obj.fields.id.id;
    }

    public static getCoinTypeFromArg(coinTypeArg: string) {
        return `${COIN_TYPE}<${coinTypeArg}>`;
    }

    /**
     * Transfer `amount` of Coin<T> to `recipient`.
     *
     * @param signer A signer with connection to the gateway:e.g., new RawSigner(keypair, new JsonRpcProvider(endpoint))
     * @param coins A list of Coins owned by the signer with the same generic type(e.g., 0x2::Haneul::Haneul)
     * @param amount The amount to be transfer
     * @param recipient The haneul address of the recipient
     */
    public static async transferCoin(
        signer: RawSigner,
        coins: HaneulMoveObject[],
        amount: BigInt,
        recipient: HaneulAddress
    ): Promise<TransactionResponse> {
        if (coins.length < 2) {
            throw new Error(`Not enough coins to transfer`);
        }
        const coin = await Coin.selectCoin(coins, amount);
        return await signer.transferCoin({
            objectId: coin,
            gasBudget: 1000,
            recipient: recipient,
        });
    }

    private static async selectCoin(
        coins: HaneulMoveObject[],
        amount: BigInt
    ): Promise<ObjectId> {
        const coin = await Coin.selectCoinForSplit(coins, amount);
        // TODO: Split coin not implemented yet
        return Coin.getID(coin);
    }

    private static async selectCoinForSplit(
        coins: HaneulMoveObject[],
        amount: BigInt
    ): Promise<HaneulMoveObject> {
        // Sort coins by balance in an ascending order
        coins.sort();

        const coinWithSufficientBalance = coins.find(
            (c) => Coin.getBalance(c) >= amount
        );
        if (coinWithSufficientBalance) {
            return coinWithSufficientBalance;
        }

        // merge coins to have a coin with sufficient balance
        throw new Error(`Merge coin Not implemented`);
    }
}
