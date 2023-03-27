// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    is,
    HaneulObjectData,
    getObjectOwner,
    type HaneulAddress,
} from '@haneullabs/haneul.js';
import { useMemo } from 'react';

import { useGetObject } from './useGetObject';

export function useOwnedNFT(
    nftObjectId: string | null,
    address: HaneulAddress | null
) {
    const data = useGetObject(nftObjectId);
    const { data: objectData } = data;
    const objectDetails = useMemo(() => {
        if (!objectData || !is(objectData.data, HaneulObjectData) || !address)
            return null;
        const objectOwner = getObjectOwner(objectData);
        return objectOwner &&
            objectOwner !== 'Immutable' &&
            'AddressOwner' in objectOwner &&
            objectOwner.AddressOwner === address
            ? objectData.data
            : null;
    }, [address, objectData]);
    return { ...data, data: objectDetails };
}
