// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulObjectResponse, getObjectDisplay } from '@haneullabs/haneul.js';

export const hasDisplayData = (obj: HaneulObjectResponse) => !!getObjectDisplay(obj).data;
