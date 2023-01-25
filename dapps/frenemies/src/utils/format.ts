// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import BigNumber from "bignumber.js";
import { HaneulAddress } from "@haneullabs/haneul.js";
import { Goal } from "../network/types";

/** Formats address as `0xXXXX...YYYY` */
export function formatAddress(addr: HaneulAddress): string {
  if (addr.startsWith("0x")) {
    addr = addr.slice(2);
  }
  return "0x" + addr.slice(0, 4) + "..." + addr.slice(-4);
}

const GOAL_TO_COPY = {
  [Goal.Enemy]: "Enemy",
  [Goal.Friend]: "Friend",
  [Goal.Neutral]: "Neutral",
};

export function formatGoal(goal: Goal): string {
  return GOAL_TO_COPY[goal];
}

/** Pretty-print balance of the currency based on the decimals */
export function formatBalance(
  balance: bigint | string,
  decimals: number
): string {
  return new BigNumber(balance.toString()).shiftedBy(-1 * decimals).toFormat();
}
