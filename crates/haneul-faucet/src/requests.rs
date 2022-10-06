// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use haneul_types::base_types::HaneulAddress;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FaucetRequest {
    FixedAmountRequest(FixedAmountRequest),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FixedAmountRequest {
    pub recipient: HaneulAddress,
}

impl FaucetRequest {
    pub fn new_fixed_amount_request(recipient: impl Into<HaneulAddress>) -> Self {
        Self::FixedAmountRequest(FixedAmountRequest {
            recipient: recipient.into(),
        })
    }
}
