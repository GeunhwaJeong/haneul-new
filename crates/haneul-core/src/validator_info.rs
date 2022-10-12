// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_config::ValidatorInfo;
use haneul_types::{
    committee::{Committee, EpochId},
    error::HaneulResult,
};

pub fn make_committee(epoch: EpochId, validator_set: &[ValidatorInfo]) -> HaneulResult<Committee> {
    Committee::new(epoch, ValidatorInfo::voting_rights(validator_set))
}
