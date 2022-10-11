// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  MoveCallTransaction,
  HaneulAddress,
  HaneulTransactionResponse,
} from "@haneullabs/haneul.js";
import { WalletAdapter } from "@haneullabs/wallet-adapter-base";

const ALL_PERMISSION_TYPES = ["viewAccount", "suggestTransactions"];
type AllPermissionsType = typeof ALL_PERMISSION_TYPES;
type PermissionType = AllPermissionsType[number];

interface HaneulWallet {
  hasPermissions(permissions: readonly PermissionType[]): Promise<boolean>;
  requestPermissions(): Promise<boolean>;
  getAccounts(): Promise<HaneulAddress[]>;
  executeMoveCall: (
    transaction: MoveCallTransaction
  ) => Promise<HaneulTransactionResponse>;
  executeSerializedMoveCall: (
    transactionBytes: Uint8Array
  ) => Promise<HaneulTransactionResponse>;
}
interface HaneulWalletWindow {
  haneulWallet: HaneulWallet;
}

declare const window: HaneulWalletWindow;

// Stored as state somewhere (Probably in a place with generics )
export class MockWalletAdapter implements WalletAdapter {
  connecting: boolean;
  connected: boolean;

  getAccounts(): Promise<string[]> {
    return window.haneulWallet.getAccounts();
  }
  executeMoveCall(
    transaction: MoveCallTransaction
  ): Promise<HaneulTransactionResponse> {
    return window.haneulWallet.executeMoveCall(transaction);
  }
  executeSerializedMoveCall(
    transactionBytes: Uint8Array
  ): Promise<HaneulTransactionResponse> {
    return window.haneulWallet.executeSerializedMoveCall(transactionBytes);
  }

  name: string;

  async connect(): Promise<void> {
    this.connecting = true;
    if (window.haneulWallet) {
      const wallet = window.haneulWallet;
      try {
        let given = await wallet.requestPermissions();
        const newLocal: readonly PermissionType[] = ["viewAccount"];
        let perms = await wallet.hasPermissions(newLocal);
        console.log(perms);
        console.log(given);
        this.connected = true;
      } catch (err) {
        console.error(err);
      } finally {
        this.connecting = false;
      }
    }
  }

  // Come back to this later
  async disconnect(): Promise<void> {
    if (this.connected == true) {
      this.connected = false;
    }
    console.log("disconnected");
  }

  constructor(name: string) {
    this.connected = false;
    this.connecting = false;
    this.name = name;
  }
}
