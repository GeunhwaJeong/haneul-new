// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject } from '@haneullabs/haneul.js';
import { useMemo } from 'react';

import type { HaneulData } from '@haneullabs/haneul.js';

export default function useMediaUrl(objData: HaneulData) {
    const { fields } = (isHaneulMoveObject(objData) && objData) || {};
    return useMemo(() => {
        if (fields) {
            const mediaUrl = fields.url || fields.metadata?.fields.uri;
            if (typeof mediaUrl === 'string') {
                return mediaUrl.replace(/^ipfs:\/\//, 'https://ipfs.io/ipfs/');
            }
        }
        return null;
    }, [fields]);
}
