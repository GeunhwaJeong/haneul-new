// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use tokio_stream::Stream;
use tracing::{error, instrument, trace};

use haneul_json_rpc_types::{EventFilter, HaneulTransactionEffects, HaneulTransactionEvents};
use haneul_json_rpc_types::{HaneulEvent, HaneulTransactionEffectsAPI};
use haneul_types::error::HaneulResult;

use crate::streamer::Streamer;

#[cfg(test)]
#[path = "unit_tests/event_handler_tests.rs"]
mod event_handler_tests;

pub const EVENT_DISPATCH_BUFFER_SIZE: usize = 1000;

pub struct EventHandler {
    event_streamer: Streamer<HaneulEvent, EventFilter>,
}

impl Default for EventHandler {
    fn default() -> Self {
        let streamer = Streamer::spawn(EVENT_DISPATCH_BUFFER_SIZE);
        Self {
            event_streamer: streamer,
        }
    }
}

impl EventHandler {
    #[instrument(level = "debug", skip_all, fields(tx_digest=?effects.transaction_digest()), err)]
    pub async fn process_events(
        &self,
        effects: &HaneulTransactionEffects,
        events: &HaneulTransactionEvents,
    ) -> HaneulResult {
        trace!(
            num_events = events.data.len(),
            tx_digest =? effects.transaction_digest(),
            "Finished writing events to event store"
        );

        // serially dispatch event processing to honor events' orders.
        for event in events.data.clone() {
            if let Err(e) = self.event_streamer.send(event).await {
                error!(error =? e, "Failed to send event to dispatch");
            }
        }
        Ok(())
    }

    pub fn subscribe(&self, filter: EventFilter) -> impl Stream<Item = HaneulEvent> {
        self.event_streamer.subscribe(filter)
    }
}
