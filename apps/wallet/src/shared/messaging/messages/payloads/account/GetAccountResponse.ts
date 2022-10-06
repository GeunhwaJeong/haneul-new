// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulAddress } from '@haneullabs/haneul.js';
import type { BasePayload } from '_payloads';

export interface GetAccountResponse extends BasePayload {
    type: 'get-account-response';
    accounts: HaneulAddress[];
}
