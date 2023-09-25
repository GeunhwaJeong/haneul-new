// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { DEFAULT_API_ENV } from '_app/ApiProvider';
import { API_ENV, type NetworkEnvType } from '_src/shared/api-env';
import { isValidUrl } from '_src/shared/utils';
import mitt from 'mitt';
import Browser from 'webextension-polyfill';

class NetworkEnv {
	#events = mitt<{ changed: NetworkEnvType }>();

	async getActiveNetwork(): Promise<NetworkEnvType> {
		const { haneul_Env, haneul_Env_RPC } = await Browser.storage.local.get({
			haneul_Env: DEFAULT_API_ENV,
			haneul_Env_RPC: null,
		});
		const adjCustomUrl = haneul_Env === API_ENV.customRPC ? haneul_Env_RPC : null;
		return { env: haneul_Env, customRpcUrl: adjCustomUrl };
	}

	async setActiveNetwork(network: NetworkEnvType) {
		const { env, customRpcUrl } = network;
		if (env === API_ENV.customRPC && !isValidUrl(customRpcUrl)) {
			throw new Error(`Invalid custom RPC url ${customRpcUrl}`);
		}
		await Browser.storage.local.set({
			haneul_Env: env,
			haneul_Env_RPC: customRpcUrl,
		});
		this.#events.emit('changed', network);
	}

	on = this.#events.on;

	off = this.#events.off;
}

const networkEnv = new NetworkEnv();
export default networkEnv;
