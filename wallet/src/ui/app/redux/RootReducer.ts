// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { combineReducers } from '@reduxjs/toolkit';

import account from './slices/account';
import app from './slices/app';
import haneulObjects from './slices/haneul-objects';
import transactions from './slices/transactions';

const rootReducer = combineReducers({ account, app, haneulObjects, transactions });

export type RootState = ReturnType<typeof rootReducer>;

export default rootReducer;
