// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { defineConfig } from 'vitest/config';

export default defineConfig({
	resolve: {
		alias: {
			'@haneullabs/bcs': new URL('../bcs/src', import.meta.url).toString(),
			'@haneullabs/haneul.js': new URL('../typescript/src', import.meta.url).toString(),
		},
	},
});
