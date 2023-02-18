// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  ConnectFeature,
  DisconnectFeature,
  EventsFeature,
  Wallet,
  WalletWithFeatures,
} from "@wallet-standard/core";
import { HaneulFeatures } from "./features";

export type StandardWalletAdapterWallet = WalletWithFeatures<
  ConnectFeature &
    EventsFeature &
    HaneulFeatures &
    // Disconnect is an optional feature:
    Partial<DisconnectFeature>
>;

// TODO: Enable filtering by subset of features:
export function isStandardWalletAdapterCompatibleWallet(
  wallet: Wallet
): wallet is StandardWalletAdapterWallet {
  return (
    "standard:connect" in wallet.features &&
    "standard:events" in wallet.features &&
    // TODO: Enable once ecosystem wallets adopt this:
    // "haneul:signTransaction" in wallet.features &&
    "haneul:signAndExecuteTransaction" in wallet.features
  );
}
