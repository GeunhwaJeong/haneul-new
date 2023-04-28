// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import {
    HaneulTransactionBlockResponse,
    getTransactionSender,
} from '@haneullabs/haneul.js';

// todo: add more logic for deriving transaction label
export const getLabel = (transaction: HaneulTransactionBlockResponse) => {
    const isSender = getTransactionSender(transaction);

    return isSender ? 'Send' : 'Receive';
};
