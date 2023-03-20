// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { JsonRpcProvider, localnetConnection } from '@haneullabs/haneul.js';

export const provider = new JsonRpcProvider(localnetConnection);
