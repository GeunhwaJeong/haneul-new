// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getObjectType, getMoveObjectType } from '@haneullabs/haneul.js';

import type { GetObjectInfoResponse } from '@haneullabs/haneul.js';

export function parseImageURL(data: any): string {
    return (
        data?.url ||
        // TODO: Remove Legacy format
        data?.display ||
        data?.contents?.display ||
        ''
    );
}

export function parseObjectType(data: GetObjectInfoResponse): string {
    // TODO: define better naming and typing here
    const dataType = getObjectType(data);
    if (dataType === 'package') {
        return 'Move Package';
    }
    if (dataType === 'moveObject') {
        return getMoveObjectType(data)!;
    }
    return 'unknown';
}
