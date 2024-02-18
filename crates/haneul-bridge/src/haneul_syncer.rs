// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! The HaneulSyncer module is responsible for synchronizing Events emitted on Haneul blockchain from
//! concerned bridge packages.

use crate::{
    error::BridgeResult,
    retry_with_max_elapsed_time,
    haneul_client::{HaneulClient, HaneulClientInner},
    haneul_transaction_builder::get_bridge_package_id,
};
use haneullabs_metrics::spawn_logged_monitored_task;
use std::{collections::HashMap, sync::Arc};
use haneul_json_rpc_types::HaneulEvent;
use haneul_types::{event::EventID, Identifier};
use tokio::{
    task::JoinHandle,
    time::{self, Duration},
};

// TODO: use the right package id
// const PACKAGE_ID: ObjectID = HANEUL_SYSTEM_PACKAGE_ID;
const HANEUL_EVENTS_CHANNEL_SIZE: usize = 1000;

/// Map from contract address to their start cursor (exclusive)
pub type HaneulTargetModules = HashMap<Identifier, EventID>;

pub struct HaneulSyncer<C> {
    haneul_client: Arc<HaneulClient<C>>,
    // The last transaction that the syncer has fully processed.
    // Syncer will resume post this transaction (i.e. exclusive), when it starts.
    cursors: HaneulTargetModules,
}

impl<C> HaneulSyncer<C>
where
    C: HaneulClientInner + 'static,
{
    pub fn new(haneul_client: Arc<HaneulClient<C>>, cursors: HaneulTargetModules) -> Self {
        Self {
            haneul_client,
            cursors,
        }
    }

    pub async fn run(
        self,
        query_interval: Duration,
    ) -> BridgeResult<(
        Vec<JoinHandle<()>>,
        haneullabs_metrics::metered_channel::Receiver<(Identifier, Vec<HaneulEvent>)>,
    )> {
        let (events_tx, events_rx) = haneullabs_metrics::metered_channel::channel(
            HANEUL_EVENTS_CHANNEL_SIZE,
            &haneullabs_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["haneul_events_queue"]),
        );

        let mut task_handles = vec![];
        for (module, cursor) in self.cursors {
            let events_rx_clone = events_tx.clone();
            let haneul_client_clone = self.haneul_client.clone();
            task_handles.push(spawn_logged_monitored_task!(
                Self::run_event_listening_task(
                    module,
                    cursor,
                    events_rx_clone,
                    haneul_client_clone,
                    query_interval
                )
            ));
        }
        Ok((task_handles, events_rx))
    }

    async fn run_event_listening_task(
        // The module where interested events are defined.
        // Moudle is always of bridge package 0x9.
        module: Identifier,
        mut cursor: EventID,
        events_sender: haneullabs_metrics::metered_channel::Sender<(Identifier, Vec<HaneulEvent>)>,
        haneul_client: Arc<HaneulClient<C>>,
        query_interval: Duration,
    ) {
        tracing::info!(?module, ?cursor, "Starting haneul events listening task");
        let mut interval = time::interval(query_interval);
        interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            let Ok(Ok(events)) = retry_with_max_elapsed_time!(
                haneul_client.query_events_by_module(*get_bridge_package_id(), module.clone(), cursor),
                Duration::from_secs(10)
            ) else {
                tracing::error!("Failed to query events from haneul client after retry");
                continue;
            };

            let len = events.data.len();
            if len != 0 {
                // Note: it's extremely critical to make sure the HaneulEvents we send via this channel
                // are complete per transaction level. Namely, we should never send a partial list
                // of events for a transaction. Otherwise, we may end up missing events.
                // See `haneul_client.query_events_by_module` for how this is implemented.
                events_sender
                    .send((module.clone(), events.data))
                    .await
                    .expect("All Haneul event channel receivers are closed");
                if let Some(next) = events.next_cursor {
                    cursor = next;
                }
                tracing::info!(?module, ?cursor, "Observed {len} new Haneul events");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{haneul_client::HaneulClient, haneul_mock_client::HaneulMockClient};
    use prometheus::Registry;
    use haneul_json_rpc_types::EventPage;
    use haneul_types::{digests::TransactionDigest, event::EventID, Identifier};
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_haneul_syncer_basic() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        haneullabs_metrics::init_metrics(&registry);

        let mock = HaneulMockClient::default();
        let client = Arc::new(HaneulClient::new_for_testing(mock.clone()));
        let module_foo = Identifier::new("Foo").unwrap();
        let module_bar = Identifier::new("Bar").unwrap();
        let empty_events = EventPage::empty();
        let cursor = EventID {
            tx_digest: TransactionDigest::random(),
            event_seq: 0,
        };
        add_event_response(&mock, module_foo.clone(), cursor, empty_events.clone());
        add_event_response(&mock, module_bar.clone(), cursor, empty_events.clone());

        let target_modules = HashMap::from_iter(vec![
            (module_foo.clone(), cursor),
            (module_bar.clone(), cursor),
        ]);
        let interval = Duration::from_millis(200);
        let (_handles, mut events_rx) = HaneulSyncer::new(client, target_modules)
            .run(interval)
            .await
            .unwrap();

        // Initially there are no events
        assert_no_more_events(interval, &mut events_rx).await;

        // Module Foo has new events
        let mut event_1: HaneulEvent = HaneulEvent::random_for_testing();
        let package_id = *get_bridge_package_id();
        event_1.type_.address = package_id.into();
        event_1.type_.module = module_foo.clone();
        let module_foo_events_1: haneul_json_rpc_types::Page<HaneulEvent, EventID> = EventPage {
            data: vec![event_1.clone(), event_1.clone()],
            next_cursor: Some(event_1.id),
            has_next_page: false,
        };
        add_event_response(&mock, module_foo.clone(), event_1.id, empty_events.clone());
        add_event_response(
            &mock,
            module_foo.clone(),
            cursor,
            module_foo_events_1.clone(),
        );

        let (identifier, received_events) = events_rx.recv().await.unwrap();
        assert_eq!(identifier, module_foo);
        assert_eq!(received_events.len(), 2);
        assert_eq!(received_events[0].id, event_1.id);
        assert_eq!(received_events[1].id, event_1.id);
        // No more
        assert_no_more_events(interval, &mut events_rx).await;

        // Module Bar has new events
        let mut event_2: HaneulEvent = HaneulEvent::random_for_testing();
        event_2.type_.address = package_id.into();
        event_2.type_.module = module_bar.clone();
        let module_bar_events_1 = EventPage {
            data: vec![event_2.clone()],
            next_cursor: Some(event_2.id),
            has_next_page: false,
        };
        add_event_response(&mock, module_bar.clone(), event_2.id, empty_events.clone());

        add_event_response(&mock, module_bar.clone(), cursor, module_bar_events_1);

        let (identifier, received_events) = events_rx.recv().await.unwrap();
        assert_eq!(identifier, module_bar);
        assert_eq!(received_events.len(), 1);
        assert_eq!(received_events[0].id, event_2.id);
        // No more
        assert_no_more_events(interval, &mut events_rx).await;

        Ok(())
    }

    async fn assert_no_more_events(
        interval: Duration,
        events_rx: &mut haneullabs_metrics::metered_channel::Receiver<(Identifier, Vec<HaneulEvent>)>,
    ) {
        match timeout(interval * 2, events_rx.recv()).await {
            Err(_e) => (),
            other => panic!("Should have timed out, but got: {:?}", other),
        };
    }

    fn add_event_response(
        mock: &HaneulMockClient,
        module: Identifier,
        cursor: EventID,
        events: EventPage,
    ) {
        mock.add_event_response(
            *get_bridge_package_id(),
            module.clone(),
            cursor,
            events.clone(),
        );
    }
}
