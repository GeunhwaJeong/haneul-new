// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { WalletWithFeatures } from "@wallet-standard/standard";
import type { HaneulSignAndExecuteTransactionFeature } from "./haneulSignAndExecuteTransaction";

/**
 * Wallet Standard features that are unique to Haneul, and that all Haneul wallets are expected to implement.
 */
export type HaneulFeatures = HaneulSignAndExecuteTransactionFeature;

export type WalletWithHaneulFeatures = WalletWithFeatures<HaneulFeatures>;

export * from "./haneulSignAndExecuteTransaction";
