// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getObjectFields, isHaneulObject } from '@haneullabs/haneul.js';
import { useMemo } from 'react';

import { useGetObjectData } from './useGetObjectData';

export type NFTMetadata = {
    name: string | null;
    description: string | null;
    url: string;
};

export function useGetNFTMeta(objectID: string): NFTMetadata | null {
    const { data, isError } = useGetObjectData(objectID);

    const nftMeta = useMemo(() => {
        if (isError) return null;

        const { details } = data || {};
        if (!isHaneulObject(details) || !data) return null;
        const fields = getObjectFields(data);
        if (!fields?.url) return null;
        return {
            description:
                typeof fields.description === 'string'
                    ? fields.description
                    : null,
            name: typeof fields.name === 'string' ? fields.name : null,
            url: fields.url,
        };
    }, [data, isError]);

    return nftMeta;
}
