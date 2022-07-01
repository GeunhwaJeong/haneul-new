// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{
    account_address::AccountAddress, ident_str, identifier::IdentStr, language_storage::StructTag,
};
use serde::{Deserialize, Serialize};

use crate::{
    balance::{Balance, Supply},
    id::VersionedID,
    HANEUL_FRAMEWORK_ADDRESS,
};

const HANEUL_SYSTEM_STATE_STRUCT_NAME: &IdentStr = ident_str!("HaneulSystemState");
pub const HANEUL_SYSTEM_MODULE_NAME: &IdentStr = ident_str!("haneul_system");
pub const ADVANCE_EPOCH_FUNCTION_NAME: &IdentStr = ident_str!("advance_epoch");

/// Rust version of the Move haneul::haneul_system::SystemParameters type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SystemParameters {
    pub min_validator_stake: u64,
    pub max_validator_candidate_count: u64,
    pub storage_gas_price: u64,
}

/// Rust version of the Move Std::Option::Option type.
/// Putting it in this file because it's only used here.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MoveOption<T> {
    pub vec: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorMetadata {
    pub haneul_address: AccountAddress,
    pub pubkey_bytes: Vec<u8>,
    pub name: Vec<u8>,
    pub net_address: Vec<u8>,
    pub next_epoch_stake: u64,
    pub next_epoch_delegation: u64,
}

/// Rust version of the Move haneul::validator::Validator type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Validator {
    pub metadata: ValidatorMetadata,
    pub stake_amount: u64,
    pub delegation: u64,
    pub pending_stake: u64,
    pub pending_withdraw: u64,
    pub pending_delegation: u64,
    pub pending_delegation_withdraw: u64,
    pub delegator_count: u64,
    pub pending_delegator_count: u64,
    pub pending_delegator_withdraw_count: u64,
}

/// Rust version of the Move haneul::validator_set::ValidatorSet type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidatorSet {
    pub validator_stake: u64,
    pub delegation_stake: u64,
    pub quorum_stake_threshold: u64,
    pub active_validators: Vec<Validator>,
    pub pending_validators: Vec<Validator>,
    pub pending_removals: Vec<u64>,
    pub next_epoch_validators: Vec<ValidatorMetadata>,
}

/// Rust version of the Move haneul::haneul_system::HaneulSystemState type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct HaneulSystemState {
    pub id: VersionedID,
    pub epoch: u64,
    pub validators: ValidatorSet,
    pub treasury_cap: Supply,
    pub storage_fund: Balance,
    pub parameters: SystemParameters,
    pub delegation_reward: Balance,
    // TODO: Use getters instead of all pub.
}

impl HaneulSystemState {
    pub fn type_() -> StructTag {
        StructTag {
            address: HANEUL_FRAMEWORK_ADDRESS,
            name: HANEUL_SYSTEM_STATE_STRUCT_NAME.to_owned(),
            module: HANEUL_SYSTEM_MODULE_NAME.to_owned(),
            type_params: vec![],
        }
    }
}
