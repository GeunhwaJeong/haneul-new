// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IdentifierString } from '@wallet-standard/core';

/** Haneul Devnet */
export const HANEUL_DEVNET_CHAIN = 'haneul:devnet';

/** Haneul Testnet */
export const HANEUL_TESTNET_CHAIN = 'haneul:testnet';

/** Haneul Localnet */
export const HANEUL_LOCALNET_CHAIN = 'haneul:localnet';

/** Haneul Mainnet */
export const HANEUL_MAINNET_CHAIN = 'haneul:mainnet';

export const HANEUL_CHAINS = [
	HANEUL_DEVNET_CHAIN,
	HANEUL_TESTNET_CHAIN,
	HANEUL_LOCALNET_CHAIN,
	HANEUL_MAINNET_CHAIN,
] as const;

export type HaneulChain =
	| typeof HANEUL_DEVNET_CHAIN
	| typeof HANEUL_TESTNET_CHAIN
	| typeof HANEUL_LOCALNET_CHAIN
	| typeof HANEUL_MAINNET_CHAIN;

/**
 * Utility that returns whether or not a chain identifier is a valid Haneul chain.
 * @param chain a chain identifier in the form of `${string}:{$string}`
 */
export function isHaneulChain(chain: IdentifierString): chain is HaneulChain {
	return HANEUL_CHAINS.includes(chain as HaneulChain);
}
