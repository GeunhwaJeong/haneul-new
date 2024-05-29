// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { DelegatedStake } from '@haneullabs/haneul/client';

// Helper function to get the delegation by stakedHaneulId
export const getDelegationDataByStakeId = (
	delegationsStake: DelegatedStake[],
	stakeHaneulId: string,
) => {
	let stake = null;
	for (const { stakes } of delegationsStake) {
		stake = stakes.find(({ stakedHaneulId }) => stakedHaneulId === stakeHaneulId) || null;
		if (stake) return stake;
	}

	return stake;
};
