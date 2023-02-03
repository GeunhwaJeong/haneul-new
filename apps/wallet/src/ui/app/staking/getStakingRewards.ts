// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import BigNumber from 'bignumber.js';

import type { MoveActiveValidator, DelegatedStake } from '@haneullabs/haneul.js';

export function getStakingRewards(
    activeValidators: MoveActiveValidator[],
    delegation: DelegatedStake
) {
    if (
        !activeValidators ||
        !delegation ||
        delegation.delegation_status === 'Pending'
    )
        return 0;
    const validatorAddress = delegation.staked_haneul.validator_address;
    const validator = activeValidators.find(
        ({ fields }) =>
            fields.delegation_staking_pool.fields.validator_address ===
            validatorAddress
    );

    if (!validator) return 0;
    const { fields: validatorFields } = validator;

    const poolTokens = new BigNumber(
        delegation.delegation_status.Active.pool_tokens.value
    );
    const delegationTokenSupply = new BigNumber(
        validatorFields.delegation_staking_pool.fields.delegation_token_supply.fields.value
    );
    const haneulBalance = new BigNumber(
        validatorFields.delegation_staking_pool.fields.haneul_balance
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
