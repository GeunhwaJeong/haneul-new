// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type HaneulEvent } from '@haneullabs/haneul.js/client';

export function getValidatorMoveEvent(validatorsEvent: HaneulEvent[], validatorAddress: string) {
	const event = validatorsEvent.find(
		({ parsedJson }) =>
			(parsedJson as { validator_address?: unknown })!.validator_address === validatorAddress,
	);

	return event && event.parsedJson;
}
