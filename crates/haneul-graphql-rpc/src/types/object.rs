// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{connection::Connection, *};

use super::{
    balance::{Balance, BalanceConnection},
    coin::CoinConnection,
    name_service::NameServiceConnection,
    owner::Owner,
    stake::StakeConnection,
    haneul_address::HaneulAddress,
    transaction_block::TransactionBlock,
};
use crate::{
    server::data_provider::{fetch_balance, fetch_owned_objs, fetch_tx},
    types::base64::Base64,
};

pub(crate) struct Object {
    pub address: HaneulAddress,
    pub version: u64,
    pub digest: String,
    pub storage_rebate: Option<u64>,
    pub owner: Option<HaneulAddress>,
    pub bcs: Option<Base64>,
    pub previous_transaction: Option<String>,
    pub kind: Option<ObjectKind>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ObjectKind {
    Owned,
    Child,
    Shared,
    Immutable,
}

#[derive(InputObject)]
pub(crate) struct ObjectFilter {
    package: Option<HaneulAddress>,
    module: Option<String>,
    ty: Option<String>,

    owner: Option<HaneulAddress>,
    object_id: Option<HaneulAddress>,
    version: Option<u64>,
}

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

    async fn storage_rebate(&self) -> Option<u64> {
        self.storage_rebate
    }

    async fn bcs(&self) -> Option<Base64> {
        self.bcs.clone()
    }

    async fn previous_transaction_block(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<TransactionBlock>> {
        if let Some(tx) = &self.previous_transaction {
            fetch_tx(ctx.data_unchecked::<haneul_sdk::HaneulClient>(), tx).await
        } else {
            Ok(None)
        }
    }

    async fn kind(&self) -> Option<ObjectKind> {
        self.kind
    }

    async fn owner(&self) -> Option<Owner> {
        self.owner.as_ref().map(|q| Owner { address: q.clone() })
    }

    // =========== Owner interface methods =============

    pub async fn location(&self) -> HaneulAddress {
        self.address.clone()
    }

    pub async fn object_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        filter: Option<ObjectFilter>,
    ) -> Result<Connection<String, Object>> {
        fetch_owned_objs(
            ctx.data_unchecked::<haneul_sdk::HaneulClient>(),
            &self.address,
            first,
            after,
            last,
            before,
            filter,
        )
        .await
    }

    pub async fn balance(&self, ctx: &Context<'_>, type_: Option<String>) -> Result<Balance> {
        fetch_balance(
            ctx.data_unchecked::<haneul_sdk::HaneulClient>(),
            &self.address,
            type_,
        )
        .await
    }

    pub async fn balance_connection(
        &self,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Option<BalanceConnection> {
        unimplemented!()
    }

    pub async fn coin_connection(
        &self,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        type_: Option<String>,
    ) -> Option<CoinConnection> {
        unimplemented!()
    }

    pub async fn stake_connection(
        &self,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Option<StakeConnection> {
        unimplemented!()
    }

    pub async fn default_name_service_name(&self) -> Option<String> {
        unimplemented!()
    }

    pub async fn name_service_connection(
        &self,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Option<NameServiceConnection> {
        unimplemented!()
    }
}
