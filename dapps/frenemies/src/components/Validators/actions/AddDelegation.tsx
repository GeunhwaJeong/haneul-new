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
import { getGas } from "../../../utils/coins";
import { StakeButton } from "../../StakeButton";

interface Props {
  validator: HaneulAddress;
  /** Amount to Delegate */
  amount: string;
}

const HANEUL_DECIMALS = 9;
const GAS_BUDGET = 10000n;

function toMist(haneul: string) {
  return BigInt(new BigNumber(haneul).shiftedBy(HANEUL_DECIMALS).toString());
}

/**
 * Requests Delegation object for a Validator.
 * Can only be performed if there's no `StakedHaneul` (hence no `Delegation`) object.
 */
export function AddDelegation({ validator, amount }: Props) {
  const { currentAccount, signAndExecuteTransaction } = useWalletKit();
  const { data: coins } = useMyType<Coin>(HANEUL_COIN, currentAccount);

  const stakeFor = useMutation(["stake-for-validator"], async () => {
    if (!coins || !coins.length) {
      return null;
    }

    const geunhwaAmount = toMist(amount);

    const gasPrice = await provider.getReferenceGasPrice();
    const gasRequred = GAS_BUDGET * BigInt(gasPrice);
    const { gas, coins: available, max } = getGas(coins, gasRequred);

    if (geunhwaAmount > max) {
      console.log("Requested amt %d is bigger than max %d", geunhwaAmount, max);
      return null;
    }

    if (!gas) {
      return null;
    }

    await signAndExecuteTransaction({
      kind: "moveCall",
      data: {
        packageObjectId: HANEUL_FRAMEWORK_ADDRESS,
        module: "haneul_system",
        function: "request_add_delegation_mul_coin",
        gasPayment: normalizeHaneulAddress(gas.reference.objectId),
        typeArguments: [],
        gasBudget: 10000,
        arguments: [
          HANEUL_SYSTEM_ID,
          available.map((c) => normalizeHaneulAddress(c.reference.objectId)),
          [geunhwaAmount.toString()], // Option<u64> // [amt] = Some(amt)
          normalizeHaneulAddress(validator),
        ],
      },
    });
  });

  return (
    <StakeButton
      // we can only stake if there's at least 2 coins (one gas and one stake)
      disabled={!amount || !coins?.length || coins.length < 2}
      onClick={() => stakeFor.mutate()}
    >
      Stake
    </StakeButton>
  );
}
