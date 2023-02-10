// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ReactNode } from "react";
import { ObjectData } from "../../network/rawObject";
import {
  DELEGATION,
  Delegation,
  StakedHaneul,
  STAKED_HANEUL,
} from "../../network/types";
import { useMyType } from "../../network/queries/use-raw";
import { GridItem } from "./GridItem";
import { ValidatorItem } from "./Validator";
import { normalizeHaneulAddress } from "@haneullabs/haneul.js";
import { useValidators } from "../../network/queries/haneul-system";

function Header({ children }: { children: ReactNode }) {
  return (
    <div className="text-left font-normal uppercase text-base text-steel-dark">
      {children}
    </div>
  );
}

export function Table() {
  const { data: stakes } = useMyType<StakedHaneul>(STAKED_HANEUL);
  const { data: delegations } = useMyType<Delegation>(DELEGATION);

  const { data: validators } = useValidators();

  // sort validators by their live stake info in DESC order
  const sorted = [...(validators || [])].sort((a, b) =>
    Number(
      BigInt(b.next_epoch_stake) +
        BigInt(b.next_epoch_delegation) -
        (BigInt(a.next_epoch_stake) + BigInt(a.next_epoch_delegation))
    )
  );

  const stakeByValidator: Record<string, ObjectData<StakedHaneul>> = (
    stakes || []
  ).reduce(
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
        <Header>Your HANEUL Stake</Header>
      </GridItem>

      <div className="flex flex-col gap-1">
        {sorted.map((validator, index) => {
          const address = normalizeHaneulAddress(validator.haneul_address);

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
