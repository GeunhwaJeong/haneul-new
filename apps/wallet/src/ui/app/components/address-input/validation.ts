// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isValidHaneulAddress } from '@haneullabs/haneul.js';
import * as Yup from 'yup';

export const HANEUL_ADDRESS_VALIDATION = Yup.string()
    .ensure()
    .trim()
    .required()
    .transform((value: string) =>
        value.startsWith('0x') || value === '' || value === '0'
            ? value
            : `0x${value}`
    )
    .test(
        'is-haneul-address',
        // eslint-disable-next-line no-template-curly-in-string
        'Invalid address. Please check again.',
        (value) => isValidHaneulAddress(value)
    )
    .label("Recipient's address");
