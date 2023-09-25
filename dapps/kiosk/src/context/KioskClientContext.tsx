// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { KioskClient } from '@haneullabs/kiosk';
import { createContext, useContext } from 'react';

export const KioskClientContext = createContext<KioskClient | undefined>(undefined);

export function useKioskClient() {
	const kioskClient = useContext(KioskClientContext);
	if (!kioskClient) {
		throw new Error('kioskClient not setup properly.');
	}
	return kioskClient;
}
