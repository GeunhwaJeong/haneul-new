// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::base_types::ObjectID;
use crate::committee::CommitteeWithNetworkMetadata;
use crate::dynamic_field::get_dynamic_field_from_store;
use crate::error::HaneulError;
use crate::storage::ObjectStore;
use crate::haneul_system_state::epoch_start_haneul_system_state::EpochStartSystemState;
use crate::versioned::Versioned;
use crate::{id::UID, MoveTypeTagTrait, HANEUL_SYSTEM_ADDRESS, HANEUL_SYSTEM_STATE_OBJECT_ID};
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt;

use self::haneul_system_state_inner_v1::{HaneulSystemStateInnerV1, ValidatorV1};
use self::haneul_system_state_summary::{HaneulSystemStateSummary, HaneulValidatorSummary};

pub mod epoch_start_haneul_system_state;
pub mod haneul_system_state_inner_v1;
pub mod haneul_system_state_summary;

const HANEUL_SYSTEM_STATE_WRAPPER_STRUCT_NAME: &IdentStr = ident_str!("HaneulSystemState");

pub const HANEUL_SYSTEM_MODULE_NAME: &IdentStr = ident_str!("haneul_system");
pub const ADVANCE_EPOCH_FUNCTION_NAME: &IdentStr = ident_str!("advance_epoch");
pub const ADVANCE_EPOCH_SAFE_MODE_FUNCTION_NAME: &IdentStr = ident_str!("advance_epoch_safe_mode");

/// Rust version of the Move haneul::haneul_system::HaneulSystemState type
/// This repreents the object with 0x5 ID.
/// In Rust, this type should be rarely used since it's just a thin
/// wrapper used to access the inner object.
/// Within this module, we use it to determine the current version of the system state inner object type,
/// so that we could deserialize the inner object correctly.
/// Outside of this module, we only use it in genesis snapshot and testing.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HaneulSystemStateWrapper {
    pub id: UID,
    pub version: u64,
}

impl HaneulSystemStateWrapper {
    pub fn type_() -> StructTag {
        StructTag {
            address: HANEUL_SYSTEM_ADDRESS,
            name: HANEUL_SYSTEM_STATE_WRAPPER_STRUCT_NAME.to_owned(),
            module: HANEUL_SYSTEM_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}

/// This is the standard API that all inner system state object type should implement.
#[enum_dispatch]
pub trait HaneulSystemStateTrait {
    fn epoch(&self) -> u64;
    fn reference_gas_price(&self) -> u64;
    fn protocol_version(&self) -> u64;
    fn system_state_version(&self) -> u64;
    fn epoch_start_timestamp_ms(&self) -> u64;
    fn epoch_duration_ms(&self) -> u64;
    fn safe_mode(&self) -> bool;
    fn get_current_epoch_committee(&self) -> CommitteeWithNetworkMetadata;
    fn into_epoch_start_state(self) -> EpochStartSystemState;
    fn into_haneul_system_state_summary(self) -> HaneulSystemStateSummary;
}

/// HaneulSystemState provides an abstraction over multiple versions of the inner HaneulSystemStateInner object.
/// This should be the primary interface to the system state object in Rust.
/// We use enum dispatch to dispatch all methods defined in HaneulSystemStateTrait to the actual
/// implementation in the inner types.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[enum_dispatch(HaneulSystemStateTrait)]
pub enum HaneulSystemState {
    V1(HaneulSystemStateInnerV1),
}

/// This is the fixed type used by genesis.
pub type HaneulSystemStateInnerGenesis = HaneulSystemStateInnerV1;
pub type HaneulValidatorGenesis = ValidatorV1;

impl HaneulSystemState {
    /// Always return the version that we will be using for genesis.
    /// Genesis always uses this version regardless of the current version.
    /// Note that since it's possible for the actual genesis of the network to diverge from the
    /// genesis of the latest Rust code, it's important that we only use this for tooling purposes.
    pub fn into_genesis_version_for_tooling(self) -> HaneulSystemStateInnerGenesis {
        match self {
            HaneulSystemState::V1(inner) => inner,
        }
    }

    pub fn version(&self) -> u64 {
        self.system_state_version()
    }
}

pub fn get_haneul_system_state_wrapper<S>(object_store: &S) -> Result<HaneulSystemStateWrapper, HaneulError>
where
    S: ObjectStore,
{
    let wrapper = object_store
        .get_object(&HANEUL_SYSTEM_STATE_OBJECT_ID)?
        // Don't panic here on None because object_store is a generic store.
        .ok_or_else(|| {
            HaneulError::HaneulSystemStateReadError("HaneulSystemStateWrapper object not found".to_owned())
        })?;
    let move_object = wrapper.data.try_as_move().ok_or_else(|| {
        HaneulError::HaneulSystemStateReadError(
            "HaneulSystemStateWrapper object must be a Move object".to_owned(),
        )
    })?;
    let result = bcs::from_bytes::<HaneulSystemStateWrapper>(move_object.contents())
        .map_err(|err| HaneulError::HaneulSystemStateReadError(err.to_string()))?;
    Ok(result)
}

pub fn get_haneul_system_state<S>(object_store: &S) -> Result<HaneulSystemState, HaneulError>
where
    S: ObjectStore,
{
    let wrapper = get_haneul_system_state_wrapper(object_store)?;
    match wrapper.version {
        1 => {
            let id = wrapper.id.id.bytes;
            let result: HaneulSystemStateInnerV1 =
                get_dynamic_field_from_store(object_store, id, &wrapper.version).map_err(
                    |err| {
                        HaneulError::DynamicFieldReadError(format!(
                            "Failed to load haneul system state inner object with ID {:?} and version {:?}: {:?}",
                            id, wrapper.version, err
                        ))
                    },
                )?;
            Ok(HaneulSystemState::V1(result))
        }
        _ => Err(HaneulError::HaneulSystemStateReadError(format!(
            "Unsupported HaneulSystemState version: {}",
            wrapper.version
        ))),
    }
}

/// Given a system state type version, and the ID of the table, along with a key, retrieve the
/// dynamic field as a Validator type. We need the version to determine which inner type to use for
/// the Validator type. This is assuming that the validator is stored in the table as
/// ValidatorWrapper type.
pub fn get_validator_from_table<S, K>(
    object_store: &S,
    table_id: ObjectID,
    key: &K,
) -> Result<HaneulValidatorSummary, HaneulError>
where
    S: ObjectStore,
    K: MoveTypeTagTrait + Serialize + DeserializeOwned + fmt::Debug,
{
    let field: ValidatorWrapper = get_dynamic_field_from_store(object_store, table_id, key)
        .map_err(|err| {
            HaneulError::HaneulSystemStateReadError(format!(
                "Failed to load validator wrapper from table: {:?}",
                err
            ))
        })?;
    let versioned = field.inner;
    let version = versioned.version;
    match version {
        1 => {
            let validator: ValidatorV1 =
                get_dynamic_field_from_store(object_store, versioned.id.id.bytes, &version)
                    .map_err(|err| {
                        HaneulError::HaneulSystemStateReadError(format!(
                            "Failed to load inner validator from the wrapper: {:?}",
                            err
                        ))
                    })?;
            Ok(validator.into_haneul_validator_summary())
        }
        _ => Err(HaneulError::HaneulSystemStateReadError(format!(
            "Unsupported Validator version: {}",
            version
        ))),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct PoolTokenExchangeRate {
    haneul_amount: u64,
    pool_token_amount: u64,
}

impl PoolTokenExchangeRate {
    /// Rate of the staking pool, pool token amount : Haneul amount
    pub fn rate(&self) -> f64 {
        if self.haneul_amount == 0 {
            1_f64
        } else {
            self.pool_token_amount as f64 / self.haneul_amount as f64
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorWrapper {
    pub inner: Versioned,
}
