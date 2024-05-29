// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useFeatureIsOn } from '@growthbook/growthbook-react';
import { normalizeHaneulNSName } from '@haneullabs/haneul/utils';

import { useResolveHaneulNSName as useResolveHaneulNSNameCore } from '../../../../../core';

export function useResolveHaneulNSName(address?: string) {
	const enableNewHaneulnsFormat = useFeatureIsOn('wallet-enable-new-haneulns-name-format');
	const { data } = useResolveHaneulNSNameCore(address);
	return data ? normalizeHaneulNSName(data, enableNewHaneulnsFormat ? 'at' : 'dot') : undefined;
}
