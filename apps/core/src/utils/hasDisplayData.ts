// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulObjectResponse } from '@haneullabs/haneul/client';

export const hasDisplayData = (obj: HaneulObjectResponse) => !!obj.data?.display?.data;
