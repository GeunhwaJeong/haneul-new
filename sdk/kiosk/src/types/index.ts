// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from '@haneullabs/haneul.js/bcs';
import { HaneulObjectRef } from '@haneullabs/haneul.js/client';
import { TransactionArgument } from '@haneullabs/haneul.js/transactions';

export * from './kiosk';
export * from './transfer-policy';
export * from './env';

/**
 * A valid argument for any of the Kiosk functions.
 */
export type ObjectArgument = string | TransactionArgument | SharedObjectRef | HaneulObjectRef;
