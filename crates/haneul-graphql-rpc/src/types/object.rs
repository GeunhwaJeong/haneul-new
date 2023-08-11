// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::*;

use super::{
    balance::{Balance, BalanceConnection},
    coin::CoinConnection,
    name_service::NameServiceConnection,
    stake::StakeConnection,
    haneul_address::HaneulAddress,
    transaction_block::TransactionBlock,
};
use crate::{
    server::data_provider::{fetch_balance, fetch_tx},
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
}

pub(crate) struct ObjectConnection;

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

    // =========== Owner interface methods =============

    pub async fn location(&self) -> HaneulAddress {
        self.address.clone()
    }

    pub async fn object_connection(
        &self,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        filter: Option<ObjectFilter>,
    ) -> Option<ObjectConnection> {
        unimplemented!()
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

#[allow(unreachable_code)]
#[allow(unused_variables)]
#[Object]
impl ObjectConnection {
    async fn unimplemented(&self) -> bool {
        unimplemented!()
    }
}
