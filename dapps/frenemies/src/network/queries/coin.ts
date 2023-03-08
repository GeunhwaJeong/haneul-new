// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { HANEUL_TYPE_ARG } from "@haneullabs/haneul.js";
import { useWalletKit } from "@haneullabs/wallet-kit";
import { useQuery } from "@tanstack/react-query";
import provider from "../provider";

const DEC = 9;

export function useBalance() {
  const { currentAccount } = useWalletKit();
  return useQuery(
    ["account-balance", currentAccount?.address],
    async () => {
      const { totalBalance } = await provider.getBalance({
        owner: currentAccount?.address!,
        coinType: HANEUL_TYPE_ARG,
      });

      return {
        balance: BigInt(totalBalance),
        decimals: DEC,
      };
    },
    {
      enabled: !!currentAccount?.address,
      refetchInterval: 60 * 1000,
      staleTime: 2000,
    }
  );
}
