// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ReactNode } from "react";
import { ObjectData } from "../../network/rawObject";
import { DELEGATION, Delegation, StakedHaneul, STAKED_HANEUL } from "../../network/types";
import { useWalletKit } from "@haneullabs/wallet-kit";
import { useMyType } from "../../network/queries/use-raw";
import { GridItem } from "./GridItem";
import { ValidatorItem } from "./Validator";
import { MoveActiveValidator, normalizeHaneulAddress } from "@haneullabs/haneul.js";

function Header({ children }: { children: ReactNode }) {
  return (
    <div className="text-left font-normal uppercase text-base text-steel-dark">
      {children}
    </div>
  );
}

interface Props {
  /** Set of 40 currently active validators */
  validators: MoveActiveValidator[];
}

export function Table({ validators }: Props) {
  const { currentAccount } = useWalletKit();
  const { data: stakes } = useMyType<StakedHaneul>(STAKED_HANEUL, currentAccount);
  const { data: delegations } = useMyType<Delegation>(
    DELEGATION,
    currentAccount
  );

  // sort validators by their voting power in DESC order (not by stake - these are different)
  // TODO: using `OR "0"` since voting power is an optional field;
  const sorted = [...validators].sort((a, b) =>
    Number(BigInt(b.fields.voting_power || "0") - BigInt(a.fields.voting_power || "0"))
  );

  const stakeByValidator: Record<string, ObjectData<StakedHaneul>> = (stakes || []).reduce(
    (acc, stake) =>
      Object.assign(acc, {
        [normalizeHaneulAddress(stake.data.validatorAddress)]: stake,
      }),
    {}
  );

  function getDelegation(address: string) {
    const stake = stakeByValidator[address];
    return (
      stake &&
      (delegations || []).find((d) => d.data.stakedHaneulId == stake.data.id)
    );
  }

  return (
    <>
      <GridItem className="px-5 py-4">
        <Header>Rank</Header>
        <Header>Validator</Header>
        <Header>Your Haneul Stake</Header>
      </GridItem>

      <div className="flex flex-col gap-1">
        {sorted.map((validator, index) => {
          const address = normalizeHaneulAddress(
            validator.fields.metadata.fields.haneul_address
          );

          return (
            <ValidatorItem
              key={address}
              index={index}
              validator={validator}
              stake={stakeByValidator[address]}
              delegation={getDelegation(address)}
            />
          );
        })}
      </div>
    </>
  );
}
