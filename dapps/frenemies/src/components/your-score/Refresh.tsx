// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { normalizeHaneulAddress } from "@haneullabs/haneul.js";
import { useWalletKit } from "@haneullabs/wallet-kit";
import { useMutation } from "@tanstack/react-query";
import { config } from "../../config";
import { useEpoch } from "../../network/queries/epoch";
import { HANEUL_SYSTEM_ID } from "../../network/queries/haneul-system";
import { useRawObject } from "../../network/queries/use-raw";
import { ObjectData } from "../../network/rawObject";
import { Leaderboard, LEADERBOARD, Scorecard } from "../../network/types";

const GAS_BUDGET = 100000n;

interface Props {
  scorecard: ObjectData<Scorecard>;
  leaderboardID: string;
  round: bigint;
}

export function Refresh({ scorecard, round, leaderboardID }: Props) {
  const { signAndExecuteTransaction } = useWalletKit();
  const { data: epoch } = useEpoch();
  const { data: leaderboard } = useRawObject<Leaderboard>(
    config.VITE_LEADERBOARD,
    LEADERBOARD
  );

  const refreshScorecard = useMutation(["refresh-scorecard"], async () => {
    await signAndExecuteTransaction({
      kind: "moveCall",
      data: {
        packageObjectId: config.VITE_PKG,
        module: "frenemies",
        function: "update",
        typeArguments: [],
        gasBudget: Number(GAS_BUDGET),
        arguments: [
          normalizeHaneulAddress(scorecard.reference.objectId),
          HANEUL_SYSTEM_ID,
          normalizeHaneulAddress(leaderboardID),
        ],
      },
    });
  });

  if (scorecard.data.assignment.epoch == epoch?.data.epoch || !leaderboard) {
    return null;
  }

  return (
    <div className="absolute top-0 right-0">
      <button
        className="bg-white shadow-button text-body font-semibold text-frenemies py-3 px-4 rounded-lg inline-flex items-center gap-2"
        onClick={() => {
          refreshScorecard.mutate();
        }}
      >
        <img src="/refresh.svg" alt="refresh" />
        Play Round {(round || 0).toString()}
      </button>
    </div>
  );
}
