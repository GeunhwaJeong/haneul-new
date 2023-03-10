// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  normalizeHaneulAddress,
  HaneulAddress,
  HANEUL_FRAMEWORK_ADDRESS,
} from "@haneullabs/haneul.js";
import { useWalletKit } from "@haneullabs/wallet-kit";
import { useMutation } from "@tanstack/react-query";
import BigNumber from "bignumber.js";
import provider from "../../../network/provider";
import { HANEUL_SYSTEM_ID } from "../../../network/queries/haneul-system";
import { useGetLatestCoins, useManageCoin } from "../../../utils/coins";
import { formatBalance } from "../../../utils/format";
import { StakeButton } from "../../StakeButton";

interface Props {
  validator: HaneulAddress;
  /** Amount to Stake */
  amount: string;
}

const HANEUL_DECIMALS = 9;
const GAS_BUDGET = 100000n;

function toMist(haneul: string) {
  return BigInt(new BigNumber(haneul).shiftedBy(HANEUL_DECIMALS).toString());
}

/**
 * Requests Delegation object for a Validator.
 * Can only be performed if there's no `StakedHaneul` (hence no `Delegation`) object.
 */
export function AddDelegation({ validator, amount }: Props) {
  const manageCoins = useManageCoin();
  const { signAndExecuteTransaction } = useWalletKit();
  const getLatestCoins = useGetLatestCoins();

  const stake = useMutation(["stake-for-validator"], async () => {
    const coins = await getLatestCoins();

    if (!coins || !coins.length) {
      throw new Error("No coins found.");
    }

    const totalBalance = coins.reduce(
      (acc, coin) => (acc += BigInt(coin.balance)),
      0n
    );

    const geunhwaAmount = toMist(amount);

    const gasPrice = await provider.getReferenceGasPrice();
    const gasRequired = GAS_BUDGET * BigInt(gasPrice);

    if (geunhwaAmount > totalBalance) {
      throw new Error(
        `Requested amount ${formatBalance(
          geunhwaAmount,
          HANEUL_DECIMALS
        )} is bigger than max ${formatBalance(totalBalance, HANEUL_DECIMALS)}`
      );
    }

    const stakeCoin = await manageCoins(coins, geunhwaAmount, gasRequired);

    await signAndExecuteTransaction({
      transaction: {
        kind: "moveCall",
        data: {
          packageObjectId: HANEUL_FRAMEWORK_ADDRESS,
          module: "haneul_system",
          function: "request_add_stake_mul_coin",
          typeArguments: [],
          gasBudget: Number(GAS_BUDGET),
          arguments: [
            HANEUL_SYSTEM_ID,
            [stakeCoin],
            [geunhwaAmount.toString()], // Option<u64> // [amt] = Some(amt)
            normalizeHaneulAddress(validator),
          ],
        },
      },
      // options: {
      // requestType: "WaitForEffectsCert",
      // },
    });
  });

  return (
    <StakeButton
      disabled={!amount || stake.isLoading}
      onClick={() => stake.mutate()}
    >
      Stake
    </StakeButton>
  );
}
