// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject, Delegation } from '@haneullabs/haneul.js';
import { createSelector } from '@reduxjs/toolkit';

import { ownedObjects } from '_redux/slices/account';
import { haneulSystemObjectSelector } from '_redux/slices/haneul-objects';

import type { HaneulMoveObject, DelegationHaneulObject } from '@haneullabs/haneul.js';

export const delegationsSelector = createSelector(
    ownedObjects,
    (objects) =>
        objects.filter((obj) =>
            Delegation.isDelegationHaneulObject(obj)
        ) as DelegationHaneulObject[]
);

export const activeDelegationsSelector = createSelector(
    delegationsSelector,
    (delegations) => delegations.filter((obj) => new Delegation(obj).isActive())
);

export const activeDelegationIDsSelector = createSelector(
    activeDelegationsSelector,
    (delegations) => delegations.map(({ reference: { objectId } }) => objectId)
);

export const totalActiveStakedSelector = createSelector(
    activeDelegationsSelector,
    (activeDelegations) =>
        activeDelegations.reduce((total, obj) => {
            total += BigInt(new Delegation(obj).activeDelegation());
            return total;
        }, BigInt(0))
);

export const epochSelector = createSelector(
    haneulSystemObjectSelector,
    (systemObj) =>
        systemObj && isHaneulMoveObject(systemObj.data)
            ? (systemObj.data.fields.epoch as number)
            : null
);

export function getValidatorSelector(validatorAddress?: string) {
    // TODO this is limited only to the active and next set of validators. Is there a way to access the list of all validators?
    return createSelector(haneulSystemObjectSelector, (systemObj) => {
        const { data } = systemObj || {};
        if (isHaneulMoveObject(data)) {
            const { active_validators: active, next_epoch_validators: next } =
                data.fields.validators.fields;
            const validator: HaneulMoveObject | undefined = [
                ...active.map((v: HaneulMoveObject) => v.fields.metadata),
                ...next,
            ].find(
                (aValidator) =>
                    aValidator.fields.haneul_address === validatorAddress
            );
            return validator;
        }
        return undefined;
    });
}
