// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useWalletKit } from "@haneullabs/wallet-kit";
import { useScorecard } from "../../network/queries/scorecard";
import { useHaneulSystem } from "../../network/queries/haneul-system";
import { formatGoal } from "../../utils/format";
import { Goal } from "../../network/types";
import { Card } from "../Card";
import { Balance } from "./Balance";
import { Table } from "./Table";

export function Validators() {
  const { currentAccount } = useWalletKit();
  const { data: system } = useHaneulSystem();
  const { data: scorecard } = useScorecard(currentAccount);

  // At this point there's no way it errors out.
  if (!system || !scorecard || !currentAccount) {
    return null;
  }

  const validators = system.validators.fields.active_validators;
  const assignment = scorecard.data.assignment;

  return (
    <Card variant="white" spacing="lg">
      <div className="flex items-center justify-between mb-10">
        <h2 className="text-steel-dark font-normal text-2xl">
          Stake HANEUL to achieve your goal as{" "}
          {assignment.goal === Goal.Enemy ? "an " : "a "}
          <span className="font-bold">{formatGoal(assignment.goal)}</span>.
        </h2>

        <Balance />
      </div>
      <Table validators={validators} />
    </Card>
  );
}
