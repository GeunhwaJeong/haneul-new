// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getActiveNetworkHaneulClient } from '_shared/haneul-client';
import { DeepBookClient } from '@haneullabs/deepbook';

export async function getDeepbookClient(): Promise<DeepBookClient> {
	const haneulClient = await getActiveNetworkHaneulClient();
	return new DeepBookClient(haneulClient);
}
