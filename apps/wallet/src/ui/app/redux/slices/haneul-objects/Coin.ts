// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    getCoinAfterMerge,
    getMoveObject,
    isHaneulMoveObject,
} from '@haneullabs/haneul.js';

import type {
    ObjectId,
    HaneulObject,
    HaneulMoveObject,
    HaneulTransactionResponse,
    RawSigner,
    HaneulAddress,
    JsonRpcProvider,
} from '@haneullabs/haneul.js';

const COIN_TYPE = '0x2::coin::Coin';
const COIN_TYPE_ARG_REGEX = /^0x2::coin::Coin<(.+)>$/;
export const DEFAULT_GAS_BUDGET_FOR_SPLIT = 10000;
export const DEFAULT_GAS_BUDGET_FOR_MERGE = 10000;
export const DEFAULT_GAS_BUDGET_FOR_TRANSFER = 100;
export const DEFAULT_GAS_BUDGET_FOR_TRANSFER_HANEUL = 100;
export const DEFAULT_GAS_BUDGET_FOR_STAKE = 10000;
export const GAS_TYPE_ARG = '0x2::haneul::HANEUL';
export const GAS_SYMBOL = 'HANEUL';
export const DEFAULT_NFT_TRANSFER_GAS_FEE = 450;
export const HANEUL_SYSTEM_STATE_OBJECT_ID =
    '0x0000000000000000000000000000000000000005';

// TODO use sdk
export class Coin {
    public static isCoin(obj: HaneulObject) {
        return isHaneulMoveObject(obj.data) && obj.data.type.startsWith(COIN_TYPE);
    }

    public static getCoinTypeArg(obj: HaneulMoveObject) {
        const res = obj.type.match(COIN_TYPE_ARG_REGEX);
        return res ? res[1] : null;
    }

    public static isHANEUL(obj: HaneulMoveObject) {
        const arg = Coin.getCoinTypeArg(obj);
        return arg ? Coin.getCoinSymbol(arg) === 'HANEUL' : false;
    }

    public static getCoinSymbol(coinTypeArg: string) {
        return coinTypeArg.substring(coinTypeArg.lastIndexOf(':') + 1);
    }

    public static getBalance(obj: HaneulMoveObject): bigint {
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
        amount: bigint,
        recipient: HaneulAddress
    ): Promise<HaneulTransactionResponse> {
        await signer.syncAccountState();
        const coin = await Coin.selectCoin(signer, coins, amount);
        return await signer.pay({
            inputCoins: [coin],
            recipients: [recipient],
            amounts: [Number(amount)],
            gasBudget: DEFAULT_GAS_BUDGET_FOR_TRANSFER,
        });
    }

    /**
     * Transfer `amount` of Coin<Haneul> to `recipient`.
     *
     * @param signer A signer with connection to the gateway:e.g., new RawSigner(keypair, new JsonRpcProvider(endpoint))
     * @param coins A list of Haneul Coins owned by the signer
     * @param amount The amount to be transferred
     * @param recipient The haneul address of the recipient
     */
    public static async transferHaneul(
        signer: RawSigner,
        coins: HaneulMoveObject[],
        amount: bigint,
        recipient: HaneulAddress
    ): Promise<HaneulTransactionResponse> {
        await signer.syncAccountState();
        const coin = await Coin.prepareCoinWithEnoughBalance(
            signer,
            coins,
            amount + BigInt(DEFAULT_GAS_BUDGET_FOR_TRANSFER_HANEUL)
        );
        return await signer.transferHaneul({
            haneulObjectId: Coin.getID(coin),
            gasBudget: DEFAULT_GAS_BUDGET_FOR_TRANSFER_HANEUL,
            recipient: recipient,
            amount: Number(amount),
        });
    }

    /**
     * Stake `amount` of Coin<T> to `validator`. Technically it means user delegates `amount` of Coin<T> to `validator`,
     * such that `validator` will stake the `amount` of Coin<T> for the user.
     *
     * @param signer A signer with connection to the gateway:e.g., new RawSigner(keypair, new JsonRpcProvider(endpoint))
     * @param coins A list of Coins owned by the signer with the same generic type(e.g., 0x2::Haneul::Haneul)
     * @param amount The amount to be staked
     * @param validator The haneul address of the chosen validator
     */
    public static async stakeCoin(
        signer: RawSigner,
        coins: HaneulMoveObject[],
        amount: bigint,
        validator: HaneulAddress
    ): Promise<HaneulTransactionResponse> {
        const coin = await Coin.selectCoin(signer, coins, amount);
        await signer.syncAccountState();
        return await signer.executeMoveCall({
            packageObjectId: '0x2',
            module: 'haneul_system',
            function: 'request_add_delegation',
            typeArguments: [],
            arguments: [HANEUL_SYSTEM_STATE_OBJECT_ID, coin, validator],
            gasBudget: DEFAULT_GAS_BUDGET_FOR_STAKE,
        });
    }

    private static async selectCoin(
        signer: RawSigner,
        coins: HaneulMoveObject[],
        amount: bigint
    ): Promise<ObjectId> {
        const coin = await Coin.prepareCoinWithEnoughBalance(
            signer,
            coins,
            amount
        );
        const coinID = Coin.getID(coin);
        const balance = Coin.getBalance(coin);
        if (balance === amount) {
            return coinID;
        } else if (balance > amount) {
            await signer.splitCoin({
                coinObjectId: coinID,
                gasBudget: DEFAULT_GAS_BUDGET_FOR_SPLIT,
                splitAmounts: [Number(balance - amount)],
            });
            return coinID;
        } else {
            throw new Error(`Insufficient balance`);
        }
    }

    private static async prepareCoinWithEnoughBalance(
        signer: RawSigner,
        coins: HaneulMoveObject[],
        amount: bigint
    ): Promise<HaneulMoveObject> {
        // Sort coins by balance in an ascending order
        coins.sort((a, b) =>
            Coin.getBalance(a) - Coin.getBalance(b) > 0 ? 1 : -1
        );

        // return the coin with the smallest balance that is greater than or equal to the amount
        const coinWithSufficientBalance = coins.find(
            (c) => Coin.getBalance(c) >= amount
        );
        if (coinWithSufficientBalance) {
            return coinWithSufficientBalance;
        }

        // merge coins to have a coin with sufficient balance
        // we will start from the coins with the largest balance
        // and end with the coin with the second smallest balance(i.e., i > 0 instead of i >= 0)
        // we cannot merge coins with the smallest balance because we
        // need to have a separate coin to pay for the gas
        // TODO: there's some edge cases here. e.g., the total balance is enough before spliting/merging
        // but not enough if we consider the cost of splitting and merging.
        let primaryCoin = coins[coins.length - 1];
        for (let i = coins.length - 2; i > 0; i--) {
            const mergeTxn = await signer.mergeCoin({
                primaryCoin: Coin.getID(primaryCoin),
                coinToMerge: Coin.getID(coins[i]),
                gasBudget: DEFAULT_GAS_BUDGET_FOR_MERGE,
            });
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            primaryCoin = getMoveObject(getCoinAfterMerge(mergeTxn)!)!;
            if (Coin.getBalance(primaryCoin) >= amount) {
                return primaryCoin;
            }
        }
        // primary coin might have a balance smaller than the `amount`
        return primaryCoin;
    }

    public static async getActiveValidators(
        provider: JsonRpcProvider
    ): Promise<Array<HaneulMoveObject>> {
        const contents = await provider.getObject(HANEUL_SYSTEM_STATE_OBJECT_ID);
        const data = (contents.details as HaneulObject).data;
        const validators = (data as HaneulMoveObject).fields.validators;
        const active_validators = (validators as HaneulMoveObject).fields
            .active_validators;
        return active_validators as Array<HaneulMoveObject>;
    }
}
