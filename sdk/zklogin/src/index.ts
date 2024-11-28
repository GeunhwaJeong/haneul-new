// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import '@haneullabs/haneul/zklogin';

import type { ComputeZkLoginAddressOptions } from '@haneullabs/haneul/zklogin';
import {
	computeZkLoginAddress as haneulComputeZkLoginAddress,
	jwtToAddress as haneulJwtToAddress,
} from '@haneullabs/haneul/zklogin';

export type { ComputeZkLoginAddressOptions } from '@haneullabs/haneul/zklogin';

export {
	/** @deprecated, use `import { genAddressSeed } from '@haneullabs/haneul/zklogin';` instead */
	genAddressSeed,
	/** @deprecated, use `import { generateNonce } from '@haneullabs/haneul/zklogin';` instead */
	generateNonce,
	/** @deprecated, use `import { generateRandomness } from '@haneullabs/haneul/zklogin';` instead */
	generateRandomness,
	/** @deprecated, use `import { getExtendedEphemeralPublicKey } from '@haneullabs/haneul/zklogin';` instead */
	getExtendedEphemeralPublicKey,
	/** @deprecated, use `import { getZkLoginSignature } from '@haneullabs/haneul/zklogin';` instead */
	getZkLoginSignature,
	/** @deprecated, use `import { hashASCIIStrToField } from '@haneullabs/haneul/zklogin';` instead */
	hashASCIIStrToField,
	/** @deprecated, use `import { poseidonHash } from '@haneullabs/haneul/zklogin';` instead */
	poseidonHash,
} from '@haneullabs/haneul/zklogin';

/** @deprecated, use `import { parseZkLoginSignature } from '@haneullabs/haneul/zklogin';` instead */
export function computeZkLoginAddress(options: ComputeZkLoginAddressOptions) {
	return haneulComputeZkLoginAddress({
		...options,
		legacyAddress: true,
	});
}

/** @deprecated, use `import { jwtToAddress } from '@haneullabs/haneul/zklogin';` instead */
export function jwtToAddress(jwt: string, userSalt: string | bigint, legacyAddress = true) {
	return haneulJwtToAddress(jwt, userSalt, legacyAddress);
}
