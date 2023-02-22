// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee_proc_macros::rpc;

use haneul_json_rpc_types::{EventPage, HaneulEventFilter};
use haneul_open_rpc_macros::open_rpc;
use haneul_types::event::EventID;

use haneul_json_rpc_types::HaneulEventEnvelope;
use haneul_types::query::EventQuery;

#[open_rpc(namespace = "haneul", tag = "Event Read API")]
#[rpc(server, client, namespace = "haneul")]
pub trait EventReadApi {
    /// Return list of events for a specified query criteria.
    #[method(name = "getEvents")]
    async fn get_events(
        &self,
        /// the event query criteria.
        query: EventQuery,
        /// optional paging cursor
        cursor: Option<EventID>,
        /// maximum number of items per page, default to [QUERY_MAX_RESULT_LIMIT] if not specified.
        limit: Option<usize>,
        /// query result ordering, default to false (ascending order), oldest record first.
        descending_order: Option<bool>,
    ) -> RpcResult<EventPage>;

    /// Subscribe to a stream of Haneul event
    #[subscription(name = "subscribeEvent", item = HaneulEventEnvelope)]
    fn subscribe_event(
        &self,
        /// the filter criteria of the event stream, see the [Haneul docs](https://docs.haneul.io/build/pubsub#event-filters) for detailed examples.
        filter: HaneulEventFilter,
    );
}
