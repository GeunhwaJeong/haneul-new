// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::{context_data::db_data_provider::PgManager, data::Db};

use super::cursor::Page;
use super::object::{Object, ObjectFilter};
use super::{
    big_int::BigInt, epoch::Epoch, move_object::MoveObject, object, haneul_address::HaneulAddress,
};
use async_graphql::connection::Connection;
use async_graphql::*;
use move_core_types::language_storage::StructTag;
use haneul_json_rpc_types::{Stake as RpcStakedHaneul, StakeStatus as RpcStakeStatus};
use haneul_types::base_types::MoveObjectType;
use haneul_types::governance::StakedHaneul as NativeStakedHaneul;

#[derive(Copy, Clone, Enum, PartialEq, Eq)]
/// The stake's possible status: active, pending, or unstaked.
pub(crate) enum StakeStatus {
    /// The stake object is active in a staking pool and it is generating rewards.
    Active,
    /// The stake awaits to join a staking pool in the next epoch.
    Pending,
    /// The stake is no longer active in any staking pool.
    Unstaked,
}

pub(crate) enum StakedHaneulDowncastError {
    NotAStakedHaneul,
    Bcs(bcs::Error),
}

#[derive(Clone)]
pub(crate) struct StakedHaneul {
    /// Representation of this StakedHaneul as a generic Move Object.
    pub super_: MoveObject,

    /// Deserialized representation of the Move Object's contents as a
    /// `0x3::staking_pool::StakedHaneul`.
    pub native: NativeStakedHaneul,
}

/// Represents a 0x3::staking_pool::StakedHaneul Move object on-chain.
#[Object]
impl StakedHaneul {
    /// A stake can be pending, active, or unstaked
    async fn status(&self, ctx: &Context<'_>) -> Result<StakeStatus> {
        Ok(match self.rpc_stake(ctx).await.extend()?.status {
            RpcStakeStatus::Pending => StakeStatus::Pending,
            RpcStakeStatus::Active { .. } => StakeStatus::Active,
            RpcStakeStatus::Unstaked => StakeStatus::Unstaked,
        })
    }

    /// The epoch at which this stake became active.
    async fn activated_epoch(&self, ctx: &Context<'_>) -> Result<Option<Epoch>> {
        Epoch::query(ctx.data_unchecked(), Some(self.native.activation_epoch()))
            .await
            .extend()
    }

    /// The epoch at which this object was requested to join a stake pool.
    async fn requested_epoch(&self, ctx: &Context<'_>) -> Result<Option<Epoch>> {
        Epoch::query(ctx.data_unchecked(), Some(self.native.request_epoch()))
            .await
            .extend()
    }

    /// The HANEUL that was initially staked.
    async fn principal(&self) -> Option<BigInt> {
        Some(BigInt::from(self.native.principal()))
    }

    /// The estimated reward for this stake object, calculated as:
    ///
    ///  principal * (initial_stake_rate / current_stake_rate - 1.0)
    ///
    /// Or 0, if this value is negative, where:
    ///
    /// - `initial_stake_rate` is the stake rate at the epoch this stake was activated at.
    /// - `current_stake_rate` is the stake rate in the current epoch.
    ///
    /// This value is only available if the stake is active.
    async fn estimated_reward(&self, ctx: &Context<'_>) -> Result<Option<BigInt>, Error> {
        let RpcStakeStatus::Active { estimated_reward } = self.rpc_stake(ctx).await?.status else {
            return Ok(None);
        };

        Ok(Some(BigInt::from(estimated_reward)))
    }

    /// The corresponding `0x3::staking_pool::StakedHaneul` Move object.
    async fn as_move_object(&self) -> &MoveObject {
        &self.super_
    }
}

impl StakedHaneul {
    /// Query the database for a `page` of Staked HANEUL. The page uses the same cursor type as is used
    /// for `Object`, and is further filtered to a particular `owner`.
    pub(crate) async fn paginate(
        db: &Db,
        page: Page<object::Cursor>,
        owner: HaneulAddress,
    ) -> Result<Connection<String, StakedHaneul>, Error> {
        let type_: StructTag = MoveObjectType::staked_haneul().into();

        let filter = ObjectFilter {
            type_: Some(type_.into()),
            owner: Some(owner),
            ..Default::default()
        };

        Object::paginate_subtype(db, page, filter, |object| {
            let address = object.address;
            let move_object = MoveObject::try_from(&object).map_err(|_| {
                Error::Internal(format!(
                    "Expected {address} to be a StakedHaneul, but it's not a Move Object.",
                ))
            })?;

            StakedHaneul::try_from(&move_object).map_err(|_| {
                Error::Internal(format!(
                    "Expected {address} to be a StakedHaneul, but it is not."
                ))
            })
        })
        .await
    }

    /// The JSON-RPC representation of a StakedHaneul so that we can "cheat" to implement fields that
    /// are not yet implemented directly for GraphQL.
    ///
    /// TODO: Make this obsolete
    async fn rpc_stake(&self, ctx: &Context<'_>) -> Result<RpcStakedHaneul, Error> {
        ctx.data_unchecked::<PgManager>()
            .fetch_rpc_staked_haneul(self.native.clone())
            .await
    }
}

impl TryFrom<&MoveObject> for StakedHaneul {
    type Error = StakedHaneulDowncastError;

    fn try_from(move_object: &MoveObject) -> Result<Self, Self::Error> {
        if !move_object.native.is_staked_haneul() {
            return Err(StakedHaneulDowncastError::NotAStakedHaneul);
        }

        Ok(Self {
            super_: move_object.clone(),
            native: bcs::from_bytes(move_object.native.contents())
                .map_err(StakedHaneulDowncastError::Bcs)?,
        })
    }
}
