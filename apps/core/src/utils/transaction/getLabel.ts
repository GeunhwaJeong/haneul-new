// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { HaneulTransactionBlockResponse, getTransactionSender, type HaneulAddress } from '@haneullabs/haneul.js';

// todo: add more logic for deriving transaction label
export const getLabel = (transaction: HaneulTransactionBlockResponse, currentAddress?: HaneulAddress) => {
	const isSender = getTransactionSender(transaction) === currentAddress;
	// Rename to "Send" to Transaction
	return isSender ? 'Transaction' : 'Receive';
};
