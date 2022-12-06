// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Delegation } from '@haneullabs/haneul.js';
import { useMemo } from 'react';

import { haneulObjectsAdapterSelectors } from '../../redux/slices/haneul-objects';
import { getValidatorSelector } from '../selectors';
import { DelegationCard, DelegationState } from './DelegationCard';
import { useAppSelector } from '_hooks';

import type { RootState } from '_redux/RootReducer';

interface Props {
    id: string;
}

export function ActiveDelegation({ id }: Props) {
    const delegationSelector = useMemo(
        () => (state: RootState) => {
            const haneulObj = haneulObjectsAdapterSelectors.selectById(state, id);
            if (haneulObj && Delegation.isDelegationHaneulObject(haneulObj)) {
                return new Delegation(haneulObj);
            }
            return undefined;
        },
        [id]
    );

    const delegation = useAppSelector(delegationSelector);
    const validatorAddress = delegation?.validatorAddress();
    const validatorSelector = useMemo(
        () => getValidatorSelector(validatorAddress),
        [validatorAddress]
    );
    const validator = useAppSelector(validatorSelector);
    const validatorName = useMemo(() => {
        if (!validator) {
            return null;
        }
        return Buffer.from(validator.fields.name, 'base64').toString();
    }, [validator]);

    if (!validator || !delegation || !validatorName) {
        return null;
    }

    return (
        <DelegationCard
            name={validatorName}
            staked={delegation.delegateAmount()}
            state={DelegationState.EARNING}
        />
    );
}
