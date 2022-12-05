// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject, getObjectId, getObjectFields } from '@haneullabs/haneul.js';

import useFileExtensionType from './useFileExtensionType';
import useMediaUrl from './useMediaUrl';

import type { HaneulObject } from '@haneullabs/haneul.js';

export default function useNFTBasicData(nftObj: HaneulObject | null) {
    const nftObjectID = (nftObj && getObjectId(nftObj.reference)) || null;
    const filePath = useMediaUrl(nftObj?.data || null);
    let objType = null;
    let nftFields = null;
    if (nftObj && isHaneulMoveObject(nftObj.data)) {
        objType = nftObj.data.type;
        nftFields = getObjectFields(nftObj.data);
    }
    const fileExtensionType = useFileExtensionType(filePath || '');
    return {
        nftObjectID,
        filePath,
        nftFields,
        fileExtensionType,
        objType,
    };
}
