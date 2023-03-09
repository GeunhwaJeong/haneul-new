// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::StructTag;

use crate::balance::Balance;
use crate::base_types::{ObjectID, HaneulAddress};
use crate::committee::EpochId;
use crate::error::HaneulError;
use crate::id::{ID, UID};
use crate::object::{Data, Object};
use crate::HANEUL_FRAMEWORK_ADDRESS;
use serde::Deserialize;
use serde::Serialize;

/// Minimum amount of stake required for a validator to be in the validator set
pub const MINIMUM_VALIDATOR_STAKE_HANEUL: u64 = 25_000_000;

pub const STAKING_POOL_MODULE_NAME: &IdentStr = ident_str!("staking_pool");
pub const STAKED_HANEUL_STRUCT_NAME: &IdentStr = ident_str!("StakedHaneul");

pub const ADD_STAKE_MUL_COIN_FUN_NAME: &IdentStr = ident_str!("request_add_stake_mul_coin");
pub const ADD_STAKE_FUN_NAME: &IdentStr = ident_str!("request_add_stake_mul_coin");
pub const ADD_STAKE_LOCKED_COIN_FUN_NAME: &IdentStr =
    ident_str!("request_add_stake_mul_locked_coin");
pub const WITHDRAW_STAKE_FUN_NAME: &IdentStr = ident_str!("request_withdraw_stake");

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct StakedHaneul {
    id: UID,
    pool_id: ID,
    validator_address: HaneulAddress,
    stake_activation_epoch: u64,
    principal: Balance,
    haneul_token_lock: Option<EpochId>,
}

impl StakedHaneul {
    pub fn type_() -> StructTag {
        StructTag {
            address: HANEUL_FRAMEWORK_ADDRESS,
            module: STAKING_POOL_MODULE_NAME.to_owned(),
            name: STAKED_HANEUL_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }

    pub fn is_staked_haneul(s: &StructTag) -> bool {
        s.address == HANEUL_FRAMEWORK_ADDRESS
            && s.module.as_ident_str() == STAKING_POOL_MODULE_NAME
            && s.name.as_ident_str() == STAKED_HANEUL_STRUCT_NAME
            && s.type_params.is_empty()
    }

    pub fn id(&self) -> ObjectID {
        self.id.id.bytes
    }

    pub fn pool_id(&self) -> ObjectID {
        self.pool_id.bytes
    }

    pub fn request_epoch(&self) -> EpochId {
        self.stake_activation_epoch
    }

    pub fn principal(&self) -> u64 {
        self.principal.value()
    }

    pub fn validator_address(&self) -> HaneulAddress {
        self.validator_address
    }

    pub fn haneul_token_lock(&self) -> Option<EpochId> {
        self.haneul_token_lock
    }
}

impl TryFrom<&Object> for StakedHaneul {
    type Error = HaneulError;
    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        match &object.data {
            Data::Move(o) => {
                if o.type_().is_staked_haneul() {
                    return bcs::from_bytes(o.contents()).map_err(|err| HaneulError::TypeError {
                        error: format!("Unable to deserialize StakedHaneul object: {:?}", err),
                    });
                }
            }
            Data::Package(_) => {}
        }

        Err(HaneulError::TypeError {
            error: format!("Object type is not a StakedHaneul: {:?}", object),
        })
    }
}
