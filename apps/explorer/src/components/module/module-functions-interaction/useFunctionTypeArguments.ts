// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

import type { HaneulMoveAbilitySet } from '@haneullabs/haneul.js/client';

export function useFunctionTypeArguments(typeArguments: HaneulMoveAbilitySet[]) {
	return useMemo(
		() =>
			typeArguments.map(
				(aTypeArgument, index) =>
					`T${index}${
						aTypeArgument.abilities.length ? `: ${aTypeArgument.abilities.join(' + ')}` : ''
					}`,
			),
		[typeArguments],
	);
}
