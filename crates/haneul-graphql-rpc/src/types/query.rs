// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::*;

use super::{address::Address, object::Object, owner::ObjectOwner, haneul_address::HaneulAddress};
use crate::server::data_provider::fetch_chain_id;

pub(crate) struct Query;
pub(crate) type HaneulGraphQLSchema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

#[allow(unreachable_code)]
#[allow(unused_variables)]
#[Object]
impl Query {
    async fn chain_identifier(&self, ctx: &Context<'_>) -> Result<String> {
        fetch_chain_id(ctx.data_unchecked::<haneul_sdk::HaneulClient>()).await
    }

    async fn owner(&self, ctx: &Context<'_>, address: HaneulAddress) -> Result<Option<ObjectOwner>> {
        // Currently only an account address can own an object
        let cl = ctx.data_unchecked::<haneul_sdk::HaneulClient>();
        let o = crate::server::data_provider::fetch_obj(cl, address, None).await?;
        Ok(o.and_then(|q| q.owner)
            .map(|o| ObjectOwner::Address(Address { address: o })))
    }

    async fn object(
        &self,
        ctx: &Context<'_>,
        address: HaneulAddress,
        version: Option<u64>,
    ) -> Result<Option<Object>> {
        let cl = ctx.data_unchecked::<haneul_sdk::HaneulClient>();
        crate::server::data_provider::fetch_obj(cl, address, version).await
    }

    async fn address(&self, address: HaneulAddress) -> Option<Address> {
        Some(Address {
            address: address.clone(),
        })
    }
}
