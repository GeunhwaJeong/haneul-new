// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { createHaneulAddressValidation } from '_components/address-input/validation';
import { createTokenValidation } from '_src/shared/validation';
import { type HaneulClient } from '@haneullabs/haneul.js/client';
import * as Yup from 'yup';

export function createValidationSchemaStepOne(
	client: HaneulClient,
	haneulNSEnabled: boolean,
	...args: Parameters<typeof createTokenValidation>
) {
	return Yup.object({
		to: createHaneulAddressValidation(client, haneulNSEnabled),
		amount: createTokenValidation(...args),
	});
}
