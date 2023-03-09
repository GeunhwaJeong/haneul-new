// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import BigNumber from 'bignumber.js';

import type { HaneulValidatorSummary, StakeObject } from '@haneullabs/haneul.js';

export function getStakingRewards(
    validator: HaneulValidatorSummary,
    stakes: StakeObject
) {
    if (!validator || !stakes || stakes.status === 'Pending') return 0;

    if (!validator) return 0;

    const poolTokens = new BigNumber(stakes.principal);
    const delegationTokenSupply = new BigNumber(validator.poolTokenBalance);
    const haneulBalance = new BigNumber(validator.stakingPoolHaneulBalance);
    const principalAmount = new BigNumber(stakes.principal);

    const currentHaneulWorth = poolTokens
        .multipliedBy(haneulBalance)
        .dividedBy(delegationTokenSupply);

    const earnToken = currentHaneulWorth.minus(principalAmount);
    return earnToken.decimalPlaces(0, BigNumber.ROUND_DOWN).toNumber();
}
