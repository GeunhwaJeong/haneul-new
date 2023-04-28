// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import {
    type HaneulTransactionBlockResponse,
    type HaneulAddress,
    type DryRunTransactionBlockResponse,
    HaneulObjectChangeTransferred,
    HaneulObjectChangeCreated,
    HaneulObjectChangeMutated,
} from '@haneullabs/haneul.js';

export type ObjectChangeSummary = {
    mutated: HaneulObjectChangeMutated[];
    created: HaneulObjectChangeCreated[];
    transferred: HaneulObjectChangeTransferred[];
};

export const getObjectChangeSummary = (
    transaction: DryRunTransactionBlockResponse | HaneulTransactionBlockResponse,
    currentAddress?: HaneulAddress | null
) => {
    const { objectChanges } = transaction;
    if (!objectChanges) return null;

    const mutated = objectChanges.filter(
        (change) => change.type === 'mutated'
    ) as HaneulObjectChangeMutated[];

    const created = objectChanges.filter(
        (change) =>
            change.type === 'created' && change.sender === currentAddress
    ) as HaneulObjectChangeCreated[];

    const transferred = objectChanges.filter(
        (change) => change.type === 'transferred'
    ) as HaneulObjectChangeTransferred[];

    return {
        mutated,
        created,
        transferred,
    };
};
