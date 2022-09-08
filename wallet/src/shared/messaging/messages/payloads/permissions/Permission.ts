// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { PermissionType } from './PermissionType';
import type { HaneulAddress } from '@haneullabs/haneul.js';

export interface Permission {
    id: string;
    origin: string;
    favIcon: string | undefined;
    accounts: HaneulAddress[];
    allowed: boolean | null;
    permissions: PermissionType[];
    createdDate: string;
    responseDate: string | null;
    requestMsgID: string;
}
