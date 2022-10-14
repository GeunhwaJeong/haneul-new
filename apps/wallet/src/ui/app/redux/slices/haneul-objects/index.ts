// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    getExecutionStatusType,
    getObjectExistsResponse,
    getTimestampFromTransactionResponse,
    getTotalGasUsed,
    getTransactionDigest,
} from '@haneullabs/haneul.js';
import {
    createAsyncThunk,
    createEntityAdapter,
    createSlice,
} from '@reduxjs/toolkit';

import { HANEUL_SYSTEM_STATE_OBJECT_ID } from './Coin';
import { ExampleNFT } from './NFT';
import { FEATURES } from '_src/ui/app/experimentation/features';

import type {
    HaneulObject,
    HaneulAddress,
    ObjectId,
    HaneulExecuteTransactionResponse,
    HaneulTransactionResponse,
} from '@haneullabs/haneul.js';
import type { RootState } from '_redux/RootReducer';
import type { AppThunkConfig } from '_store/thunk-extras';

const objectsAdapter = createEntityAdapter<HaneulObject>({
    selectId: ({ reference }) => reference.objectId,
    sortComparer: (a, b) =>
        a.reference.objectId.localeCompare(b.reference.objectId),
});

export const fetchAllOwnedAndRequiredObjects = createAsyncThunk<
    HaneulObject[],
    void,
    AppThunkConfig
>('haneul-objects/fetch-all', async (_, { getState, extra: { api } }) => {
    const address = getState().account.address;
    const allHaneulObjects: HaneulObject[] = [];
    if (address) {
        const allObjectRefs =
            await api.instance.fullNode.getObjectsOwnedByAddress(`${address}`);
        const objectIDs = allObjectRefs.map((anObj) => anObj.objectId);
        objectIDs.push(HANEUL_SYSTEM_STATE_OBJECT_ID);
        const allObjRes = await api.instance.fullNode.getObjectBatch(objectIDs);
        for (const objRes of allObjRes) {
            const haneulObj = getObjectExistsResponse(objRes);
            if (haneulObj) {
                allHaneulObjects.push(haneulObj);
            }
        }
    }
    return allHaneulObjects;
});

export const batchFetchObject = createAsyncThunk<
    HaneulObject[],
    ObjectId[],
    AppThunkConfig
>('haneul-objects/batch', async (objectIDs, { extra: { api } }) => {
    const allHaneulObjects: HaneulObject[] = [];
    const allObjRes = await api.instance.fullNode.getObjectBatch(objectIDs);
    for (const objRes of allObjRes) {
        const haneulObj = getObjectExistsResponse(objRes);
        if (haneulObj) {
            allHaneulObjects.push(haneulObj);
        }
    }
    return allHaneulObjects;
});

export const mintDemoNFT = createAsyncThunk<void, void, AppThunkConfig>(
    'mintDemoNFT',
    async (_, { extra: { api, keypairVault, featureGating }, dispatch }) => {
        const signer = api.getSignerInstance(keypairVault.getKeyPair());
        if (featureGating.isOn(FEATURES.DEPRECATE_GATEWAY)) {
            await ExampleNFT.mintExampleNFTWithFullnode(signer);
        } else {
            await ExampleNFT.mintExampleNFT(signer);
        }

        await dispatch(fetchAllOwnedAndRequiredObjects());
    }
);

type NFTTxResponse = {
    timestamp_ms?: number;
    status?: string;
    gasFee?: number;
    txId?: string;
};

export const transferHaneulNFT = createAsyncThunk<
    NFTTxResponse,
    { nftId: ObjectId; recipientAddress: HaneulAddress; transferCost: number },
    AppThunkConfig
>(
    'transferHaneulNFT',
    async (data, { extra: { api, keypairVault, featureGating }, dispatch }) => {
        let txn: HaneulTransactionResponse | HaneulExecuteTransactionResponse;
        const signer = api.getSignerInstance(keypairVault.getKeyPair());
        if (featureGating.isOn(FEATURES.DEPRECATE_GATEWAY)) {
            txn = await ExampleNFT.TransferNFTWithFullnode(
                signer,
                data.nftId,
                data.recipientAddress,
                data.transferCost
            );
        } else {
            txn = await ExampleNFT.TransferNFT(
                signer,
                data.nftId,
                data.recipientAddress,
                data.transferCost
            );
        }

        await dispatch(fetchAllOwnedAndRequiredObjects());
        const txnResp = {
            timestamp_ms: getTimestampFromTransactionResponse(txn),
            status: getExecutionStatusType(txn),
            gasFee: txn ? getTotalGasUsed(txn) : 0,
            txId: getTransactionDigest(txn),
        };

        return txnResp as NFTTxResponse;
    }
);
interface HaneulObjectsManualState {
    loading: boolean;
    error: false | { code?: string; message?: string; name?: string };
    lastSync: number | null;
}
const initialState = objectsAdapter.getInitialState<HaneulObjectsManualState>({
    loading: true,
    error: false,
    lastSync: null,
});

const slice = createSlice({
    name: 'haneul-objects',
    initialState: initialState,
    reducers: {
        clearForNetworkSwitch: (state) => {
            state.error = false;
            state.lastSync = null;
            objectsAdapter.removeAll(state);
        },
    },
    extraReducers: (builder) => {
        builder
            .addCase(
                fetchAllOwnedAndRequiredObjects.fulfilled,
                (state, action) => {
                    objectsAdapter.setAll(state, action.payload);
                    state.loading = false;
                    state.error = false;
                    state.lastSync = Date.now();
                }
            )
            .addCase(
                fetchAllOwnedAndRequiredObjects.pending,
                (state, action) => {
                    state.loading = true;
                }
            )
            .addCase(
                fetchAllOwnedAndRequiredObjects.rejected,
                (state, { error: { code, name, message } }) => {
                    state.loading = false;
                    state.error = { code, message, name };
                }
            );
    },
});

export default slice.reducer;

export const { clearForNetworkSwitch } = slice.actions;

export const haneulObjectsAdapterSelectors = objectsAdapter.getSelectors(
    (state: RootState) => state.haneulObjects
);

export const haneulSystemObjectSelector = (state: RootState) =>
    haneulObjectsAdapterSelectors.selectById(state, HANEUL_SYSTEM_STATE_OBJECT_ID);
