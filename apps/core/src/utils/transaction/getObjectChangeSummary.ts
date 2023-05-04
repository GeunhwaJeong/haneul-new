// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import {
    type HaneulTransactionBlockResponse,
    type HaneulAddress,
    type DryRunTransactionBlockResponse,
    HaneulObjectChangeTransferred,
    HaneulObjectChangeCreated,
    HaneulObjectChangeMutated,
    HaneulObjectChangePublished,
} from '@haneullabs/haneul.js';

export type ObjectChangeSummary = {
    mutated: HaneulObjectChangeMutated[];
    created: HaneulObjectChangeCreated[];
    transferred: HaneulObjectChangeTransferred[];
    published: HaneulObjectChangePublished[];
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
            change.type === 'created' &&
            (typeof currentAddress === 'undefined' ||
                change.sender === currentAddress)
    ) as HaneulObjectChangeCreated[];

    const transferred = objectChanges.filter(
        (change) => change.type === 'transferred'
    ) as HaneulObjectChangeTransferred[];

    const published = objectChanges.filter(
        (change) => change.type === 'published'
    ) as HaneulObjectChangePublished[];

    return {
        mutated,
        created,
        transferred,
        published,
    };
};
