// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

import { getNormalizedFunctionParameterTypeDetails } from '../utils';

import type { HaneulMoveNormalizedType } from '@haneullabs/haneul.js/client';

export function useFunctionParamsDetails(
	params: HaneulMoveNormalizedType[],
	functionTypeArgNames?: string[],
) {
	return useMemo(
		() =>
			params
				.map((aParam) => getNormalizedFunctionParameterTypeDetails(aParam, functionTypeArgNames))
				.filter(({ isTxContext }) => !isTxContext),
		[params, functionTypeArgNames],
	);
}
