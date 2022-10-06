// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    minThreads: 1,
    maxThreads: 8,
  },
  resolve: {
    alias: {
      '@haneullabs/bcs': new URL('../bcs/src', import.meta.url).toString(),
      '@haneullabs/haneul-open-rpc': new URL(
        '../../crates/haneul-open-rpc',
        import.meta.url
      ).toString(),
    },
  },
});
