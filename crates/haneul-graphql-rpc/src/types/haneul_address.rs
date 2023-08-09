// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::*;
use serde::{Deserialize, Serialize};

const HANEUL_ADDRESS_LENGTH: usize = 32;

#[derive(Serialize, Deserialize)]
struct HaneulAddress([u8; HANEUL_ADDRESS_LENGTH]);

scalar!(HaneulAddress, "HaneulAddress", "Representation of Haneul Addresses");
