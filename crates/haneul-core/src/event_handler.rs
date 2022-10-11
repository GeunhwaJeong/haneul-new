// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use move_bytecode_utils::module_cache::SyncModuleCache;
use haneul_json_rpc_types::HaneulMoveStruct;
use tokio_stream::Stream;
use tracing::{debug, error, instrument, trace};

use haneul_storage::event_store::{EventStore, EventStoreType};
use haneul_types::base_types::TransactionDigest;
use haneul_types::{
    error::{HaneulError, HaneulResult},
    event::{Event, EventEnvelope},
    messages::TransactionEffects,
};

use crate::authority::{AuthorityStore, ResolverWrapper};
use crate::streamer::Streamer;
use haneul_types::filter::EventFilter;

#[cfg(test)]
#[path = "unit_tests/event_handler_tests.rs"]
mod event_handler_tests;

pub const EVENT_DISPATCH_BUFFER_SIZE: usize = 1000;

pub struct EventHandler {
    module_cache: Arc<SyncModuleCache<ResolverWrapper<AuthorityStore>>>,
    event_streamer: Streamer<EventEnvelope, EventFilter>,
    pub(crate) event_store: Arc<EventStoreType>,
}

impl EventHandler {
    pub fn new(validator_store: Arc<AuthorityStore>, event_store: Arc<EventStoreType>) -> Self {
        let streamer = Streamer::spawn(EVENT_DISPATCH_BUFFER_SIZE);
        Self {
            module_cache: Arc::new(SyncModuleCache::new(ResolverWrapper(validator_store))),
            event_streamer: streamer,
            event_store,
        }
    }

    #[instrument(level = "debug", skip_all, fields(seq=?seq_num, tx_digest=?effects.transaction_digest), err)]
    pub async fn process_events(
        &self,
        effects: &TransactionEffects,
        timestamp_ms: u64,
        seq_num: u64,
    ) -> HaneulResult {
        let res: Result<Vec<_>, _> = effects
            .events
            .iter()
            .map(|e| self.create_envelope(e, effects.transaction_digest, seq_num, timestamp_ms))
            .collect();
        let envelopes = res?;

        // Ingest all envelopes together at once (for efficiency) into Event Store
        self.event_store.add_events(&envelopes).await?;
        trace!(
            num_events = envelopes.len(),
            tx_digest =? effects.transaction_digest,
            "Finished writing events to event store"
        );

        // serially dispatch event processing to honor events' orders.
        for envelope in envelopes {
            if let Err(e) = self.event_streamer.send(envelope).await {
                error!(error =? e, "Failed to send EventEnvelope to dispatch");
            }
        }

        Ok(())
    }

    fn create_envelope(
        &self,
        event: &Event,
        digest: TransactionDigest,
        seq_num: u64,
        timestamp_ms: u64,
    ) -> Result<EventEnvelope, HaneulError> {
        let json_value = match event {
            Event::MoveEvent {
                type_, contents, ..
            } => {
                debug!(event =? event, "Process MoveEvent.");
                let move_struct =
                    Event::move_event_to_move_struct(type_, contents, self.module_cache.as_ref())?;
                // Convert into `HaneulMoveStruct` which is a mirror of MoveStruct but with additional type supports, (e.g. ascii::String).
                let haneul_move_struct = HaneulMoveStruct::from(move_struct);
                Some(haneul_move_struct.to_json_value().map_err(|e| {
                    HaneulError::ObjectSerializationError {
                        error: e.to_string(),
                    }
                })?)
            }
            _ => None,
        };

        Ok(EventEnvelope::new(
            timestamp_ms,
            Some(digest),
            seq_num,
            event.clone(),
            json_value,
        ))
    }

    pub fn subscribe(&self, filter: EventFilter) -> impl Stream<Item = EventEnvelope> {
        self.event_streamer.subscribe(filter)
    }
}
