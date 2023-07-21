// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef, HaneulObjectRef } from '@haneullabs/haneul.js';
import { TransactionArgument } from '@haneullabs/haneul.js/transactions';

export * from './kiosk';
export * from './transfer-policy';
export * from './env';

/**
 * A valid argument for any of the Kiosk functions.
 */
export type ObjectArgument = string | TransactionArgument | SharedObjectRef | HaneulObjectRef;
