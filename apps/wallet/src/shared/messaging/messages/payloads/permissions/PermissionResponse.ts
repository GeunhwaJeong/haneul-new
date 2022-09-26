// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isBasePayload } from '_payloads';

import type { HaneulAddress } from '@haneullabs/haneul.js';
import type { BasePayload, Payload } from '_payloads';

export interface PermissionResponse extends BasePayload {
    type: 'permission-response';
    id: string;
    accounts: HaneulAddress[];
    allowed: boolean;
    responseDate: string;
}

export function isPermissionResponse(
    payload: Payload
): payload is PermissionResponse {
    return isBasePayload(payload) && payload.type === 'permission-response';
}
