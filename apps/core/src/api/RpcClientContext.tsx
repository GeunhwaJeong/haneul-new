// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulClient } from '@haneullabs/haneul.js/client';
import { createContext, useContext } from 'react';

export const RpcClientContext = createContext<HaneulClient | undefined>(undefined);

export function useRpcClient() {
	const rpcClient = useContext(RpcClientContext);
	if (!rpcClient) {
		throw new Error('useRpcClient must be within RpcClientContext');
	}
	return rpcClient;
}
