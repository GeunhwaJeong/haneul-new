// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use haneul_types::base_types::{AuthorityName, EpochId, ObjectID, HaneulAddress};
use haneul_types::committee::{Committee, StakeUnit};

/// RPC representation of the [Committee] type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "CommitteeInfo")]
pub struct HaneulCommittee {
    pub epoch: EpochId,
    pub validators: Vec<(AuthorityName, StakeUnit)>,
}

impl From<Committee> for HaneulCommittee {
    fn from(committee: Committee) -> Self {
        Self {
            epoch: committee.epoch,
            validators: committee.voting_rights,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DelegatedStake {
    /// Validator's Address.
    pub validator_address: HaneulAddress,
    /// Staking pool object id.
    pub staking_pool: ObjectID,
    pub stakes: Vec<Stake>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(tag = "status")]
pub enum StakeStatus {
    Pending,
    #[serde(rename_all = "camelCase")]
    Active {
        estimated_reward: u64,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Stake {
    /// ID of the StakedHaneul receipt object.
    pub staked_haneul_id: ObjectID,
    pub stake_request_epoch: EpochId,
    pub stake_active_epoch: EpochId,
    pub principal: u64,
    #[serde(flatten)]
    pub status: StakeStatus,
}
