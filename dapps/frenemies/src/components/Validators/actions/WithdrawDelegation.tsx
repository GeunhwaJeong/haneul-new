// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { normalizeHaneulAddress, HANEUL_FRAMEWORK_ADDRESS } from "@haneullabs/haneul.js";
import { useWalletKit } from "@haneullabs/wallet-kit";
import { useMutation } from "@tanstack/react-query";
import { HANEUL_SYSTEM_ID } from "../../../network/queries/haneul-system";
import { useMyType } from "../../../network/queries/use-raw";
import { ObjectData } from "../../../network/rawObject";
import { Coin, Delegation, StakedHaneul, HANEUL_COIN } from "../../../network/types";
import { getGas } from "../../../utils/coins";
import provider from "../../../network/provider";

interface Props {
  stake: ObjectData<StakedHaneul>;
  delegation: ObjectData<Delegation>;
}

/**
 * Arguments required for WithdrawDelegation transaction.
 */
interface WithdrawDelegationTx {
  /** Current stake for the Validator */
  stake: ObjectData<StakedHaneul>;
  /** Delegation object which matches the `StakedHaneul` */
  delegation: ObjectData<Delegation>;
  /** Coins to get Gas from */
  coins: ObjectData<Coin>[] | null | undefined;
}

const GAS_BUDGET = 10000n;

/**
 * Request delegation withdrawal.
 * Can only be called if the Delegation and StakedHaneul objects are present.
 */
export function WithdrawDelegation({ stake, delegation }: Props) {
  const { currentAccount, signAndExecuteTransaction } = useWalletKit();
  const { data: coins } = useMyType<Coin>(HANEUL_COIN, currentAccount);

  const withdrawDelegation = useMutation(
    ["unstake-validator"],
    async ({ stake, delegation, coins }: WithdrawDelegationTx) => {
      if (!coins || !coins.length) {
        return null;
      }

      const gasPrice = await provider.getReferenceGasPrice();
      const gasRequred = GAS_BUDGET * BigInt(gasPrice);
      const { gas } = getGas(coins, gasRequred);

      if (!gas) {
        return null;
      }

      await signAndExecuteTransaction({
        kind: "moveCall",
        data: {
          packageObjectId: HANEUL_FRAMEWORK_ADDRESS,
          module: "haneul_system",
          function: "request_withdraw_delegation",
          gasBudget: 10000,
          typeArguments: [],
          gasPayment: normalizeHaneulAddress(gas.reference.objectId),
          arguments: [
            HANEUL_SYSTEM_ID,
            normalizeHaneulAddress(delegation.reference.objectId),
            normalizeHaneulAddress(stake.reference.objectId),
          ],
        },
      });
    }
  );

  const clickHandler = () =>
    withdrawDelegation.mutate({ stake, delegation, coins });

  return (
    <button
      disabled={!coins?.length}
      onClick={clickHandler}
      className="absolute right-0 flex py-1 px-4 text-sm leading-none bg-gradient-to-b from-[#D0E8EF] to-[#B9DAE4] opacity-60 hover:opacity-100 uppercase mr-2 rounded-[4px]"
    >
      Unstake
    </button>
  );
}
