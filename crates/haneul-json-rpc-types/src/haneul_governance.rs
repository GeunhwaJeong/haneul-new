// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use haneul_types::base_types::{AuthorityName, EpochId};
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
