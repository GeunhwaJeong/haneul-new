// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { normalizeHaneulAddress } from "@haneullabs/haneul.js";
import { useQuery } from "@tanstack/react-query";
import provider from "../provider";

/**
 * Address of the Haneul System object.
 * Always the same in every Haneul network (local, devnet, testnet).
 */
export const HANEUL_SYSTEM_ID: string = normalizeHaneulAddress("0x5");

export function useValidators() {
  return useQuery(
    ["validators"],
    async () => {
      return provider.getValidators();
    },
    {
      refetchInterval: 60 * 1000,
      staleTime: 5000,
    }
  );
}
