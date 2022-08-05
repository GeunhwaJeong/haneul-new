// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    createSlice,
    // createSelector,
    createEntityAdapter,
} from '@reduxjs/toolkit';

import type { HaneulObject } from '@haneullabs/haneul.js';
import type { PayloadAction } from '@reduxjs/toolkit';

export type ActiveNFT = {
    data?: HaneulObject;
    loaded: boolean;
};
const initialState: ActiveNFT = {
    loaded: false,
};
const selectedNFTAdapter = createEntityAdapter<ActiveNFT>({});

const selectedNft = createSlice({
    name: 'selected-nft',
    initialState: selectedNFTAdapter.getInitialState(initialState),
    reducers: {
        setSelectedNFT: (state, { payload }: PayloadAction<ActiveNFT>) => {
            state.data = payload.data;
            state.loaded = payload.loaded;
        },
        clearActiveNFT: (state) => {
            state.data = undefined;
            state.loaded = false;
        },
    },
});
export const { setSelectedNFT, clearActiveNFT } = selectedNft.actions;
export default selectedNft.reducer;
