// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type JsonRpcProvider } from '@haneullabs/haneul.js';
import * as Yup from 'yup';

import { createHaneulAddressValidation } from '_components/address-input/validation';
import { createTokenValidation } from '_src/shared/validation';

export function createValidationSchemaStepOne(
    rpc: JsonRpcProvider,
    haneulNSEnabled: boolean,
    ...args: Parameters<typeof createTokenValidation>
) {
    return Yup.object({
        to: createHaneulAddressValidation(rpc, haneulNSEnabled),
        amount: createTokenValidation(...args),
    });
}
