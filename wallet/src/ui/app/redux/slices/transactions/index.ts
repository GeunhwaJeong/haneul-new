// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject } from '@haneullabs/haneul.js';
import {
    createAsyncThunk,
    createEntityAdapter,
    createSlice,
} from '@reduxjs/toolkit';

import {
    fetchAllOwnedObjects,
    haneulObjectsAdapterSelectors,
} from '_redux/slices/haneul-objects';
import { Coin } from '_redux/slices/haneul-objects/Coin';

import type {
    HaneulAddress,
    HaneulMoveObject,
    TransactionEffectsResponse,
} from '@haneullabs/haneul.js';
import type { RootState } from '_redux/RootReducer';
import type { AppThunkConfig } from '_store/thunk-extras';

type SendTokensTXArgs = {
    tokenTypeArg: string;
    amount: bigint;
    recipientAddress: HaneulAddress;
};
type TransactionResult = { EffectResponse: TransactionEffectsResponse };

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
        const response = await Coin.transferCoin(
            api.getSignerInstance(keypairVault.getKeyPair()),
            coins,
            amount,
            recipientAddress
        );

        // TODO: better way to sync latest objects
        dispatch(fetchAllOwnedObjects());
        // TODO: is this correct? Find a better way to do it
        return response as TransactionResult;
    }
);

const txAdapter = createEntityAdapter<TransactionResult>({
    selectId: (tx) => tx.EffectResponse.certificate.transactionDigest,
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
    },
});

export default slice.reducer;
