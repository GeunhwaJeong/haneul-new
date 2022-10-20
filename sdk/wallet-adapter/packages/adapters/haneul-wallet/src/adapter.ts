// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  SignableTransaction,
  HaneulAddress,
  HaneulTransactionResponse,
} from "@haneullabs/haneul.js";
import { WalletAdapter } from "@haneullabs/wallet-adapter-base";

const ALL_PERMISSION_TYPES = ["viewAccount", "suggestTransactions"] as const;
type AllPermissionsType = typeof ALL_PERMISSION_TYPES;
type PermissionType = AllPermissionsType[number];

interface HaneulWallet {
  hasPermissions(permissions: readonly PermissionType[]): Promise<boolean>;
  requestPermissions(): Promise<boolean>;
  getAccounts(): Promise<HaneulAddress[]>;
  signAndExecuteTransaction: (
    transaction: SignableTransaction
  ) => Promise<HaneulTransactionResponse>;
}

interface HaneulWalletWindow {
  haneulWallet: HaneulWallet;
}

declare const window: HaneulWalletWindow;

/**
 * @deprecated This wallet adapter has been replaced by the `WalletStandardAdapterProvider`.
 */
export class HaneulWalletAdapter implements WalletAdapter {
  connecting: boolean;
  connected: boolean;

  getAccounts(): Promise<string[]> {
    return window.haneulWallet.getAccounts();
  }

  signAndExecuteTransaction(
    transaction: SignableTransaction
  ): Promise<HaneulTransactionResponse> {
    return window.haneulWallet.signAndExecuteTransaction(transaction);
  }

  name = "Haneul Wallet (legacy)";

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

  constructor() {
    this.connected = false;
    this.connecting = false;
  }
}
