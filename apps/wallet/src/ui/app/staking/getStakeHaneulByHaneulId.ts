// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type DelegatedStake } from '@haneullabs/haneul/client';

// Get Stake HANEUL by stakeHaneulId
export const getStakeHaneulByHaneulId = (allDelegation: DelegatedStake[], stakeHaneulId?: string | null) => {
	return (
		allDelegation.reduce((acc, curr) => {
			const total = BigInt(
				curr.stakes.find(({ stakedHaneulId }) => stakedHaneulId === stakeHaneulId)?.principal || 0,
			);
			return total + acc;
		}, 0n) || 0n
	);
};
