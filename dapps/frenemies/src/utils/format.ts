// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HaneulAddress } from "@haneullabs/haneul.js";

// Contains a set of hadful utilities when displaying different types
export function formatAddress(addr: HaneulAddress): string {
    return addr.slice(0, 6) + '...' + addr.slice(-4);
}
