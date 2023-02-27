// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import BigNumber from 'bignumber.js';

import type { Validator, DelegatedStake } from '@haneullabs/haneul.js';

export function getStakingRewards(
    activeValidators: Validator[],
    delegation: DelegatedStake
) {
    if (
        !activeValidators ||
        !delegation ||
        delegation.delegation_status === 'Pending'
    )
        return 0;
    const pool_id = delegation.staked_haneul.pool_id;
    const validator = activeValidators.find(
        (validator) => validator.delegation_staking_pool.id === pool_id
    );

    if (!validator) return 0;

    const poolTokens = new BigNumber(
        delegation.delegation_status.Active.pool_tokens.value
    );
    const delegationTokenSupply = new BigNumber(
        validator.delegation_staking_pool.delegation_token_supply.value
    );
    const haneulBalance = new BigNumber(
        validator.delegation_staking_pool.haneul_balance
    );
    const pricipalAmout = new BigNumber(
        delegation.delegation_status.Active.principal_haneul_amount
    );
    const currentHaneulWorth = poolTokens
        .multipliedBy(haneulBalance)
        .dividedBy(delegationTokenSupply);

    const earnToken = currentHaneulWorth.minus(pricipalAmout);
    return earnToken.decimalPlaces(0, BigNumber.ROUND_DOWN).toNumber();
}
