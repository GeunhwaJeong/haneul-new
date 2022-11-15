// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject, Coin as CoinAPI, HANEUL_TYPE_ARG } from '@haneullabs/haneul.js';

import type {
    ObjectId,
    HaneulObject,
    HaneulMoveObject,
    RawSigner,
    HaneulAddress,
    JsonRpcProvider,
    HaneulExecuteTransactionResponse,
} from '@haneullabs/haneul.js';

const COIN_TYPE = '0x2::coin::Coin';
const COIN_TYPE_ARG_REGEX = /^0x2::coin::Coin<(.+)>$/;

export const DEFAULT_GAS_BUDGET_FOR_PAY = 150;
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

    public static computeGasBudgetForPay(
        coins: HaneulMoveObject[],
        amountToSend: bigint
    ): number {
        // TODO: improve the gas budget estimation
        const numInputCoins =
            CoinAPI.selectCoinSetWithCombinedBalanceGreaterThanOrEqual(
                coins,
                amountToSend
            ).length;
        return (
            DEFAULT_GAS_BUDGET_FOR_PAY *
            Math.max(2, Math.min(100, numInputCoins / 2))
        );
    }

    /**
     * Stake `amount` of Coin<T> to `validator`. Technically it means user delegates `amount` of Coin<T> to `validator`,
     * such that `validator` will stake the `amount` of Coin<T> for the user.
     *
     * @param signer A signer with connection to fullnode
     * @param coins A list of Coins owned by the signer with the same generic type(e.g., 0x2::Haneul::Haneul)
     * @param amount The amount to be staked
     * @param validator The haneul address of the chosen validator
     */
    public static async stakeCoin(
        signer: RawSigner,
        coins: HaneulMoveObject[],
        amount: bigint,
        validator: HaneulAddress
    ): Promise<HaneulExecuteTransactionResponse> {
        const coin = await Coin.requestHaneulCoinWithExactAmount(
            signer,
            coins,
            amount
        );
        const txn = {
            packageObjectId: '0x2',
            module: 'haneul_system',
            function: 'request_add_delegation',
            typeArguments: [],
            arguments: [HANEUL_SYSTEM_STATE_OBJECT_ID, coin, validator],
            gasBudget: DEFAULT_GAS_BUDGET_FOR_STAKE,
        };
        return await signer.executeMoveCall(txn);
    }

    private static async requestHaneulCoinWithExactAmount(
        signer: RawSigner,
        coins: HaneulMoveObject[],
        amount: bigint
    ): Promise<ObjectId> {
        const coinWithExactAmount = await Coin.selectHaneulCoinWithExactAmount(
            signer,
            coins,
            amount
        );
        if (coinWithExactAmount) {
            return coinWithExactAmount;
        }
        // use transferHaneul API to get a coin with the exact amount
        await CoinAPI.transfer(
            signer,
            coins,
            HANEUL_TYPE_ARG,
            amount,
            await signer.getAddress(),
            Coin.computeGasBudgetForPay(coins, amount)
        );

        const coinWithExactAmount2 = await Coin.selectHaneulCoinWithExactAmount(
            signer,
            coins,
            amount,
            true
        );
        if (!coinWithExactAmount2) {
            throw new Error(`requestCoinWithExactAmount failed unexpectedly`);
        }
        return coinWithExactAmount2;
    }

    private static async selectHaneulCoinWithExactAmount(
        signer: RawSigner,
        coins: HaneulMoveObject[],
        amount: bigint,
        refreshData = false
    ): Promise<ObjectId | undefined> {
        const coinsWithSufficientAmount = refreshData
            ? await signer.provider.selectCoinsWithBalanceGreaterThanOrEqual(
                  await signer.getAddress(),
                  amount,
                  HANEUL_TYPE_ARG,
                  []
              )
            : await CoinAPI.selectCoinsWithBalanceGreaterThanOrEqual(
                  coins,
                  amount
              );

        if (
            coinsWithSufficientAmount.length > 0 &&
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            CoinAPI.getBalance(coinsWithSufficientAmount[0])! === amount
        ) {
            return CoinAPI.getID(coinsWithSufficientAmount[0]);
        }

        return undefined;
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
