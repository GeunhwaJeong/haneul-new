// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use tokio_stream::Stream;
use tracing::{error, instrument, trace};

use crate::streamer::Streamer;
use haneul_json_rpc_types::{
    EffectsWithInput, EventFilter, HaneulTransactionBlockEffects, HaneulTransactionBlockEvents,
    TransactionFilter,
};
use haneul_json_rpc_types::{HaneulEvent, HaneulTransactionBlockEffectsAPI};
use haneul_types::error::HaneulResult;
use haneul_types::messages::TransactionData;

#[cfg(test)]
#[path = "unit_tests/event_handler_tests.rs"]
mod event_handler_tests;

pub const EVENT_DISPATCH_BUFFER_SIZE: usize = 1000;

pub struct SubscriptionHandler {
    event_streamer: Streamer<HaneulEvent, HaneulEvent, EventFilter>,
    transaction_streamer: Streamer<EffectsWithInput, HaneulTransactionBlockEffects, TransactionFilter>,
}

impl Default for SubscriptionHandler {
    fn default() -> Self {
        Self {
            event_streamer: Streamer::spawn(EVENT_DISPATCH_BUFFER_SIZE),
            transaction_streamer: Streamer::spawn(EVENT_DISPATCH_BUFFER_SIZE),
        }
    }
}

impl SubscriptionHandler {
    #[instrument(level = "debug", skip_all, fields(tx_digest = ? effects.transaction_digest()), err)]
    pub async fn process_tx(
        &self,
        input: &TransactionData,
        effects: &HaneulTransactionBlockEffects,
        events: &HaneulTransactionBlockEvents,
    ) -> HaneulResult {
        trace!(
            num_events = events.data.len(),
            tx_digest =? effects.transaction_digest(),
            "Finished writing events to event store"
        );

        if let Err(e) = self
            .transaction_streamer
            .send(EffectsWithInput {
                input: input.clone(),
                effects: effects.clone(),
            })
            .await
        {
            error!(error =? e, "Failed to send transaction to dispatch");
        }

        // serially dispatch event processing to honor events' orders.
        for event in events.data.clone() {
            if let Err(e) = self.event_streamer.send(event).await {
                error!(error =? e, "Failed to send event to dispatch");
            }
        }
        Ok(())
    }

    pub fn subscribe_events(&self, filter: EventFilter) -> impl Stream<Item = HaneulEvent> {
        self.event_streamer.subscribe(filter)
    }

    pub fn subscribe_transactions(
        &self,
        filter: TransactionFilter,
    ) -> impl Stream<Item = HaneulTransactionBlockEffects> {
        self.transaction_streamer.subscribe(filter)
    }
}
