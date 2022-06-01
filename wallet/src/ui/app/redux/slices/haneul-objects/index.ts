// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getObjectExistsResponse } from '@haneullabs/haneul.js';
import {
    createAsyncThunk,
    createEntityAdapter,
    createSlice,
} from '@reduxjs/toolkit';

import type { HaneulObject } from '@haneullabs/haneul.js';
import type { RootState } from '_redux/RootReducer';
import type { AppThunkConfig } from '_store/thunk-extras';

const objectsAdapter = createEntityAdapter<HaneulObject>({
    selectId: ({ reference }) => reference.objectId,
    sortComparer: (a, b) =>
        a.reference.objectId.localeCompare(b.reference.objectId),
});

export const fetchAllOwnedObjects = createAsyncThunk<
    HaneulObject[],
    void,
    AppThunkConfig
>('haneul-objects/fetch-all', async (_, { getState, extra: { api } }) => {
    const address = getState().account.address;
    const allHaneulObjects: HaneulObject[] = [];
    if (address) {
        const allObjectRefs = await api.instance.getObjectsOwnedByAddress(
            `${address}`
        );
        const objectIDs = allObjectRefs.map((anObj) => anObj.objectId);
        const allObjRes = await api.instance.getObjectBatch(objectIDs);
        for (const objRes of allObjRes) {
            const haneulObj = getObjectExistsResponse(objRes);
            if (haneulObj) {
                allHaneulObjects.push(haneulObj);
            }
        }
    }
    return allHaneulObjects;
});

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
        setOwnedObjects: objectsAdapter.setAll,
    },
    extraReducers: (builder) => {
        builder
            .addCase(fetchAllOwnedObjects.fulfilled, (state, action) => {
                objectsAdapter.setAll(state, action.payload);
                state.loading = false;
                state.error = false;
                state.lastSync = Date.now();
            })
            .addCase(fetchAllOwnedObjects.pending, (state, action) => {
                state.loading = true;
            })
            .addCase(
                fetchAllOwnedObjects.rejected,
                (state, { error: { code, name, message } }) => {
                    state.loading = false;
                    state.error = { code, message, name };
                }
            );
    },
});

export default slice.reducer;

export const haneulObjectsAdapterSelectors = objectsAdapter.getSelectors(
    (state: RootState) => state.haneulObjects
);
