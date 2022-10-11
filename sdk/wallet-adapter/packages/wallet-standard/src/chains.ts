// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/** Haneul Devnet */
export const HANEUL_DEVNET_CHAIN = "haneul:devnet";

/** Haneul Testnet */
export const HANEUL_TESTNET_CHAIN = "haneul:testnet";

/** Haneul Localnet */
export const HANEUL_LOCALNET_CHAIN = "haneul:localnet";

export const HANEUL_CHAINS = [
  HANEUL_DEVNET_CHAIN,
  HANEUL_TESTNET_CHAIN,
  HANEUL_LOCALNET_CHAIN,
] as const;

export type HaneulChain =
  | typeof HANEUL_DEVNET_CHAIN
  | typeof HANEUL_TESTNET_CHAIN
  | typeof HANEUL_LOCALNET_CHAIN;
