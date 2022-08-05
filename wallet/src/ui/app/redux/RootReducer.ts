// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { combineReducers } from '@reduxjs/toolkit';

import account from './slices/account';
import app from './slices/app';
import permissions from './slices/permissions';
import selectedNft from './slices/selected-nft';
import haneulObjects from './slices/haneul-objects';
import transactionRequests from './slices/transaction-requests';
import transactions from './slices/transactions';
import txresults from './slices/txresults';

const rootReducer = combineReducers({
    account,
    app,
    haneulObjects,
    transactions,
    txresults,
    permissions,
    transactionRequests,
    selectedNft,
});

export type RootState = ReturnType<typeof rootReducer>;

export default rootReducer;
