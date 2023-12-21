// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::*;
use move_core_types::language_storage::StructTag;
use serde::{Deserialize, Serialize};
use haneul_types::id::UID;

use super::move_object::MoveObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Domain {
    labels: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct NativeHaneulnsRegistration {
    pub id: UID,
    pub domain: Domain,
    pub domain_name: String,
    pub expiration_timestamp_ms: u64,
    pub image_url: String,
}

#[derive(Clone)]
pub(crate) struct HaneulnsRegistration {
    /// Representation of this HaneulnsRegistration as a generic Move object.
    pub super_: MoveObject,

    /// The deserialized representation of the Move object's contents.
    pub native: NativeHaneulnsRegistration,
}

pub(crate) enum HaneulnsRegistrationDowncastError {
    NotAHaneulnsRegistration,
    Bcs(bcs::Error),
}

impl HaneulnsRegistration {
    // Because the type of the HaneulnsRegistration object is not constant,
    // we need to take it in as a param.
    pub fn try_from(
        move_object: &MoveObject,
        tag: &StructTag,
    ) -> Result<Self, HaneulnsRegistrationDowncastError> {
        if !move_object.native.is_type(tag) {
            return Err(HaneulnsRegistrationDowncastError::NotAHaneulnsRegistration);
        }

        Ok(Self {
            super_: move_object.clone(),
            native: bcs::from_bytes(move_object.native.contents())
                .map_err(HaneulnsRegistrationDowncastError::Bcs)?,
        })
    }
}

#[Object]
impl HaneulnsRegistration {
    /// Domain name of the HaneulnsRegistration object
    async fn domain(&self) -> &str {
        &self.native.domain_name
    }

    /// Convert the HaneulnsRegistration object into a Move object
    async fn as_move_object(&self) -> &MoveObject {
        &self.super_
    }
}
