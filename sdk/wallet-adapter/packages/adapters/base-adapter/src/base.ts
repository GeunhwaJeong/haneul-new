// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  MoveCallTransaction,
  SignableTransaction,
  HaneulAddress,
  HaneulTransactionResponse,
} from "@haneullabs/haneul.js";

export interface WalletCapabilities {
  // Metadata
  name: string;
  connected: boolean;
  connecting: boolean;
  // Connection Management
  connect: () => Promise<void>;
  disconnect: () => Promise<void>;

  /**
   * Suggest a transaction for the user to sign. Supports all valid transaction types.
   */
  signAndExecuteTransaction?(
    transaction: SignableTransaction
  ): Promise<HaneulTransactionResponse>;

  getAccounts: () => Promise<HaneulAddress[]>;

  /** @deprecated Prefer `signAndExecuteTransaction` when available. */
  executeMoveCall: (
    transaction: MoveCallTransaction
  ) => Promise<HaneulTransactionResponse>;

  /** @deprecated Prefer `signAndExecuteTransaction` when available. */
  executeSerializedMoveCall: (
    transactionBytes: Uint8Array
  ) => Promise<HaneulTransactionResponse>;
}

export type WalletAdapter = WalletCapabilities;
