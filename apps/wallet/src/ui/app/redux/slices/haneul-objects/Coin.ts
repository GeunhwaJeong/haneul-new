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
    PayTransaction,
    HaneulExecuteTransactionResponse,
} from '@haneullabs/haneul.js';

const COIN_TYPE = '0x2::coin::Coin';
const COIN_TYPE_ARG_REGEX = /^0x2::coin::Coin<(.+)>$/;
export const DEFAULT_GAS_BUDGET_FOR_SPLIT = 10000;
export const DEFAULT_GAS_BUDGET_FOR_MERGE = 10000;
export const DEFAULT_GAS_BUDGET_FOR_TRANSFER = 100;
export const DEFAULT_GAS_BUDGET_FOR_TRANSFER_HANEUL = 100;
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

    /**
     * Transfer `amount` of Coin<T> to `recipient`.
     *
     * @param signer A signer with connection to fullnode
     * @param coins A list of Coins owned by the signer with the same generic type(e.g., 0x2::Haneul::Haneul)
     * @param amount The amount to be transfer
     * @param recipient The haneul address of the recipient
     */
    public static async transferCoin(
        signer: RawSigner,
        coins: HaneulMoveObject[],
        amount: bigint,
        recipient: HaneulAddress
    ): Promise<HaneulExecuteTransactionResponse> {
        const inputCoins =
            await CoinAPI.selectCoinSetWithCombinedBalanceGreaterThanOrEqual(
                coins,
                amount
            );
        if (inputCoins.length === 0) {
            const totalBalance = CoinAPI.totalBalance(coins);
            throw new Error(
                `Coin balance ${totalBalance.toString()} is not sufficient to cover the transfer amount ` +
                    `${amount.toString()}. Try reducing the transfer amount to ${totalBalance}.`
            );
        }

        const inputCoinIDs = inputCoins.map((c) => CoinAPI.getID(c));
        const gasBudget = Coin.computeGasCostForPay(inputCoins.length);
        const payTxn: PayTransaction = {
            inputCoins: inputCoinIDs,
            recipients: [recipient],
            amounts: [Number(amount)],
            gasBudget,
            gasPayment: await Coin.selectGasPayment(
                coins,
                inputCoinIDs,
                BigInt(gasBudget)
            ),
        };
        return await signer.pay(payTxn);
    }

    private static computeGasCostForPay(numInputCoins: number): number {
        // TODO: improve the gas budget estimation
        return (
            DEFAULT_GAS_BUDGET_FOR_PAY *
            Math.max(2, Math.min(100, numInputCoins / 2))
        );
    }

    private static async selectGasPayment(
        coins: HaneulMoveObject[],
        exclude: ObjectId[],
        amount: bigint
    ): Promise<ObjectId> {
        const gasPayment =
            await CoinAPI.selectCoinWithBalanceGreaterThanOrEqual(
                coins,
                amount,
                exclude
            );
        if (gasPayment === undefined) {
            throw new Error(
                `Unable to find a coin to cover the gas budget ${amount.toString()}`
            );
        }
        return CoinAPI.getID(gasPayment);
    }

    /**
     * Transfer `amount` of Coin<Haneul> to `recipient`.
     *
     * @param signer A signer with connection to fullnode
     * @param coins A list of Haneul Coins owned by the signer
     * @param amount The amount to be transferred
     * @param recipient The haneul address of the recipient
     */
    public static async transferHaneul(
        signer: RawSigner,
        coins: HaneulMoveObject[],
        amount: bigint,
        recipient: HaneulAddress
    ): Promise<HaneulExecuteTransactionResponse> {
        const targetAmount =
            amount + BigInt(DEFAULT_GAS_BUDGET_FOR_TRANSFER_HANEUL);
        const coinsWithSufficientAmount =
            await CoinAPI.selectCoinsWithBalanceGreaterThanOrEqual(
                coins,
                targetAmount
            );
        if (coinsWithSufficientAmount.length > 0) {
            const txn = {
                haneulObjectId: CoinAPI.getID(coinsWithSufficientAmount[0]),
                gasBudget: DEFAULT_GAS_BUDGET_FOR_TRANSFER_HANEUL,
                recipient: recipient,
                amount: Number(amount),
            };
            return await signer.transferHaneul(txn);
        }

        // TODO: use PayHaneul Transaction when it is ready
        // If there is not a coin with sufficient balance, use the pay API
        const gasCostForPay = Coin.computeGasCostForPay(coins.length);
        let inputCoins = await Coin.assertAndGetCoinsWithBalanceGte(
            coins,
            amount,
            gasCostForPay
        );

        // In this case, all coins are needed to cover the transfer amount plus gas budget, leaving
        // no coins for gas payment. This won't be a problem once we introduce `PayHaneul`. But for now,
        // we address this case by splitting an extra coin.
        if (inputCoins.length === coins.length) {
            // We need to pay for an additional `transferHaneul` transaction now, assert that we have sufficient balance
            // to cover the additional cost
            await Coin.assertAndGetCoinsWithBalanceGte(
                coins,
                amount,
                gasCostForPay + DEFAULT_GAS_BUDGET_FOR_TRANSFER_HANEUL
            );

            // Split the gas budget from the coin with largest balance for simplicity. We can also use any coin
            // that has amount greater than or equal to `DEFAULT_GAS_BUDGET_FOR_TRANSFER_HANEUL * 2`
            const coinWithLargestBalance = inputCoins[inputCoins.length - 1];

            if (
                // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                CoinAPI.getBalance(coinWithLargestBalance)! <
                gasCostForPay + DEFAULT_GAS_BUDGET_FOR_TRANSFER_HANEUL
            ) {
                throw new Error(
                    `None of the coins has sufficient balance to cover gas fee`
                );
            }

            const txn = {
                haneulObjectId: CoinAPI.getID(coinWithLargestBalance),
                gasBudget: DEFAULT_GAS_BUDGET_FOR_TRANSFER_HANEUL,
                recipient: await signer.getAddress(),
                amount: gasCostForPay,
            };
            await signer.transferHaneul(txn);

            inputCoins =
                await signer.provider.selectCoinSetWithCombinedBalanceGreaterThanOrEqual(
                    await signer.getAddress(),
                    amount,
                    HANEUL_TYPE_ARG,
                    []
                );
        }
        const txn = {
            inputCoins: inputCoins.map((c) => CoinAPI.getID(c)),
            recipients: [recipient],
            amounts: [Number(amount)],
            gasBudget: gasCostForPay,
        };
        return await signer.pay(txn);
    }

    private static async assertAndGetCoinsWithBalanceGte(
        coins: HaneulMoveObject[],
        amount: bigint,
        gasBudget?: number
    ) {
        const inputCoins =
            await CoinAPI.selectCoinSetWithCombinedBalanceGreaterThanOrEqual(
                coins,
                amount + BigInt(gasBudget ?? 0)
            );
        if (inputCoins.length === 0) {
            const totalBalance = CoinAPI.totalBalance(coins);
            const maxTransferAmount = totalBalance - BigInt(gasBudget ?? 0);
            const gasText = gasBudget ? ` plus gas budget ${gasBudget}` : '';
            throw new Error(
                `Coin balance ${totalBalance.toString()} is not sufficient to cover the transfer amount ` +
                    `${amount.toString()}${gasText}. ` +
                    `Try reducing the transfer amount to ${maxTransferAmount.toString()}.`
            );
        }
        return inputCoins;
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
        await Coin.transferHaneul(
            signer,
            coins,
            amount,
            await signer.getAddress()
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
