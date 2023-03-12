// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    Coin as CoinAPI,
    HANEUL_SYSTEM_STATE_OBJECT_ID,
    getObjectType,
    Transaction,
} from '@haneullabs/haneul.js';
import * as Sentry from '@sentry/react';

import type {
    ObjectId,
    HaneulObjectData,
    HaneulAddress,
    HaneulMoveObject,
    HaneulTransactionResponse,
    SignerWithProvider,
    CoinStruct,
} from '@haneullabs/haneul.js';

const COIN_TYPE = '0x2::coin::Coin';
const COIN_TYPE_ARG_REGEX = /^0x2::coin::Coin<(.+)>$/;

export const DEFAULT_GAS_BUDGET_FOR_PAY = 150;
export const DEFAULT_GAS_BUDGET_FOR_STAKE = 15000;
export const GAS_TYPE_ARG = '0x2::haneul::HANEUL';
export const GAS_SYMBOL = 'HANEUL';
export const DEFAULT_NFT_TRANSFER_GAS_FEE = 450;
export const DEFAULT_MINT_NFT_GAS_BUDGET = 2000;

// TODO use sdk
export class Coin {
    public static isCoin(obj: HaneulObjectData) {
        return getObjectType(obj)?.startsWith(COIN_TYPE) ?? false;
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
        coins: CoinStruct[],
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

    // TODO: we should replace this function with the SDK implementation
    /**
     * Stake `amount` of Coin<T> to `validator`. Technically it means user stakes `amount` of Coin<T> to `validator`,
     * such that `validator` will stake the `amount` of Coin<T> for the user.
     *
     * @param signer A signer with connection to fullnode
     * @param coins A list of Coins owned by the signer with the same generic type(e.g., 0x2::Haneul::Haneul)
     * @param amount The amount to be staked
     * @param validator The haneul address of the chosen validator
     */
    public static async stakeCoin(
        signer: SignerWithProvider,
        amount: bigint,
        validator: HaneulAddress
    ): Promise<HaneulTransactionResponse> {
        const transaction = Sentry.startTransaction({ name: 'stake' });

        const span = transaction.startChild({
            op: 'request-add-stake',
            description: 'Staking move call',
        });

        try {
            const tx = new Transaction();
            tx.setGasBudget(DEFAULT_GAS_BUDGET_FOR_STAKE);
            const stakeCoin = tx.splitCoin(tx.gas, tx.pure(amount));
            tx.moveCall({
                target: '0x2::haneul_system::request_add_stake',
                arguments: [
                    tx.object(HANEUL_SYSTEM_STATE_OBJECT_ID),
                    stakeCoin,
                    tx.pure(validator),
                ],
            });
            return await signer.signAndExecuteTransaction(tx, {
                showInput: true,
                showEffects: true,
                showEvents: true,
            });
        } finally {
            span.finish();
            transaction.finish();
        }
    }

    public static async unStakeCoin(
        signer: SignerWithProvider,
        stake: ObjectId,
        stakedHaneulId: ObjectId
    ): Promise<HaneulTransactionResponse> {
        const transaction = Sentry.startTransaction({ name: 'unstake' });
        try {
            const tx = new Transaction();
            tx.setGasBudget(DEFAULT_GAS_BUDGET_FOR_STAKE);
            tx.moveCall({
                target: '0x2::haneul_system::request_withdraw_stake',
                arguments: [
                    tx.object(HANEUL_SYSTEM_STATE_OBJECT_ID),
                    tx.object(stake),
                    tx.object(stakedHaneulId),
                ],
            });
            return await signer.signAndExecuteTransaction(tx, {
                showInput: true,
                showEffects: true,
                showEvents: true,
            });
        } finally {
            transaction.finish();
        }
    }
}
