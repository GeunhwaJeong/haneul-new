// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { MoveCallTransaction, HaneulAddress, TransactionResponse } from "@haneullabs/haneul.js";

export interface WalletCapabilities {
    // Metadata
    name: string;
    connected: boolean;
    connecting: boolean;
    // Connection Management
    connect: () => Promise<void>;
    disconnect: () => Promise<void>;
    // DappInterfaces
    getAccounts: () => Promise<HaneulAddress[]>; 
    executeMoveCall: (transaction: MoveCallTransaction) => Promise<TransactionResponse>;
    executeSerializedMoveCall: (transactionBytes: Uint8Array) => Promise<TransactionResponse>;
}

export type WalletAdapter = WalletCapabilities;