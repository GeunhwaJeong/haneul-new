// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { normalizeHaneulAddress } from "@haneullabs/haneul.js";
import { useRawObject } from "./use-raw";
import { HaneulSystem } from "../types";

/**
 * Address of the Haneul System object.
 * Always the same in every Haneul network (local, devnet, testnet).
 */
export const HANEUL_SYSTEM: string = normalizeHaneulAddress("0x5");

/**
 * Read the HaneulSystem object.
 */
export function useHaneulSystem() {
  return useRawObject<HaneulSystem>(HANEUL_SYSTEM, "haneul_system::HaneulSystemState");
}
