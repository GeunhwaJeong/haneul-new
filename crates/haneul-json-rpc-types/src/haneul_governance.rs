// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use haneul_types::base_types::{AuthorityName, EpochId};
use haneul_types::committee::{Committee, StakeUnit};
use haneul_types::haneul_system_state::haneul_system_state_inner_v1::HaneulSystemStateInnerV1;
use haneul_types::haneul_system_state::HaneulSystemState;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema)]
#[serde(untagged, rename = "HaneulSystemState")]
pub enum HaneulSystemStateRpc {
    V1(HaneulSystemStateInnerV1),
}

impl From<HaneulSystemState> for HaneulSystemStateRpc {
    fn from(state: HaneulSystemState) -> Self {
        match state {
            HaneulSystemState::V1(state) => Self::V1(state),
        }
    }
}

impl From<HaneulSystemStateRpc> for HaneulSystemState {
    fn from(state: HaneulSystemStateRpc) -> Self {
        match state {
            HaneulSystemStateRpc::V1(state) => Self::V1(state),
        }
    }
}

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
