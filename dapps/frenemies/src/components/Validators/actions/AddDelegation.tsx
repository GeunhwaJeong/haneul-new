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
import { useMyType } from "../../../network/queries/use-raw";
import { Coin, HANEUL_COIN } from "../../../network/types";
import { getGas, useManageCoin } from "../../../utils/coins";
import { StakeButton } from "../../StakeButton";

interface Props {
  validator: HaneulAddress;
  /** Amount to Delegate */
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
  const { currentAccount, signAndExecuteTransaction } = useWalletKit();
  const { data: coins } = useMyType<Coin>(HANEUL_COIN, currentAccount);

  const stake = useMutation(["stake-for-validator"], async () => {
    if (!coins || !coins.length) {
      throw new Error("No coins found.");
    }

    const geunhwaAmount = toMist(amount);

    const gasPrice = await provider.getReferenceGasPrice();
    const gasRequired = GAS_BUDGET * BigInt(gasPrice);
    const { max } = getGas(coins, gasRequired);

    if (geunhwaAmount > max) {
      throw new Error(
        `Requested amount ${geunhwaAmount} is bigger than max ${max}`
      );
    }

    const stakeCoin = await manageCoins(geunhwaAmount, gasRequired);

    await signAndExecuteTransaction(
      {
        kind: "moveCall",
        data: {
          packageObjectId: HANEUL_FRAMEWORK_ADDRESS,
          module: "haneul_system",
          function: "request_add_delegation_mul_coin",
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
      {
        requestType: "WaitForEffectsCert",
      }
    );
  });

  return (
    <StakeButton
      disabled={!amount || !coins?.length || stake.isLoading}
      onClick={() => stake.mutate()}
    >
      Stake
    </StakeButton>
  );
}
