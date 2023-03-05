// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use haneul_types::base_types::{
    EpochId, ObjectDigest, ObjectID, ObjectRef, SequenceNumber, TransactionDigest,
};
use haneul_types::coin::CoinMetadata;

use haneul_types::error::HaneulError;
use haneul_types::object::Object;

use crate::Page;

pub type CoinPage = Page<Coin, ObjectID>;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub coin_type: String,
    pub coin_object_count: usize,
    pub total_balance: u128,
    pub locked_balance: HashMap<EpochId, u128>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub coin_type: String,
    pub coin_object_id: ObjectID,
    pub version: SequenceNumber,
    pub digest: ObjectDigest,
    pub balance: u64,
    pub locked_until_epoch: Option<EpochId>,
    pub previous_transaction: TransactionDigest,
}

impl Coin {
    pub fn object_ref(&self) -> ObjectRef {
        (self.coin_object_id, self.version, self.digest)
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HaneulCoinMetadata {
    /// Number of decimal places the coin uses.
    pub decimals: u8,
    /// Name for the token
    pub name: String,
    /// Symbol for the token
    pub symbol: String,
    /// Description of the token
    pub description: String,
    /// URL for the token logo
    pub icon_url: Option<String>,
    /// Object id for the CoinMetadata object
    pub id: Option<ObjectID>,
}

impl TryFrom<Object> for HaneulCoinMetadata {
    type Error = HaneulError;
    fn try_from(object: Object) -> Result<Self, Self::Error> {
        let metadata: CoinMetadata = object.try_into()?;
        let CoinMetadata {
            decimals,
            name,
            symbol,
            description,
            icon_url,
            id,
        } = metadata;
        Ok(Self {
            id: Some(*id.object_id()),
            decimals,
            name,
            symbol,
            description,
            icon_url,
        })
    }
}
