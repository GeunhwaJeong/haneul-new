// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use futures::future;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use serde::{Deserialize, Serialize};
use haneul_json_rpc_types::{Page, HaneulTransactionBlockResponse, HaneulTransactionBlockResponseOptions};
use haneul_open_rpc::Module;
use haneul_open_rpc_macros::open_rpc;
use haneul_types::digests::TransactionDigest;

use self::{error::Error, filter::HaneulTransactionBlockResponseQuery};

use crate::{
    context::Context,
    error::{rpc_bail, InternalContext, RpcError},
};

use super::rpc_module::RpcModule;

mod error;
mod filter;
mod response;

#[open_rpc(namespace = "haneul", tag = "Transactions API")]
#[rpc(server, namespace = "haneul")]
trait TransactionsApi {
    /// Fetch a transaction by its transaction digest.
    #[method(name = "getTransactionBlock")]
    async fn get_transaction_block(
        &self,
        /// The digest of the queried transaction.
        digest: TransactionDigest,
        /// Options controlling the output format.
        options: HaneulTransactionBlockResponseOptions,
    ) -> RpcResult<HaneulTransactionBlockResponse>;
}

#[open_rpc(namespace = "haneulx", tag = "Query Transactions API")]
#[rpc(server, namespace = "haneulx")]
trait QueryTransactionsApi {
    /// Query transactions based on their properties (sender, affected addresses, function calls,
    /// etc). Returns a paginated list of transactions.
    ///
    /// If a cursor is provided, the query will start from the transaction after the one pointed to
    /// by this cursor, otherwise pagination starts from the first transaction that meets the query
    /// criteria.
    ///
    /// The definition of "first" transaction is changed by the `descending_order` parameter, which
    /// is optional, and defaults to false, meaning that the oldest transaction is shown first.
    ///
    /// The size of each page is controlled by the `limit` parameter.
    #[method(name = "queryTransactionBlocks")]
    async fn query_transaction_blocks(
        &self,
        /// The query criteria, and the output options.
        query: HaneulTransactionBlockResponseQuery,
        /// Cursor to start paginating from.
        cursor: Option<String>,
        /// Maximum number of transactions to return per page.
        limit: Option<usize>,
        /// Order of results, defaulting to ascending order (false), by sequence on-chain.
        descending_order: Option<bool>,
    ) -> RpcResult<Page<HaneulTransactionBlockResponse, String>>;
}

pub(crate) struct Transactions(pub Context);

pub(crate) struct QueryTransactions(pub Context, pub TransactionsConfig);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionsConfig {
    /// The default page size limit when querying transactions, if none is provided.
    pub default_page_size: usize,

    /// The largest acceptable page size when querying transactions. Requesting a page larger than
    /// this is a user error.
    pub max_page_size: usize,
}

#[async_trait::async_trait]
impl TransactionsApiServer for Transactions {
    async fn get_transaction_block(
        &self,
        digest: TransactionDigest,
        options: HaneulTransactionBlockResponseOptions,
    ) -> RpcResult<HaneulTransactionBlockResponse> {
        let Self(ctx) = self;
        Ok(response::transaction(ctx, digest, &options)
            .await
            .with_internal_context(|| format!("Failed to get transaction {digest}"))?)
    }
}

#[async_trait::async_trait]
impl QueryTransactionsApiServer for QueryTransactions {
    async fn query_transaction_blocks(
        &self,
        query: HaneulTransactionBlockResponseQuery,
        cursor: Option<String>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> RpcResult<Page<HaneulTransactionBlockResponse, String>> {
        let Self(ctx, config) = self;

        let Page {
            data: digests,
            next_cursor,
            has_next_page,
        } = filter::transactions(ctx, config, &query.filter, cursor, limit, descending_order)
            .await?;

        let options = query.options.unwrap_or_default();

        let tx_futures = digests
            .iter()
            .map(|d| response::transaction(ctx, *d, &options));

        let data = future::join_all(tx_futures)
            .await
            .into_iter()
            .zip(digests)
            .map(|(r, d)| {
                if let Err(RpcError::InvalidParams(e @ Error::NotFound(_))) = r {
                    rpc_bail!(e)
                } else {
                    r.with_internal_context(|| format!("Failed to get transaction {d}"))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Page {
            data,
            next_cursor,
            has_next_page,
        })
    }
}

impl RpcModule for Transactions {
    fn schema(&self) -> Module {
        TransactionsApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}

impl RpcModule for QueryTransactions {
    fn schema(&self) -> Module {
        QueryTransactionsApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}

impl Default for TransactionsConfig {
    fn default() -> Self {
        Self {
            default_page_size: 50,
            max_page_size: 100,
        }
    }
}
