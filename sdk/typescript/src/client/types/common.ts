// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import type { CallArg } from '../../bcs/index.js';

export type HaneulJsonValue = boolean | number | string | CallArg | Array<HaneulJsonValue>;
export type Order = 'ascending' | 'descending';
export type Unsubscribe = () => Promise<boolean>;
