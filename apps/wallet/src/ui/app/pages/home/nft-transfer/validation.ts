// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type JsonRpcProvider } from '@haneullabs/haneul.js';
import * as Yup from 'yup';

import { createHaneulAddressValidation } from '_components/address-input/validation';

export function createValidationSchema(
	rpc: JsonRpcProvider,
	haneulNSEnabled: boolean,
	senderAddress: string,
	objectId: string,
) {
	return Yup.object({
		to: createHaneulAddressValidation(rpc, haneulNSEnabled)
			.test(
				'sender-address',
				// eslint-disable-next-line no-template-curly-in-string
				`NFT is owned by this address`,
				(value) => senderAddress !== value,
			)
			.test(
				'nft-sender-address',
				// eslint-disable-next-line no-template-curly-in-string
				`NFT address must be different from receiver address`,
				(value) => objectId !== value,
			),
	});
}
