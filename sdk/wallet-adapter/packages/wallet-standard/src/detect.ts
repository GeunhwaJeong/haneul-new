// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  ConnectFeature,
  DisconnectFeature,
  EventsFeature,
} from "@wallet-standard/features";
import { Wallet, WalletWithFeatures } from "@wallet-standard/standard";
import { HaneulSignAndExecuteTransactionFeature } from "./features";

export type StandardWalletAdapterWallet = WalletWithFeatures<
  ConnectFeature &
    EventsFeature &
    HaneulSignAndExecuteTransactionFeature &
    // Disconnect is an optional feature:
    Partial<DisconnectFeature>
>;

export function isStandardWalletAdapterCompatibleWallet(
  wallet: Wallet
): wallet is StandardWalletAdapterWallet {
  return (
    "standard:connect" in wallet.features &&
    "standard:events" in wallet.features &&
    "haneul:signAndExecuteTransaction" in wallet.features
  );
}
