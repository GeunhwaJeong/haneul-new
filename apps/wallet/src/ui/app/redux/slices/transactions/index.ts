// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getTransactionDigest, isHaneulMoveObject } from '@haneullabs/haneul.js';
import {
    createAsyncThunk,
    createEntityAdapter,
    createSlice,
} from '@reduxjs/toolkit';

import {
    fetchAllOwnedAndRequiredObjects,
    haneulObjectsAdapterSelectors,
} from '_redux/slices/haneul-objects';
import { Coin } from '_redux/slices/haneul-objects/Coin';

import type {
    HaneulAddress,
    HaneulMoveObject,
    HaneulExecuteTransactionResponse,
} from '@haneullabs/haneul.js';
import type { RootState } from '_redux/RootReducer';
import type { AppThunkConfig } from '_store/thunk-extras';

type SendTokensTXArgs = {
    tokenTypeArg: string;
    amount: bigint;
    recipientAddress: HaneulAddress;
};
type TransactionResult = HaneulExecuteTransactionResponse;

export const sendTokens = createAsyncThunk<
    TransactionResult,
    SendTokensTXArgs,
    AppThunkConfig
>(
    'haneul-objects/send-tokens',
    async (
        { tokenTypeArg, amount, recipientAddress },
        { getState, extra: { api, keypairVault }, dispatch }
    ) => {
        const state = getState();
        const coinType = Coin.getCoinTypeFromArg(tokenTypeArg);
        const coins: HaneulMoveObject[] = haneulObjectsAdapterSelectors
            .selectAll(state)
            .filter(
                (anObj) =>
                    isHaneulMoveObject(anObj.data) && anObj.data.type === coinType
            )
            .map(({ data }) => data as HaneulMoveObject);

        const signer = api.getSignerInstance(keypairVault.getKeyPair());

        const response =
            Coin.getCoinSymbol(tokenTypeArg) === 'HANEUL'
                ? await Coin.transferHaneul(
                      signer,
                      coins,
                      amount,
                      recipientAddress
                  )
                : await Coin.transferCoin(
                      signer,
                      coins,
                      amount,
                      recipientAddress
                  );

        // TODO: better way to sync latest objects
        dispatch(fetchAllOwnedAndRequiredObjects());
        // TODO: is this correct? Find a better way to do it
        return response;
    }
);

type StakeTokensTXArgs = {
    tokenTypeArg: string;
    amount: bigint;
};

export const StakeTokens = createAsyncThunk<
    TransactionResult,
    StakeTokensTXArgs,
    AppThunkConfig
>(
    'haneul-objects/stake',
    async (
        { tokenTypeArg, amount },
        { getState, extra: { api, keypairVault }, dispatch }
    ) => {
        const state = getState();
        const coinType = Coin.getCoinTypeFromArg(tokenTypeArg);

        const coins: HaneulMoveObject[] = haneulObjectsAdapterSelectors
            .selectAll(state)
            .filter(
                (anObj) =>
                    isHaneulMoveObject(anObj.data) && anObj.data.type === coinType
            )
            .map(({ data }) => data as HaneulMoveObject);

        // TODO: fetch the first active validator for now,
        // repalce it with the user picked one
        const activeValidators = await Coin.getActiveValidators(
            api.instance.fullNode
        );
        const first_validator = activeValidators[0];
        const metadata = (first_validator as HaneulMoveObject).fields.metadata;
        const validatorAddress = (metadata as HaneulMoveObject).fields.haneul_address;
        const response = await Coin.stakeCoin(
            api.getSignerInstance(keypairVault.getKeyPair()),
            coins,
            amount,
            validatorAddress
        );
        dispatch(fetchAllOwnedAndRequiredObjects());
        return response;
    }
);

const txAdapter = createEntityAdapter<TransactionResult>({
    selectId: (tx) => getTransactionDigest(tx),
});

export const txSelectors = txAdapter.getSelectors(
    (state: RootState) => state.transactions
);

const slice = createSlice({
    name: 'transactions',
    initialState: txAdapter.getInitialState(),
    reducers: {},
    extraReducers: (builder) => {
        builder.addCase(sendTokens.fulfilled, (state, { payload }) => {
            return txAdapter.setOne(state, payload);
        });
        builder.addCase(StakeTokens.fulfilled, (state, { payload }) => {
            return txAdapter.setOne(state, payload);
        });
    },
});

export default slice.reducer;
