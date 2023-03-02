// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ArrowRight16 } from '@haneullabs/icons';
import {
    HANEUL_TYPE_ARG,
    Coin as CoinAPI,
    type HaneulMoveObject,
} from '@haneullabs/haneul.js';
import { Field, Form, useFormikContext, Formik } from 'formik';
import { useMemo, useEffect } from 'react';

import { createValidationSchemaStepOne } from './validation';
import { useFormatCoin, CoinFormat } from '_app/hooks/useFormatCoin';
import { Button } from '_app/shared/ButtonUI';
import BottomMenuLayout, {
    Content,
    Menu,
} from '_app/shared/bottom-menu-layout';
import { Text } from '_app/shared/text';
import { IconTooltip } from '_app/shared/tooltip';
import { AddressInput } from '_components/address-input';
import Loading from '_components/loading';
import { parseAmount } from '_helpers';
import {
    useCoinDecimals,
    useAppSelector,
    useIndividualCoinMaxBalance,
} from '_hooks';
import {
    accountAggregateBalancesSelector,
    accountCoinsSelector,
} from '_redux/slices/account';
import { Coin, GAS_TYPE_ARG } from '_redux/slices/haneul-objects/Coin';
import { useGasBudgetInMist } from '_src/ui/app/hooks/useGasBudgetInMist';
import { InputWithAction } from '_src/ui/app/shared/InputWithAction';

const initialValues = {
    to: '',
    amount: '',
    isPayAllHaneul: false,
    // for gas validation purposes
    // to revalidate when amount changes
    gasInputBudgetEst: null as number | null,
};

export type FormValues = typeof initialValues;

export type SubmitProps = {
    to: string;
    amount: string;
    isPayAllHaneul: boolean;
    coinIds: string[];
    gasBudget: number;
    coins: HaneulMoveObject[];
};

export type SendTokenFormProps = {
    coinType: string;
    onSubmit: (values: SubmitProps) => void;
    initialAmount: string;
    initialTo: string;
    initialGasEstimation: number;
};

function GasBudgetEstimation({
    coinDecimals,
    haneulCoins,
}: {
    coinDecimals: number;
    haneulCoins: HaneulMoveObject[];
}) {
    const { values, setFieldValue } = useFormikContext<FormValues>();
    const gasBudgetEstimationUnits = useMemo(
        () =>
            Coin.computeGasBudgetForPay(
                haneulCoins,
                parseAmount(values.amount, coinDecimals)
            ),
        [coinDecimals, haneulCoins, values.amount]
    );
    const { gasBudget: gasBudgetEstimation, isLoading } = useGasBudgetInMist(
        gasBudgetEstimationUnits
    );

    const [formattedGas, gasSymbol] = useFormatCoin(
        gasBudgetEstimation,
        GAS_TYPE_ARG
    );

    // gasBudgetEstimation should change when the amount above changes
    useEffect(() => {
        setFieldValue('gasInputBudgetEst', gasBudgetEstimation, true);
    }, [gasBudgetEstimation, setFieldValue, values.amount]);

    return (
        <Loading loading={isLoading}>
            <div className="px-2 mt-3 mb-5 flex w-full gap-2 justify-between">
                <div className="flex gap-1">
                    <Text variant="body" color="gray-80" weight="medium">
                        Estimated Gas Fees
                    </Text>
                    <div className="text-gray-60 h-4 items-end flex">
                        <IconTooltip tip="Estimated Gas Fees" placement="top" />
                    </div>
                </div>
                <Text variant="body" color="gray-90" weight="medium">
                    {formattedGas} {gasSymbol}
                </Text>
            </div>
        </Loading>
    );
}

// Set the initial gasEstimation from initial amount
// base on the input amount field update the gasEstimation value
// Separating the gasEstimation from the formik context to access the input amount value and update the gasEstimation value
export function SendTokenForm({
    coinType,
    onSubmit,
    initialAmount = '',
    initialTo = '',
    initialGasEstimation = 0,
}: SendTokenFormProps) {
    const aggregateBalances = useAppSelector(accountAggregateBalancesSelector);
    const coinBalance = useMemo(
        () => (coinType && aggregateBalances[coinType]) || BigInt(0),
        [coinType, aggregateBalances]
    );

    const allCoins = useAppSelector(accountCoinsSelector);
    const coins = allCoins.filter(
        ({ type }) => CoinAPI.getCoinType(type) === coinType
    );

    const haneulCoins = allCoins.filter(
        ({ type }) => CoinAPI.getCoinType(type) === HANEUL_TYPE_ARG
    );

    const gasAggregateBalance = aggregateBalances[HANEUL_TYPE_ARG] || BigInt(0);
    const coinSymbol = (coinType && CoinAPI.getCoinSymbol(coinType)) || '';
    const [coinDecimals, coinDecimalsQueryResult] = useCoinDecimals(coinType);
    const [gasDecimals, gasQueryResult] = useCoinDecimals(HANEUL_TYPE_ARG);
    const maxHaneulSingleCoinBalance = useIndividualCoinMaxBalance(HANEUL_TYPE_ARG);

    const validationSchemaStepOne = useMemo(
        () =>
            createValidationSchemaStepOne(
                coinType || '',
                coinBalance,
                coinSymbol,
                gasAggregateBalance,
                coinDecimals,
                gasDecimals,
                initialGasEstimation,
                maxHaneulSingleCoinBalance
            ),
        [
            coinType,
            coinBalance,
            coinSymbol,
            gasAggregateBalance,
            coinDecimals,
            gasDecimals,
            initialGasEstimation,
            maxHaneulSingleCoinBalance,
        ]
    );

    const [tokenBalance, symbol, queryResult] = useFormatCoin(
        coinBalance,
        coinType,
        CoinFormat.FULL
    );

    // remove the comma from the token balance
    const formattedTokenBalance = tokenBalance.replace(/,/g, '');
    const initAmountBig = parseAmount(initialAmount, coinDecimals);

    return (
        <Loading
            loading={
                queryResult.isLoading ||
                gasQueryResult.isLoading ||
                coinDecimalsQueryResult.isLoading
            }
        >
            <Formik
                initialValues={{
                    amount: initialAmount,
                    to: initialTo,
                    isPayAllHaneul:
                        !!initAmountBig &&
                        initAmountBig === coinBalance &&
                        coinType === HANEUL_TYPE_ARG,
                    gasInputBudgetEst: initialGasEstimation,
                }}
                validationSchema={validationSchemaStepOne}
                enableReinitialize
                validateOnMount
                validateOnChange
                onSubmit={({
                    to,
                    amount,
                    isPayAllHaneul,
                    gasInputBudgetEst,
                }: FormValues) => {
                    if (!gasInputBudgetEst || !coins || !haneulCoins) return;
                    const coinsIDs = CoinAPI.sortByBalance(coins)
                        .reverse()
                        .map((coin) => CoinAPI.getID(coin));

                    const data = {
                        to,
                        amount,
                        isPayAllHaneul,
                        coins: allCoins,
                        coinIds: coinsIDs,
                        gasBudget: gasInputBudgetEst,
                    };
                    onSubmit(data);
                }}
            >
                {({
                    isValid,
                    isSubmitting,
                    setFieldValue,
                    values,
                    submitForm,
                }) => {
                    const newPayHaneulAll =
                        parseAmount(values.amount, coinDecimals) ===
                            coinBalance && coinType === HANEUL_TYPE_ARG;
                    if (values.isPayAllHaneul !== newPayHaneulAll) {
                        setFieldValue('isPayAllHaneul', newPayHaneulAll);
                    }

                    return (
                        <BottomMenuLayout>
                            <Content>
                                <Form autoComplete="off" noValidate>
                                    <div className="w-full flex flex-col flex-grow">
                                        <div className="px-2 mb-2.5">
                                            <Text
                                                variant="caption"
                                                color="steel-dark"
                                                weight="semibold"
                                            >
                                                Select HANEUL Amount to Send
                                            </Text>
                                        </div>

                                        <InputWithAction
                                            name="amount"
                                            placeholder="0.00"
                                            prefix={
                                                values.isPayAllHaneul ? '~ ' : ''
                                            }
                                            actionText="Max"
                                            suffix={` ${symbol}`}
                                            type="number"
                                            actionType="button"
                                            allowNegative={false}
                                            allowDecimals
                                            rounded="lg"
                                            dark
                                            onActionClicked={() =>
                                                // useFormat coin
                                                setFieldValue(
                                                    'amount',
                                                    formattedTokenBalance,
                                                    true
                                                )
                                            }
                                            actionDisabled={
                                                parseAmount(
                                                    values?.amount,
                                                    coinDecimals
                                                ) === coinBalance ||
                                                queryResult.isLoading ||
                                                !coinBalance ||
                                                !values.gasInputBudgetEst
                                            }
                                        />
                                    </div>
                                    <GasBudgetEstimation
                                        coinDecimals={coinDecimals}
                                        haneulCoins={haneulCoins}
                                    />

                                    <div className="w-full flex gap-2.5 flex-col mt-7.5">
                                        <div className="px-2 tracking-wider">
                                            <Text
                                                variant="caption"
                                                color="steel-dark"
                                                weight="semibold"
                                            >
                                                Enter Recipient Address
                                            </Text>
                                        </div>
                                        <div className="w-full flex relative items-center flex-col">
                                            <Field
                                                component={AddressInput}
                                                name="to"
                                                placeholder="Enter Address"
                                            />
                                        </div>
                                    </div>
                                </Form>
                            </Content>
                            <Menu
                                stuckClass="sendCoin-cta"
                                className="w-full px-0 pb-0 mx-0 gap-2.5"
                            >
                                <Button
                                    type="submit"
                                    onClick={submitForm}
                                    variant="primary"
                                    loading={isSubmitting}
                                    disabled={!isValid || isSubmitting}
                                    size="tall"
                                    text="Review"
                                    after={<ArrowRight16 />}
                                />
                            </Menu>
                        </BottomMenuLayout>
                    );
                }}
            </Formik>
        </Loading>
    );
}
