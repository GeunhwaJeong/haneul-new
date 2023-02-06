// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  is,
  normalizeHaneulAddress,
  HaneulObject,
  MoveHaneulSystemObjectFields,
} from "@haneullabs/haneul.js";
import { useQuery } from "@tanstack/react-query";
import provider from "../provider";

/**
 * Address of the Haneul System object.
 * Always the same in every Haneul network (local, devnet, testnet).
 */
export const HANEUL_SYSTEM_ID: string = normalizeHaneulAddress("0x5");

/**
 * Read the HaneulSystem object.
 */
export function useHaneulSystem() {
  return useQuery(
    ["object", HANEUL_SYSTEM_ID],
    async () => {
      const data = await provider.getObject(HANEUL_SYSTEM_ID);
      const systemObject =
        data &&
        is(data.details, HaneulObject) &&
        data.details.data.dataType === "moveObject"
          ? (data.details.data.fields as MoveHaneulSystemObjectFields)
          : null;

      return systemObject;
    },
    {
      refetchInterval: 60 * 1000,
      refetchOnWindowFocus: false,
    }
  );

  // TODO: Fix raw version when there is delegated stake:
  // return useRawObject<HaneulSystem>(HANEUL_SYSTEM_ID, HANEUL_SYSTEM_TYPE);
}
