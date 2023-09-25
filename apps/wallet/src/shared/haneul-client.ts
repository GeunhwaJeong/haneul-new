// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import networkEnv from '_src/background/NetworkEnv';
import { API_ENV, ENV_TO_API, type NetworkEnvType } from '_src/shared/api-env';
import { SentryHttpTransport } from '@haneullabs/core';
import { HaneulClient, HaneulHTTPTransport } from '@haneullabs/haneul.js/client';

const haneulClientPerNetwork = new Map<string, HaneulClient>();
const SENTRY_MONITORED_ENVS = [API_ENV.mainnet];

export function getHaneulClient({ env, customRpcUrl }: NetworkEnvType): HaneulClient {
	const key = `${env}_${customRpcUrl}`;
	if (!haneulClientPerNetwork.has(key)) {
		const connection = customRpcUrl ? customRpcUrl : ENV_TO_API[env];
		if (!connection) {
			throw new Error(`API url not found for network env ${env} ${customRpcUrl}`);
		}
		haneulClientPerNetwork.set(
			key,
			new HaneulClient({
				transport:
					!customRpcUrl && SENTRY_MONITORED_ENVS.includes(env)
						? new SentryHttpTransport(connection)
						: new HaneulHTTPTransport({ url: connection }),
			}),
		);
	}
	return haneulClientPerNetwork.get(key)!;
}

export async function getActiveNetworkHaneulClient(): Promise<HaneulClient> {
	return getHaneulClient(await networkEnv.getActiveNetwork());
}
