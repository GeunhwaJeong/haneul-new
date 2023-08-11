// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::*;
use serde::{Deserialize, Serialize};

const HANEUL_ADDRESS_LENGTH: usize = 32;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct HaneulAddress([u8; HANEUL_ADDRESS_LENGTH]);

#[Scalar]
impl ScalarType for HaneulAddress {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(mut s) => {
                if s.starts_with("0x") {
                    s = s[2..].to_string();
                } else {
                    return Err(InputValueError::custom(
                        "Invalid HaneulAddress. Missing 0x prefix",
                    ));
                }

                let bytes = hex::decode(s)?;
                if bytes.len() != HANEUL_ADDRESS_LENGTH {
                    return Err(InputValueError::custom(format!(
                        "Invalid HaneulAddress length: {}",
                        bytes.len()
                    )));
                }
                let mut arr = [0u8; HANEUL_ADDRESS_LENGTH];
                arr.copy_from_slice(&bytes);
                Ok(HaneulAddress(arr))
            }
            _ => Err(InputValueError::custom("Invalid HaneulAddress")),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(hex::encode(self.0))
    }
}

impl HaneulAddress {
    pub fn to_array(&self) -> [u8; HANEUL_ADDRESS_LENGTH] {
        self.0
    }

    pub fn from_array(arr: [u8; HANEUL_ADDRESS_LENGTH]) -> Self {
        HaneulAddress(arr)
    }
}
