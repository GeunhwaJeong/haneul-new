// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject, getObjectId, getObjectFields } from '@haneullabs/haneul.js';

import useFileExtentionType from './useFileExtentionType';
import useMediaUrl from './useMediaUrl';

import type { HaneulObject } from '@haneullabs/haneul.js';

export default function useNFTBasicData(nftObj: HaneulObject) {
    const nftObjectID = getObjectId(nftObj.reference);
    const filePath = useMediaUrl(nftObj.data);
    let objType = null;
    let nftFields = null;
    if (isHaneulMoveObject(nftObj.data)) {
        objType = nftObj.data.type;
        nftFields = getObjectFields(nftObj.data);
    }
    const fileExtentionType = useFileExtentionType(filePath || '');
    return {
        nftObjectID,
        filePath,
        nftFields,
        fileExtentionType,
        objType,
    };
}
