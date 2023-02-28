// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use core::time::Duration;
use move_bytecode_utils::module_cache::SyncModuleCache;
use std::sync::Arc;

use tokio_stream::Stream;
use tracing::{debug, error, instrument, trace, warn};

use haneul_json_rpc_types::HaneulMoveStruct;
use haneul_storage::event_store::{EventStore, EventStoreType};
use haneul_types::base_types::TransactionDigest;
use haneul_types::filter::EventFilter;
use haneul_types::{
    error::{HaneulError, HaneulResult},
    event::{Event, EventEnvelope},
    messages::TransactionEffects,
};

use crate::authority::{AuthorityStore, ResolverWrapper};
use crate::streamer::Streamer;

#[cfg(test)]
#[path = "unit_tests/event_handler_tests.rs"]
mod event_handler_tests;

pub const EVENT_DISPATCH_BUFFER_SIZE: usize = 1000;

pub struct EventHandler {
    event_streamer: Streamer<EventEnvelope, EventFilter>,
    pub(crate) event_store: Arc<EventStoreType>,
}

impl EventHandler {
    pub fn new(event_store: Arc<EventStoreType>) -> Self {
        let streamer = Streamer::spawn(EVENT_DISPATCH_BUFFER_SIZE);
        Self {
            event_streamer: streamer,
            event_store,
        }
    }

    /// Run a regular cleanup task on the store
    pub fn regular_cleanup_task(&self) {
        let store_copy = self.event_store.clone();
        tokio::spawn(async move {
            match store_copy.as_ref() {
                EventStoreType::SqlEventStore(db) => {
                    // Start periodic task to clean up WAL every 30 minutes
                    db.wal_cleanup_thread(Some(Duration::from_secs(30 * 60)))
                        .await;
                }
            }
        });
    }

    #[instrument(level = "debug", skip_all, fields(seq=?seq_num, tx_digest=?effects.transaction_digest), err)]
    pub async fn process_events(
        &self,
        effects: &TransactionEffects,
        timestamp_ms: u64,
        seq_num: u64,
        module_cache: &SyncModuleCache<ResolverWrapper<AuthorityStore>>,
    ) -> HaneulResult {
        let res: Result<Vec<_>, _> = effects
            .events
            .iter()
            .enumerate()
            .map(|(event_num, e)| {
                self.create_envelope(
                    e,
                    effects.transaction_digest,
                    event_num.try_into().unwrap(),
                    seq_num,
                    timestamp_ms,
                    module_cache,
                )
            })
            .collect();
        let envelopes = res?;

        // Ingest all envelopes together at once (for efficiency) into Event Store
        let row_inserted: u64 = self.event_store.add_events(&envelopes).await?;

        if row_inserted != envelopes.len() as u64 {
            warn!(
                num_events = envelopes.len(),
                row_inserted = row_inserted,
                tx_digest =? effects.transaction_digest,
                "Inserted event record is less than expected."
            );
        }

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
        event_num: u64,
        seq_num: u64,
        timestamp_ms: u64,
        module_cache: &SyncModuleCache<ResolverWrapper<AuthorityStore>>,
    ) -> Result<EventEnvelope, HaneulError> {
        let json_value = match event {
            Event::MoveEvent {
                type_, contents, ..
            } => {
                debug!(event =? event, "Process MoveEvent.");
                let move_struct = Event::move_event_to_move_struct(type_, contents, module_cache)?;
                // Convert into `HaneulMoveStruct` which is a mirror of MoveStruct but with additional type supports, (e.g. ascii::String).
                let haneul_move_struct = HaneulMoveStruct::from(move_struct);
                Some(haneul_move_struct.to_json_value())
            }
            _ => None,
        };

        Ok(EventEnvelope::new(
            timestamp_ms,
            digest,
            seq_num,
            event_num,
            event.clone(),
            json_value,
        ))
    }

    pub fn subscribe(&self, filter: EventFilter) -> impl Stream<Item = EventEnvelope> {
        self.event_streamer.subscribe(filter)
    }
}
