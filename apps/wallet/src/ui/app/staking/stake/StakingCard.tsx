// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useCoinDecimals } from '@haneullabs/core';
import { ArrowLeft16 } from '@haneullabs/icons';
import {
    getTransactionDigest,
    HANEUL_TYPE_ARG,
    type HaneulAddress,
} from '@haneullabs/haneul.js';
import * as Sentry from '@sentry/react';
import { useQueryClient, useMutation } from '@tanstack/react-query';
import { Formik } from 'formik';
import { useCallback, useMemo } from 'react';
import { toast } from 'react-hot-toast';
import { Navigate, useNavigate, useSearchParams } from 'react-router-dom';

import Alert from '../../components/alert';
import { getSignerOperationErrorMessage } from '../../helpers/errorMessages';
import { getDelegationDataByStakeId } from '../getDelegationByStakeId';
import { getStakeHaneulByHaneulId } from '../getStakeHaneulByHaneulId';
import { useGetDelegatedStake } from '../useGetDelegatedStake';
import { useSystemState } from '../useSystemState';
import StakeForm from './StakeForm';
import { UnStakeForm } from './UnstakeForm';
import { ValidatorFormDetail } from './ValidatorFormDetail';
import {
    createStakeTransaction,
    createUnstakeTransaction,
} from './utils/transaction';
import { createValidationSchema } from './utils/validation';
import { useActiveAddress } from '_app/hooks/useActiveAddress';
import { Button } from '_app/shared/ButtonUI';
import BottomMenuLayout, {
    Content,
    Menu,
} from '_app/shared/bottom-menu-layout';
import { Collapse } from '_app/shared/collapse';
import { Text } from '_app/shared/text';
import Loading from '_components/loading';
import { parseAmount } from '_helpers';
import { useSigner, useGetCoinBalance } from '_hooks';
import { Coin } from '_redux/slices/haneul-objects/Coin';
import { trackEvent } from '_src/shared/plausible';

import type { FormikHelpers } from 'formik';

const initialValues = {
    amount: '',
};

export type FormValues = typeof initialValues;

function StakingCard() {
    const coinType = HANEUL_TYPE_ARG;
    const accountAddress = useActiveAddress();
    const { data: haneulBalance, isLoading: loadingHaneulBalances } =
        useGetCoinBalance(coinType, accountAddress);
    const coinBalance = BigInt(haneulBalance?.totalBalance || 0);
    const [searchParams] = useSearchParams();
    const validatorAddress = searchParams.get('address');
    const stakeHaneulIdParams = searchParams.get('staked');
    const unstake = searchParams.get('unstake') === 'true';
    const { data: allDelegation, isLoading } = useGetDelegatedStake(
        accountAddress || ''
    );

    const { data: system, isLoading: validatorsIsloading } = useSystemState();

    const totalTokenBalance = useMemo(() => {
        if (!allDelegation) return 0n;
        // return only the total amount of tokens staked for a specific stakeId
        return getStakeHaneulByHaneulId(allDelegation, stakeHaneulIdParams);
    }, [allDelegation, stakeHaneulIdParams]);

    const stakeData = useMemo(() => {
        if (!allDelegation || !stakeHaneulIdParams) return null;
        // return delegation data for a specific stakeId
        return getDelegationDataByStakeId(allDelegation, stakeHaneulIdParams);
    }, [allDelegation, stakeHaneulIdParams]);

    const coinSymbol = useMemo(
        () => (coinType && Coin.getCoinSymbol(coinType)) || '',
        [coinType]
    );

    const haneulEarned = stakeData?.estimatedReward || 0;

    const [coinDecimals] = useCoinDecimals(coinType);
    // set minimum stake amount to 1 HANEUL
    const minimumStake = parseAmount('1', coinDecimals);

    const validationSchema = useMemo(
        () =>
            createValidationSchema(
                coinBalance,
                coinSymbol,
                coinDecimals,
                unstake,
                minimumStake
            ),
        [coinBalance, coinSymbol, coinDecimals, unstake, minimumStake]
    );

    const queryClient = useQueryClient();
    const delegationId = useMemo(() => {
        if (!stakeData || stakeData.status === 'Pending') return null;
        return stakeData.stakedHaneulId;
    }, [stakeData]);

    const navigate = useNavigate();
    const signer = useSigner();

    const stakeToken = useMutation({
        mutationFn: async ({
            tokenTypeArg,
            amount,
            validatorAddress,
        }: {
            tokenTypeArg: string;
            amount: bigint;
            validatorAddress: HaneulAddress;
        }) => {
            if (!validatorAddress || !amount || !tokenTypeArg || !signer) {
                throw new Error('Failed, missing required field');
            }
            trackEvent('Stake', {
                props: { validator: validatorAddress },
            });
            const sentryTransaction = Sentry.startTransaction({
                name: 'stake',
            });
            try {
                const transactionBlock = createStakeTransaction(
                    amount,
                    validatorAddress
                );
                return await signer.signAndExecuteTransaction({
                    transactionBlock,
                    options: {
                        showInput: true,
                        showEffects: true,
                        showEvents: true,
                    },
                });
            } finally {
                sentryTransaction.finish();
            }
        },
    });

    const unStakeToken = useMutation({
        mutationFn: async ({ stakedHaneulId }: { stakedHaneulId: string }) => {
            if (!stakedHaneulId || !signer) {
                throw new Error('Failed, missing required field.');
            }

            trackEvent('Unstake');

            const sentryTransaction = Sentry.startTransaction({
                name: 'stake',
            });
            try {
                const transactionBlock = createUnstakeTransaction(stakedHaneulId);
                return await signer.signAndExecuteTransaction({
                    transactionBlock,
                    options: {
                        showInput: true,
                        showEffects: true,
                        showEvents: true,
                    },
                });
            } finally {
                sentryTransaction.finish();
            }
        },
    });

    const onHandleSubmit = useCallback(
        async (
            { amount }: FormValues,
            { resetForm }: FormikHelpers<FormValues>
        ) => {
            if (coinType === null || validatorAddress === null) {
                return;
            }
            try {
                const bigIntAmount = parseAmount(amount, coinDecimals);
                let response;
                let txDigest;
                if (unstake) {
                    // check for delegation data
                    if (
                        !stakeData ||
                        !stakeHaneulIdParams ||
                        stakeData.status === 'Pending'
                    ) {
                        return;
                    }
                    response = await unStakeToken.mutateAsync({
                        stakedHaneulId: stakeHaneulIdParams,
                    });

                    txDigest = getTransactionDigest(response);
                } else {
                    response = await stakeToken.mutateAsync({
                        amount: bigIntAmount,
                        tokenTypeArg: coinType,
                        validatorAddress: validatorAddress,
                    });
                    txDigest = getTransactionDigest(response);
                }

                // Invalidate the react query for system state and validator
                Promise.all([
                    queryClient.invalidateQueries({
                        queryKey: ['system', 'state'],
                    }),
                    queryClient.invalidateQueries({
                        queryKey: ['validator'],
                    }),
                ]);
                resetForm();

                navigate(
                    `/receipt?${new URLSearchParams({
                        txdigest: txDigest,
                        from: 'stake',
                    }).toString()}`
                );
            } catch (error) {
                toast.error(
                    <div className="max-w-xs overflow-hidden flex flex-col">
                        <strong>{unstake ? 'Unstake' : 'Stake'} failed</strong>
                        <small className="text-ellipsis overflow-hidden">
                            {getSignerOperationErrorMessage(error)}
                        </small>
                    </div>
                );
            }
        },
        [
            coinType,
            validatorAddress,
            coinDecimals,
            unstake,
            queryClient,
            navigate,
            stakeData,
            stakeHaneulIdParams,
            unStakeToken,
            stakeToken,
        ]
    );

    if (!coinType || !validatorAddress || (!validatorsIsloading && !system)) {
        return <Navigate to="/" replace={true} />;
    }
    return (
        <div className="flex flex-col flex-nowrap flex-grow w-full">
            <Loading
                loading={isLoading || validatorsIsloading || loadingHaneulBalances}
            >
                <Formik
                    initialValues={initialValues}
                    validationSchema={validationSchema}
                    onSubmit={onHandleSubmit}
                    validateOnMount
                >
                    {({
                        isSubmitting,
                        isValid,
                        submitForm,
                        errors,
                        touched,
                    }) => (
                        <BottomMenuLayout>
                            <Content>
                                <div className="mb-4">
                                    <ValidatorFormDetail
                                        validatorAddress={validatorAddress}
                                        unstake={unstake}
                                    />
                                </div>

                                {unstake ? (
                                    <UnStakeForm
                                        stakedHaneulId={stakeHaneulIdParams!}
                                        coinBalance={totalTokenBalance}
                                        coinType={coinType}
                                        stakingReward={haneulEarned}
                                        epoch={system?.epoch || 0}
                                    />
                                ) : (
                                    <StakeForm
                                        validatorAddress={validatorAddress}
                                        coinBalance={coinBalance}
                                        coinType={coinType}
                                        epoch={system?.epoch}
                                    />
                                )}

                                {(unstake || touched.amount) &&
                                errors.amount ? (
                                    <div className="mt-2 flex flex-col flex-nowrap">
                                        <Alert
                                            mode="warning"
                                            className="text-body"
                                        >
                                            {errors.amount}
                                        </Alert>
                                    </div>
                                ) : null}

                                {!unstake && (
                                    <div className="flex-1 mt-7.5">
                                        <Collapse
                                            title="Staking Rewards"
                                            initialIsOpen
                                        >
                                            <Text
                                                variant="p3"
                                                color="steel-dark"
                                                weight="normal"
                                            >
                                                The staked HANEUL starts earning
                                                reward at the end of the Epoch
                                                in which it was staked. The
                                                rewards will become available at
                                                the end of one full Epoch of
                                                staking.
                                            </Text>
                                        </Collapse>
                                    </div>
                                )}
                            </Content>

                            <Menu
                                stuckClass="staked-cta"
                                className="w-full px-0 pb-0 mx-0"
                            >
                                <Button
                                    size="tall"
                                    variant="secondary"
                                    to="/stake"
                                    disabled={isSubmitting}
                                    before={<ArrowLeft16 />}
                                    text="Back"
                                />
                                <Button
                                    size="tall"
                                    variant="primary"
                                    onClick={submitForm}
                                    disabled={
                                        !isValid ||
                                        isSubmitting ||
                                        (unstake && !delegationId)
                                    }
                                    loading={isSubmitting}
                                    text={unstake ? 'Unstake Now' : 'Stake Now'}
                                />
                            </Menu>
                        </BottomMenuLayout>
                    )}
                </Formik>
            </Loading>
        </div>
    );
}

export default StakingCard;
