// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::committee::{CommitteeWithNetworkMetadata, EpochId, ProtocolVersion};
use crate::dynamic_field::{derive_dynamic_field_id, Field};
use crate::error::HaneulError;
use crate::storage::ObjectStore;
use crate::haneul_system_state::epoch_start_haneul_system_state::EpochStartSystemState;
use crate::{id::UID, HANEUL_FRAMEWORK_ADDRESS, HANEUL_SYSTEM_STATE_OBJECT_ID};
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use move_core_types::language_storage::TypeTag;
use move_core_types::value::MoveTypeLayout;
use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use move_vm_types::values::Value;
use multiaddr::Multiaddr;
use serde::{Deserialize, Serialize};
use tracing::error;

use self::haneul_system_state_inner_v1::HaneulSystemStateInnerV1;
use self::haneul_system_state_summary::HaneulSystemStateSummary;

pub mod epoch_start_haneul_system_state;
pub mod haneul_system_state_inner_v1;
pub mod haneul_system_state_summary;

const HANEUL_SYSTEM_STATE_WRAPPER_STRUCT_NAME: &IdentStr = ident_str!("HaneulSystemState");

pub const HANEUL_SYSTEM_MODULE_NAME: &IdentStr = ident_str!("haneul_system");
pub const ADVANCE_EPOCH_FUNCTION_NAME: &IdentStr = ident_str!("advance_epoch");
pub const ADVANCE_EPOCH_SAFE_MODE_FUNCTION_NAME: &IdentStr = ident_str!("advance_epoch_safe_mode");
pub const CONSENSUS_COMMIT_PROLOGUE_FUNCTION_NAME: &IdentStr =
    ident_str!("consensus_commit_prologue");

pub const INIT_SYSTEM_STATE_VERSION: u64 = 1;

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
            address: HANEUL_FRAMEWORK_ADDRESS,
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
    fn epoch_start_timestamp_ms(&self) -> u64;
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

/// This is the fixed type used by benchmarking.
pub type HaneulSystemStateInnerBenchmark = HaneulSystemStateInnerV1;

impl HaneulSystemState {
    pub fn new_genesis(inner: HaneulSystemStateInnerGenesis) -> Self {
        Self::V1(inner)
    }

    /// Always return the version that we will be using for genesis.
    /// Genesis always uses this version regardless of the current version.
    pub fn into_genesis_version(self) -> HaneulSystemStateInnerGenesis {
        match self {
            HaneulSystemState::V1(inner) => inner,
        }
    }

    pub fn into_benchmark_version(self) -> HaneulSystemStateInnerBenchmark {
        match self {
            HaneulSystemState::V1(inner) => inner,
        }
    }

    pub fn new_for_benchmarking(inner: HaneulSystemStateInnerBenchmark) -> Self {
        Self::V1(inner)
    }

    pub fn new_for_testing(epoch: EpochId) -> Self {
        HaneulSystemState::V1(HaneulSystemStateInnerV1 {
            epoch,
            ..Default::default()
        })
    }
}

impl Default for HaneulSystemState {
    fn default() -> Self {
        HaneulSystemState::V1(HaneulSystemStateInnerV1::default())
    }
}

pub fn get_haneul_system_state_wrapper<S>(object_store: &S) -> Result<HaneulSystemStateWrapper, HaneulError>
where
    S: ObjectStore,
{
    let haneul_system_object = object_store
        .get_object(&HANEUL_SYSTEM_STATE_OBJECT_ID)?
        .ok_or(HaneulError::HaneulSystemStateNotFound)?;
    let move_object = haneul_system_object
        .data
        .try_as_move()
        .ok_or(HaneulError::HaneulSystemStateNotFound)?;
    let result = bcs::from_bytes::<HaneulSystemStateWrapper>(move_object.contents())
        .expect("Haneul System State object deserialization cannot fail");
    Ok(result)
}

// This version is used to support authority_tests::test_haneul_system_state_nop_upgrade.
pub const HANEUL_SYSTEM_STATE_TESTING_VERSION1: u64 = u64::MAX;

pub fn get_haneul_system_state<S>(object_store: &S) -> Result<HaneulSystemState, HaneulError>
where
    S: ObjectStore,
{
    let wrapper = get_haneul_system_state_wrapper(object_store)?;
    let inner_id = derive_dynamic_field_id(
        wrapper.id.id.bytes,
        &TypeTag::U64,
        &MoveTypeLayout::U64,
        &Value::u64(wrapper.version),
    )
    .expect("Haneul System State object must exist");
    let inner = object_store
        .get_object(&inner_id)?
        .ok_or(HaneulError::HaneulSystemStateNotFound)?;
    let move_object = inner
        .data
        .try_as_move()
        .ok_or(HaneulError::HaneulSystemStateNotFound)?;
    match wrapper.version {
        1 => {
            let result =
                bcs::from_bytes::<Field<u64, HaneulSystemStateInnerV1>>(move_object.contents())
                    .expect("Haneul System State object deserialization cannot fail");
            Ok(HaneulSystemState::V1(result.value))
        }
        // The following case is for sim_test only to support authority_tests::test_haneul_system_state_nop_upgrade.
        #[cfg(msim)]
        HANEUL_SYSTEM_STATE_TESTING_VERSION1 => {
            let result =
                bcs::from_bytes::<Field<u64, HaneulSystemStateInnerV1>>(move_object.contents())
                    .expect("Haneul System State object deserialization cannot fail");
            Ok(HaneulSystemState::V1(result.value))
        }
        _ => {
            error!("Unsupported Haneul System State version: {}", wrapper.version);
            Err(HaneulError::HaneulSystemStateUnexpectedVersion)
        }
    }
}

pub fn get_haneul_system_state_version(_protocol_version: ProtocolVersion) -> u64 {
    INIT_SYSTEM_STATE_VERSION
}

pub fn multiaddr_to_anemo_address(multiaddr: &Multiaddr) -> Option<anemo::types::Address> {
    use multiaddr::Protocol;
    let mut iter = multiaddr.iter();

    match (iter.next(), iter.next(), iter.next()) {
        (Some(Protocol::Ip4(ipaddr)), Some(Protocol::Udp(port)), None) => {
            Some((ipaddr, port).into())
        }
        (Some(Protocol::Ip6(ipaddr)), Some(Protocol::Udp(port)), None) => {
            Some((ipaddr, port).into())
        }
        (Some(Protocol::Dns(hostname)), Some(Protocol::Udp(port)), None) => {
            Some((hostname.as_ref(), port).into())
        }
        _ => {
            tracing::debug!("unsupported p2p multiaddr: '{multiaddr}'");
            None
        }
    }
}
