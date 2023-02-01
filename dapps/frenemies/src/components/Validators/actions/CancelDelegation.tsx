// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { normalizeHaneulAddress, HANEUL_FRAMEWORK_ADDRESS } from "@haneullabs/haneul.js";
import { useWalletKit } from "@haneullabs/wallet-kit";
import { useMutation } from "@tanstack/react-query";
import provider from "../../../network/provider";
import { HANEUL_SYSTEM_ID } from "../../../network/queries/haneul-system";
import { useMyType } from "../../../network/queries/use-raw";
import { ObjectData } from "../../../network/rawObject";
import { Coin, StakedHaneul, HANEUL_COIN } from "../../../network/types";
import { getGas } from "../../../utils/coins";
import { StakeButton } from "../../StakeButton";

interface Props {
  stake: ObjectData<StakedHaneul>;
}

const GAS_BUDGET = 100000n;

/**
 * Request delegation withdrawal.
 * Can only be called if the Delegation and StakedHaneul objects are present.
 */
export function CancelDelegation({ stake }: Props) {
  const { currentAccount, signAndExecuteTransaction } = useWalletKit();
  const { data: coins } = useMyType<Coin>(HANEUL_COIN, currentAccount);

  const withdrawDelegation = useMutation(["unstake-validator"], async () => {
    if (!coins || !coins.length) {
      return null;
    }

    const gasPrice = await provider.getReferenceGasPrice();
    const gasRequred = GAS_BUDGET * BigInt(gasPrice);
    const { gas } = getGas(coins, gasRequred);

    if (!gas) {
      return null;
    }

    await signAndExecuteTransaction(
      {
        kind: "moveCall",
        data: {
          packageObjectId: HANEUL_FRAMEWORK_ADDRESS,
          module: "haneul_system",
          function: "cancel_delegation_request",
          gasBudget: Number(GAS_BUDGET),
          typeArguments: [],
          gasPayment: normalizeHaneulAddress(gas.reference.objectId),
          arguments: [
            HANEUL_SYSTEM_ID,
            normalizeHaneulAddress(stake.reference.objectId),
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
      disabled={!coins?.length}
      onClick={() => withdrawDelegation.mutate()}
    >
      Unstake
    </StakeButton>
  );
}
