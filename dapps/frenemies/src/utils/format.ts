// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/**
 * Contains data formatting functions
 * @module utils/format
 */

import { HaneulAddress } from "@haneullabs/haneul.js";
import { Goal } from "../network/types";

/** Formats address as `0xXXXXX...YYYY` */
export function formatAddress(addr: HaneulAddress): string {
    return '0x' + addr.slice(0, 4) + '...' + addr.slice(-4);
}

/**  Pretty pring `Goal` enum; turns values into human-readable strings */
export function formatGoal(goal: Goal): string {
    switch (goal) {
        case Goal.Enemy: return 'Enemy';
        case Goal.Friend: return 'Friend';
        case Goal.Neutral: return 'Neutral';
    }
}
