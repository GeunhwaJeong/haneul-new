// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { defineConfig } from 'vitest/config';

export default defineConfig({
    resolve: {
        alias: {
            '@haneullabs/haneul.js': new URL(
                '../sdk/typescript/src',
                import.meta.url
            ).toString(),

            '@haneullabs/bcs': new URL(
                '../sdk/typescript/bcs/src',
                import.meta.url
            ).toString(),
        },
    },
});
