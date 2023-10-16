// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{connection::Connection, *};
use haneul_json_rpc::name_service::NameServiceConfig;

use super::big_int::BigInt;
use super::digest::Digest;
use super::move_package::MovePackage;
use super::name_service::NameService;
use super::{
    balance::Balance, coin::Coin, owner::Owner, stake::Stake, haneul_address::HaneulAddress,
    transaction_block::TransactionBlock,
};
use crate::context_data::db_data_provider::PgManager;
use crate::types::base64::Base64;
use haneul_types::digests::TransactionDigest as NativeHaneulTransactionDigest;
use haneul_types::move_package::MovePackage as NativeHaneulMovePackage;
use haneul_types::object::{Data as NativeHaneulObjectData, Object as NativeHaneulObject};

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct Object {
    pub address: HaneulAddress,
    pub version: u64,
    pub digest: String,
    pub storage_rebate: Option<BigInt>,
    pub owner: Option<HaneulAddress>,
    pub bcs: Option<Base64>,
    pub previous_transaction: Option<Digest>,
    pub kind: Option<ObjectKind>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ObjectKind {
    Owned,
    Child,
    Shared,
    Immutable,
}

#[derive(InputObject, Default)]
pub(crate) struct ObjectFilter {
    pub package: Option<HaneulAddress>,
    pub module: Option<String>,
    pub ty: Option<String>,

    pub owner: Option<HaneulAddress>,
    pub object_ids: Option<Vec<HaneulAddress>>,
    pub object_keys: Option<Vec<ObjectKey>>,
}

#[derive(InputObject)]
pub(crate) struct ObjectKey {
    object_id: HaneulAddress,
    version: u64,
}

#[allow(clippy::diverging_sub_expression)]
#[allow(unreachable_code)]
#[allow(unused_variables)]
#[Object]
impl Object {
    async fn version(&self) -> u64 {
        self.version
    }

    async fn digest(&self) -> String {
        self.digest.clone()
    }

    async fn storage_rebate(&self) -> Option<BigInt> {
        self.storage_rebate.clone()
    }

    async fn bcs(&self) -> Option<Base64> {
        self.bcs.clone()
    }

    async fn previous_transaction_block(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<TransactionBlock>, crate::error::Error> {
        match self.previous_transaction {
            Some(digest) => {
                ctx.data_unchecked::<PgManager>()
                    .fetch_tx(digest.to_string().as_str())
                    .await
            }
            None => Ok(None),
        }
    }

    async fn kind(&self) -> Option<ObjectKind> {
        self.kind
    }

    async fn owner(&self) -> Option<Owner> {
        self.owner.as_ref().map(|q| Owner { address: *q })
    }

    async fn as_move_package(&self) -> Result<Option<MovePackage>> {
        if let Some(bcs) = &self.bcs {
            let bytes = bcs.0.as_slice();

            let package = bcs::from_bytes::<NativeHaneulMovePackage>(bytes)
                .map_err(|e| Error::from(format!("Failed to deserialize package: {}", e)))?;

            Ok(Some(MovePackage {
                native_object: NativeHaneulObject::new_package_from_data(
                    NativeHaneulObjectData::Package(package),
                    self.previous_transaction
                        .map(|x| NativeHaneulTransactionDigest::new(x.into_array()))
                        .ok_or(Error::new("Object must have a previous transaction digest"))?,
                ),
            }))
        } else {
            Ok(None)
        }
    }

    // =========== Owner interface methods =============

    pub async fn location(&self) -> HaneulAddress {
        self.address
    }

    pub async fn object_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        filter: Option<ObjectFilter>,
    ) -> Result<Option<Connection<String, Object>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_owned_objs(first, after, last, before, filter, self.address)
            .await
            .extend()
    }

    pub async fn balance(&self, ctx: &Context<'_>, type_: String) -> Result<Option<Balance>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_balance(self.address, type_)
            .await
            .extend()
    }

    pub async fn balance_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Result<Option<Connection<String, Balance>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_balances(self.address, first, after, last, before)
            .await
            .extend()
    }

    pub async fn coin_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        type_: Option<String>,
    ) -> Result<Option<Connection<String, Coin>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_coins(self.address, type_, first, after, last, before)
            .await
            .extend()
    }

    pub async fn stake_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Result<Option<Connection<String, Stake>>> {
        ctx.data_unchecked::<PgManager>()
            .fetch_staked_haneul(self.address, first, after, last, before)
            .await
            .extend()
    }

    pub async fn default_name_service_name(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        ctx.data_unchecked::<PgManager>()
            .default_name_service_name(ctx.data_unchecked::<NameServiceConfig>(), self.address)
            .await
            .extend()
    }

    pub async fn name_service_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Result<Option<Connection<String, NameService>>> {
        unimplemented!()
    }
}

impl From<&NativeHaneulObject> for Object {
    fn from(o: &NativeHaneulObject) -> Self {
        let kind = Some(match o.owner {
            haneul_types::object::Owner::AddressOwner(_) => ObjectKind::Owned,
            haneul_types::object::Owner::ObjectOwner(_) => ObjectKind::Child,
            haneul_types::object::Owner::Shared {
                initial_shared_version: _,
            } => ObjectKind::Shared,
            haneul_types::object::Owner::Immutable => ObjectKind::Immutable,
        });

        let owner_address = o.owner.get_owner_address().ok();
        if matches!(kind, Some(ObjectKind::Immutable) | Some(ObjectKind::Shared))
            && owner_address.is_some()
        {
            panic!("Immutable or Shared object should not have an owner_id");
        }

        let bcs = match &o.data {
            // Do we BCS serialize packages?
            NativeHaneulObjectData::Package(package) => Base64::from(
                bcs::to_bytes(package)
                    .expect("Failed to serialize package")
                    .to_vec(),
            ),
            NativeHaneulObjectData::Move(move_object) => Base64::from(move_object.contents()),
        };

        Self {
            address: HaneulAddress::from_array(o.id().into_bytes()),
            version: o.version().into(),
            digest: o.digest().base58_encode(),
            storage_rebate: Some(BigInt::from(o.storage_rebate)),
            owner: owner_address.map(HaneulAddress::from),
            bcs: Some(bcs),
            previous_transaction: Some(Digest::from_array(o.previous_transaction.into_inner())),
            kind,
        }
    }
}
