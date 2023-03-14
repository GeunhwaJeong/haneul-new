// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::types::SubscriptionResult;
use jsonrpsee::{RpcModule, SubscriptionSink};

use haneul_core::event_handler::EventHandler;
use haneul_json_rpc::api::EventReadApiClient;
use haneul_json_rpc::api::EventReadApiServer;
use haneul_json_rpc::event_api::spawn_subscription;
use haneul_json_rpc::HaneulRpcModule;
use haneul_json_rpc_types::{EventFilter, EventPage, HaneulEvent};
use haneul_open_rpc::Module;
use haneul_types::digests::TransactionDigest;
use haneul_types::event::EventID;

use crate::errors::IndexerError;
use crate::store::IndexerStore;

pub(crate) struct EventReadApi<S> {
    state: S,
    fullnode: HttpClient,
    event_handler: Arc<EventHandler>,
    method_to_be_forwarded: Vec<String>,
}

impl<S: IndexerStore> EventReadApi<S> {
    pub fn new(state: S, fullnode_client: HttpClient, event_handler: Arc<EventHandler>) -> Self {
        Self {
            state,
            fullnode: fullnode_client,
            // TODO: read from centralized config
            event_handler,
            method_to_be_forwarded: vec![],
        }
    }

    pub fn get_events_internal(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> Result<EventPage, IndexerError> {
        self.state
            .get_events(query, cursor, limit, descending_order.unwrap_or_default())
    }
}

#[async_trait]
impl<S> EventReadApiServer for EventReadApi<S>
where
    S: IndexerStore + Sync + Send + 'static,
{
    async fn query_events(
        &self,
        query: EventFilter,
        // exclusive cursor if `Some`, otherwise start from the beginning
        cursor: Option<EventID>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> RpcResult<EventPage> {
        if self
            .method_to_be_forwarded
            .contains(&"get_events".to_string())
        {
            return self
                .fullnode
                .query_events(query, cursor, limit, descending_order)
                .await;
        }
        Ok(self.get_events_internal(query, cursor, limit, descending_order)?)
    }

    fn subscribe_event(&self, sink: SubscriptionSink, filter: EventFilter) -> SubscriptionResult {
        spawn_subscription(sink, self.event_handler.subscribe(filter));
        Ok(())
    }
    async fn get_events(&self, transaction_digest: TransactionDigest) -> RpcResult<Vec<HaneulEvent>> {
        self.fullnode.get_events(transaction_digest).await
    }
}

impl<S> HaneulRpcModule for EventReadApi<S>
where
    S: IndexerStore + Sync + Send + 'static,
{
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        haneul_json_rpc::api::EventReadApiOpenRpc::module_doc()
    }
}
