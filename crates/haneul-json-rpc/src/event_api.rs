// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use std::sync::Arc;

use async_trait::async_trait;
use futures::StreamExt;
use jsonrpsee::core::RpcResult;
use jsonrpsee::types::SubscriptionResult;
use jsonrpsee_core::server::rpc_module::RpcModule;
use jsonrpsee_core::server::rpc_module::SubscriptionSink;
use tracing::warn;

use haneul_core::authority::AuthorityState;
use haneul_core::event_handler::EventHandler;
use haneul_json_rpc_types::{EventPage, HaneulEvent, HaneulEventEnvelope, HaneulEventFilter};
use haneul_open_rpc::Module;
use haneul_types::event::{EventEnvelope, EventID};
use haneul_types::query::EventQuery;

use crate::api::EventReadApiServer;
use crate::api::{cap_page_limit, EventStreamingApiServer};
use crate::streaming_api::spawn_subscription;
use crate::HaneulRpcModule;

pub struct EventStreamingApiImpl {
    state: Arc<AuthorityState>,
    event_handler: Arc<EventHandler>,
}

impl EventStreamingApiImpl {
    pub fn new(state: Arc<AuthorityState>, event_handler: Arc<EventHandler>) -> Self {
        Self {
            state,
            event_handler,
        }
    }
}

#[async_trait]
impl EventStreamingApiServer for EventStreamingApiImpl {
    fn subscribe_event(
        &self,
        mut sink: SubscriptionSink,
        filter: HaneulEventFilter,
    ) -> SubscriptionResult {
        let filter = match filter.try_into() {
            Ok(filter) => filter,
            Err(e) => {
                let e = jsonrpsee_core::Error::from(e);
                warn!(error = ?e, "Rejecting subscription request.");
                return Ok(sink.reject(e)?);
            }
        };

        let state = self.state.clone();
        let stream = self.event_handler.subscribe(filter);
        let stream = stream.map(move |e: EventEnvelope| {
            let event = HaneulEvent::try_from(e.event, state.module_cache.as_ref());
            event.map(|event| HaneulEventEnvelope {
                timestamp: e.timestamp,
                tx_digest: e.tx_digest,
                id: EventID::from((e.seq_num as i64, e.event_num as i64)),
                event,
            })
        });
        spawn_subscription(sink, stream);
        Ok(())
    }
}

impl HaneulRpcModule for EventStreamingApiImpl {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        crate::api::EventStreamingApiOpenRpc::module_doc()
    }
}

#[allow(unused)]
pub struct EventReadApiImpl {
    state: Arc<AuthorityState>,
    event_handler: Arc<EventHandler>,
}

impl EventReadApiImpl {
    pub fn new(state: Arc<AuthorityState>, event_handler: Arc<EventHandler>) -> Self {
        Self {
            state,
            event_handler,
        }
    }
}

#[allow(unused)]
#[async_trait]
impl EventReadApiServer for EventReadApiImpl {
    async fn get_events(
        &self,
        query: EventQuery,
        cursor: Option<EventID>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> RpcResult<EventPage> {
        let descending = descending_order.unwrap_or_default();
        let limit = cap_page_limit(limit)?;
        // Retrieve 1 extra item for next cursor
        let mut data = self
            .state
            .get_events(query, cursor, limit + 1, descending)
            .await?;
        let next_cursor = data.get(limit).map(|(id, _)| id.clone());
        data.truncate(limit);
        let data = data.into_iter().map(|(_, event)| event).collect();
        Ok(EventPage { data, next_cursor })
    }
}

impl HaneulRpcModule for EventReadApiImpl {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        crate::api::EventReadApiOpenRpc::module_doc()
    }
}
