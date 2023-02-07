// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { normalizeHaneulAddress, HANEUL_FRAMEWORK_ADDRESS } from "@haneullabs/haneul.js";
import { useWalletKit } from "@haneullabs/wallet-kit";
import { useMutation } from "@tanstack/react-query";
import { HANEUL_SYSTEM_ID } from "../../../network/queries/haneul-system";
import { ObjectData } from "../../../network/rawObject";
import { Delegation, StakedHaneul } from "../../../network/types";
import { StakeButton } from "../../StakeButton";

interface Props {
  stake: ObjectData<StakedHaneul>;
  delegation: ObjectData<Delegation>;
}

const GAS_BUDGET = 100000n;

/**
 * Request delegation withdrawal.
 * Can only be called if the Delegation and StakedHaneul objects are present.
 */
export function WithdrawDelegation({ stake, delegation }: Props) {
  const { currentAccount, signAndExecuteTransaction } = useWalletKit();

  const withdrawDelegation = useMutation(["unstake-validator"], async () => {
    await signAndExecuteTransaction(
      {
        kind: "moveCall",
        data: {
          packageObjectId: HANEUL_FRAMEWORK_ADDRESS,
          module: "haneul_system",
          function: "request_withdraw_delegation",
          gasBudget: Number(GAS_BUDGET),
          typeArguments: [],
          arguments: [
            HANEUL_SYSTEM_ID,
            normalizeHaneulAddress(delegation.reference.objectId),
            normalizeHaneulAddress(stake.reference.objectId),
          ],
        },
      },
      {
        // requestType: "WaitForEffectsCert",
      }
    );
  });

  return (
    <StakeButton onClick={() => withdrawDelegation.mutate()}>
      Unstake
    </StakeButton>
  );
}
