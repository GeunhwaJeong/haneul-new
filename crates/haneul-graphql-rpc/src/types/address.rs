// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{
    balance::{self, Balance},
    coin::Coin,
    cursor::Page,
    move_object::MoveObject,
    object::{self, ObjectFilter},
    owner::OwnerImpl,
    stake::StakedHaneul,
    haneul_address::HaneulAddress,
    haneulns_registration::HaneulnsRegistration,
    transaction_block::{self, TransactionBlock, TransactionBlockFilter},
    type_filter::ExactTypeFilter,
};
use async_graphql::{connection::Connection, *};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub(crate) struct Address {
    pub address: HaneulAddress,
}

/// The possible relationship types for a transaction block: sign, sent, received, or paid.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub(crate) enum AddressTransactionBlockRelationship {
    /// Transactions this address has signed either as a sender or as a sponsor.
    Sign,
    /// Transactions that sent objects to this address.
    Recv,
}

/// The 32-byte address that is an account address (corresponding to a public key).
#[Object]
impl Address {
    pub(crate) async fn address(&self) -> HaneulAddress {
        OwnerImpl(self.address).address().await
    }

    /// Objects owned by this address, optionally `filter`-ed.
    pub(crate) async fn objects(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
        filter: Option<ObjectFilter>,
    ) -> Result<Connection<String, MoveObject>> {
        OwnerImpl(self.address)
            .objects(ctx, first, after, last, before, filter)
            .await
    }

    /// Total balance of all coins with marker type owned by this address. If type is not supplied,
    /// it defaults to `0x2::haneul::HANEUL`.
    pub(crate) async fn balance(
        &self,
        ctx: &Context<'_>,
        type_: Option<ExactTypeFilter>,
    ) -> Result<Option<Balance>> {
        OwnerImpl(self.address).balance(ctx, type_).await
    }

    /// The balances of all coin types owned by this address.
    pub(crate) async fn balances(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<balance::Cursor>,
        last: Option<u64>,
        before: Option<balance::Cursor>,
    ) -> Result<Connection<String, Balance>> {
        OwnerImpl(self.address)
            .balances(ctx, first, after, last, before)
            .await
    }

    /// The coin objects for this address.
    ///
    ///`type` is a filter on the coin's type parameter, defaulting to `0x2::haneul::HANEUL`.
    pub(crate) async fn coins(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
        type_: Option<ExactTypeFilter>,
    ) -> Result<Connection<String, Coin>> {
        OwnerImpl(self.address)
            .coins(ctx, first, after, last, before, type_)
            .await
    }

    /// The `0x3::staking_pool::StakedHaneul` objects owned by this address.
    pub(crate) async fn staked_haneuls(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
    ) -> Result<Connection<String, StakedHaneul>> {
        OwnerImpl(self.address)
            .staked_haneuls(ctx, first, after, last, before)
            .await
    }

    /// The domain explicitly configured as the default domain pointing to this address.
    pub(crate) async fn default_haneulns_name(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        OwnerImpl(self.address).default_haneulns_name(ctx).await
    }

    /// The HaneulnsRegistration NFTs owned by this address. These grant the owner the capability to
    /// manage the associated domain.
    pub(crate) async fn haneulns_registrations(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::Cursor>,
        last: Option<u64>,
        before: Option<object::Cursor>,
    ) -> Result<Connection<String, HaneulnsRegistration>> {
        OwnerImpl(self.address)
            .haneulns_registrations(ctx, first, after, last, before)
            .await
    }

    /// Similar behavior to the `transactionBlocks` in Query but supporting the additional
    /// `AddressTransactionBlockRelationship` filter, which defaults to `SIGN`.
    async fn transaction_blocks(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<transaction_block::Cursor>,
        last: Option<u64>,
        before: Option<transaction_block::Cursor>,
        relation: Option<AddressTransactionBlockRelationship>,
        filter: Option<TransactionBlockFilter>,
    ) -> Result<Connection<String, TransactionBlock>> {
        use AddressTransactionBlockRelationship as R;
        let page = Page::from_params(ctx.data_unchecked(), first, after, last, before)?;

        let Some(filter) = filter.unwrap_or_default().intersect(match relation {
            // Relationship defaults to "signer" if none is supplied.
            Some(R::Sign) | None => TransactionBlockFilter {
                sign_address: Some(self.address),
                ..Default::default()
            },

            Some(R::Recv) => TransactionBlockFilter {
                recv_address: Some(self.address),
                ..Default::default()
            },
        }) else {
            return Ok(Connection::new(false, false));
        };

        TransactionBlock::paginate(ctx.data_unchecked(), page, filter)
            .await
            .extend()
    }
}
