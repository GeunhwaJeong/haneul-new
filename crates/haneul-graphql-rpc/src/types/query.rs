// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::*;

use super::{address::Address, object::Object, owner::Owner, haneul_address::HaneulAddress};

pub(crate) struct Query;

pub(crate) type HaneulGraphQLSchema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

#[allow(unreachable_code)]
#[allow(unused_variables)]
#[Object]
impl Query {
    async fn chain_identifier(&self) -> String {
        "0000".to_string()
    }

    async fn owner(&self, address: HaneulAddress) -> Option<Owner> {
        None
    }

    async fn object(&self, address: HaneulAddress, version: Option<u64>) -> Option<Object> {
        None
    }

    async fn address(&self, address: HaneulAddress) -> Option<Address> {
        None
    }
}
