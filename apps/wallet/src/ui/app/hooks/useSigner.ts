// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useActiveAddress } from './useActiveAddress';
import { thunkExtras } from '_redux/store/thunk-extras';

import type { HaneulAddress } from '@haneullabs/haneul.js';

export function useSigner(address?: HaneulAddress) {
    const activeAddress = useActiveAddress();
    const signerAddress = address || activeAddress;
    const { api, background } = thunkExtras;
    if (!signerAddress) {
        return null;
    }
    return api.getSignerInstance(signerAddress, background);
}
